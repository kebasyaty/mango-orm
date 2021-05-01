//! # Macros
//!
//! `Model` - Macro for converting Structure to mango-orm Model.
//! The model can access the database.
//! The model can create, update, and delete documents in collections.
//!
//! `Form` - Macro for converting Structure to mango-orm Form.
//! The form does not have access to the database.
//! Form are needed where it makes no sense to use a model -
//! To create a search form, to recover a password, to combine models, etc.
//!

use proc_macro::TokenStream;
use quote::quote;
use serde::Serialize;
use syn::Ident;
use syn::{parse_macro_input, Attribute, AttributeArgs, DeriveInput, MetaNameValue, NestedMeta};

// MODEL - MACRO FOR CONVERTING STRUCTURE TO MANGO-ORM MODEL
// #################################################################################################
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Model(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut ast = parse_macro_input!(input as DeriveInput);
    impl_create_model(&args, &mut ast)
}

// Parsing fields and attributes of a structure, creating implementation of methods.
// *************************************************************************************************
fn impl_create_model(args: &Vec<NestedMeta>, ast: &mut DeriveInput) -> TokenStream {
    // Clear the field type from `Option <>`
    let re_clear_field_type = regex::RegexBuilder::new(r"^Option < ([a-z\d\s<>]+) >$")
        .case_insensitive(true)
        .build()
        .unwrap();
    let model_name = &ast.ident;
    if model_name.to_string().len() > 31 {
        panic!(
            "Model: `{}` : Model name - Max size: 31 characters.",
            model_name.to_string()
        )
    }
    let mut trans_meta = Meta {
        model_name: ast.ident.to_string(),
        ..Default::default()
    };
    let mut trans_map_widgets: TransMapWidgets = Default::default();
    // <field_name, (widget_type, value)>
    let mut map_default_values: std::collections::HashMap<String, (String, String)> =
        std::collections::HashMap::new();
    let mut add_trait_custom_valid = quote! {impl AdditionalValidation for #model_name {}};

    // Get Model attributes.
    // *********************************************************************************************
    for nested_meta in args {
        if let NestedMeta::Meta(meta) = nested_meta {
            if let syn::Meta::NameValue(mnv) = meta {
                if mnv.path.is_ident("database") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        trans_meta.database_name = lit_str.value().trim().to_string();
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `database`. Use the `&str` type.",
                            model_name.to_string()
                        )
                    }
                } else if mnv.path.is_ident("db_client_name") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        trans_meta.db_client_name = lit_str.value().trim().to_string();
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `db_client_name`. Use the `&str` type.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("db_query_docs_limit") {
                    if let syn::Lit::Int(lit_int) = &mnv.lit {
                        trans_meta.db_query_docs_limit = lit_int.base10_parse::<u32>().unwrap();
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `db_client_name`. Use the `&str` type.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("is_add_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_add_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `is_add_docs`. Use the `bool` type.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("is_up_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_up_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `is_up_docs`. Use the `bool` type.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("is_del_docs") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        trans_meta.is_del_docs = lit_bool.value;
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `is_del_docs`. Use the `bool` type.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("ignore_fields") {
                    if let syn::Lit::Str(lit_str) = &mnv.lit {
                        let mut value = lit_str.value();
                        value.retain(|chr| !chr.is_whitespace());
                        trans_meta.ignore_fields = value
                            .to_lowercase()
                            .split(',')
                            .map(|item| item.to_string())
                            .collect();
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `ignore_fields`. Use the type `&str` in \
                            the format - <field_name, field_name>.",
                            model_name.to_string(),
                        )
                    }
                } else if mnv.path.is_ident("is_use_add_valid") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        if lit_bool.value {
                            add_trait_custom_valid = quote! {};
                        }
                    } else {
                        panic!(
                            "Model: `{}` : Could not determine value for \
                            parameter `is_use_add_valid`. Use the `bool` type.",
                            model_name.to_string(),
                        )
                    }
                }
            }
        }
    }

    // Get fields of Model.
    // *********************************************************************************************
    if let syn::Data::Struct(ref mut data) = &mut ast.data {
        if let syn::Fields::Named(ref mut fields) = &mut data.fields {
            let fields = &mut fields.named;

            // Add new field `hash`.
            let new_field: syn::FieldsNamed = syn::parse2(quote! {
                {#[serde(default)] #[field_attrs(widget = "hiddenText")] pub hash: Option<String>}
            })
            .unwrap_or_else(|err| panic!("{}", err.to_string()));
            let new_field = new_field.named.first().unwrap().to_owned();
            &fields.push(new_field);

            // Get the number of fields.
            trans_meta.fields_count = fields.len();

            // Loop over fields.
            // -------------------------------------------------------------------------------------
            for field in fields {
                let mut field_name = String::new();
                let mut field_type = String::new();

                // Get field name.
                if let Some(ident) = &field.ident {
                    field_name = ident.to_string();

                    // Check for fields with reserved names - `created_at`, `updated_at`.
                    if field_name == "created_at".to_string() {
                        panic!(
                            "Model: `{}` : The field named `created_at` is reserved.",
                            model_name.to_string()
                        )
                    } else if field_name == "updated_at".to_string() {
                        panic!(
                            "Model: `{}` : The field named `updated_at` is reserved.",
                            model_name.to_string()
                        )
                    }

                    trans_meta.fields_name.push(field_name.clone());
                }
                // Get field type.
                if let syn::Type::Path(ty) = &field.ty {
                    field_type = quote! {#ty}.to_string();
                    let cap = &re_clear_field_type
                        .captures_iter(field_type.as_str())
                        .next();
                    if cap.is_some() {
                        field_type = cap.as_ref().unwrap()[1].to_string();
                    } else {
                        panic!(
                            "Model: `{}` > Field: `{}` : Change field type to `Option < {} >`.",
                            model_name.to_string(),
                            field_name,
                            field_type
                        )
                    }
                    trans_meta
                        .map_field_type
                        .insert(field_name.clone(), field_type.clone());
                }

                // Get the attribute of the field `field_attrs`.
                let attrs: Option<&Attribute> = get_field_attr(&field, "field_attrs");
                let mut widget = Widget {
                    id: get_id(model_name.to_string(), field_name.clone()),
                    name: field_name.clone(),
                    ..Default::default()
                };
                // Allow Validation - Whether the Widget supports the current field type.
                let mut check_field_type = true;

                // Get field attributes.
                if attrs.is_some() {
                    match attrs.unwrap().parse_meta() {
                        Ok(meta) => {
                            if let syn::Meta::List(meta_list) = meta {
                                for nested_meta in meta_list.nested {
                                    if let NestedMeta::Meta(meta) = nested_meta {
                                        if let syn::Meta::NameValue(mnv) = meta {
                                            let attr_name =
                                                &mnv.path.get_ident().unwrap().to_string()[..];
                                            get_param_value(
                                                attr_name,
                                                &mnv,
                                                &mut widget,
                                                model_name.to_string().as_ref(),
                                                field_name.as_ref(),
                                                field_type.as_ref(),
                                                &mut check_field_type,
                                                "Model",
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => panic!("{}", err.to_string()),
                    }
                }

                // Match widget type and field type.
                if check_field_type {
                    let widget_name = widget.widget.clone();
                    let widget_info = get_widget_info(&widget_name).unwrap_or_else(|err| {
                        panic!(
                            "Model: `{}` > Field: `{}` : {}",
                            model_name.to_string(),
                            field_name,
                            err.to_string()
                        )
                    });
                    if widget_info.0 != field_type {
                        panic!(
                            "Model: `{}` > Field: `{}` > Type: {}: \
                            The widget type `{}` is not the same \
                            as the field type.",
                            model_name.to_string(),
                            field_name,
                            field_type,
                            widget_info.0
                        )
                    }
                }
                // Validation the `min` and` max` parameters for date and time.
                if widget.widget == "inputDate".to_string() {
                    let re_valid_date = regex::RegexBuilder::new(
                    r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)$"
                        )
                        .build()
                        .unwrap();
                    if !widget.value.is_empty() {
                        if !re_valid_date.is_match(widget.value.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `default` : \
                                Incorrect date format. Example: \"1970-02-28\"",
                                model_name, field_name
                            )
                        }
                    }
                    if !widget.min.is_empty() {
                        if !re_valid_date.is_match(widget.min.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `min` : \
                                Incorrect date format. Example: \"1970-02-28\"",
                                model_name, field_name
                            )
                        }
                    }
                    if !widget.max.is_empty() {
                        if !re_valid_date.is_match(widget.max.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `max` : \
                                Incorrect date format. Example: \"1970-02-28\"",
                                model_name, field_name
                            )
                        }
                    }
                }
                if widget.widget == "inputDateTime".to_string() {
                    let re_valid_datetime = regex::RegexBuilder::new(
                    r"^(?:[1-9]\d{3}-(?:(?:0[1-9]|1[0-2])-(?:0[1-9]|1\d|2[0-8])|(?:0[13-9]|1[0-2])-(?:29|30)|(?:0[13578]|1[02])-31)|(?:[1-9]\d(?:0[48]|[2468][048]|[13579][26])|(?:[2468][048]|[13579][26])00)-02-29)T(?:[01]\d|2[0-3]):[0-5]\d$"
                        )
                        .build()
                        .unwrap();
                    if !widget.value.is_empty() {
                        if !re_valid_datetime.is_match(widget.value.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `default` : \
                                Incorrect date and time format. Example: \"1970-02-28T00:00\"",
                                model_name, field_name
                            )
                        }
                    }
                    if !widget.min.is_empty() {
                        if !re_valid_datetime.is_match(widget.min.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `min` : \
                                Incorrect date and time format. Example: \"1970-02-28T00:00\"",
                                model_name, field_name
                            )
                        }
                    }
                    if !widget.max.is_empty() {
                        if !re_valid_datetime.is_match(widget.max.as_str()) {
                            panic!(
                                "Model: `{}` > Field: `{}` > Parameter: `max` : \
                                Incorrect date and time format. Example: \"1970-02-28T00:00\"",
                                model_name, field_name
                            )
                        }
                    }
                }
                // Add field name and widget name to the map.
                trans_meta
                    .map_widget_type
                    .insert(field_name.clone(), widget.widget.clone());
                // Add widget to map.
                trans_map_widgets
                    .map_widgets
                    .insert(field_name.clone(), widget);

                // Delete field attributes.
                // ( To avoid conflicts with the compiler )
                field.attrs = Vec::new();
            }
        } else {
            panic!(
                "Model: `{}` : Expected a struct with named fields.",
                model_name.to_string()
            )
        }
    }

    // Post processing.
    // *********************************************************************************************
    // Checking the name of ignored fields.
    for field_name in trans_meta.ignore_fields.iter() {
        if !trans_meta.fields_name.contains(field_name) {
            panic!(
                "Model: `{}` : Model does not have an ignored field named `{}`.",
                model_name.to_string(),
                field_name,
            )
        }
    }
    // Collect `map_default_values` and add to `trans_meta`.
    for field_name in trans_meta.fields_name.iter() {
        let widget = trans_map_widgets
            .map_widgets
            .get_mut(field_name.as_str())
            .unwrap();
        // For dynamic widgets, the default is invalid.
        if widget.widget.contains("Dyn") {
            if !widget.value.is_empty() {
                panic!(
                    "Model: `{}` > Field: `{}` : \
                    For dynamic widgets, it is unacceptable to use default values.",
                    model_name.to_string(),
                    field_name,
                )
            } else if !widget.options.is_empty() {
                panic!(
                    "Model: `{}` > Field: `{}` : \
                    For dynamic widgets, it is unacceptable to use `select` parameter.",
                    model_name.to_string(),
                    field_name,
                )
            } else if trans_meta.ignore_fields.contains(&widget.name) {
                panic!(
                    "Model: `{}` > Field: `{}` : \
                    Dynamic widgets for ignored fields are not allowed.",
                    model_name.to_string(),
                    field_name,
                )
            }
        }
        // File fields must not be ignored.
        match widget.widget.as_str() {
            "inputFile" | "inputImage" if trans_meta.ignore_fields.contains(field_name) => {
                panic!(
                    "Model: `{}` > Field: `{}` : \
                     Ignored fields are incompatible with fields of type `file`.",
                    model_name.to_string(),
                    field_name,
                )
            }
            _ => {}
        }
        // For widgets of the `select` type,
        // the default value must correspond to one of the proposed options.
        if widget.widget.contains("select") {
            if !widget.value.is_empty()
                && widget
                    .options
                    .iter()
                    .filter(|item| item.0 == widget.value)
                    .count()
                    == 0
            {
                panic!(
                    "Model: `{}` > Field: `{}` : \
                    There is no default value in the `options` parameter.",
                    model_name.to_string(),
                    field_name,
                )
            }
        }
        // For widgets with support for u32 numbers, parameter min = 0
        if widget.widget.contains("U32") {
            widget.min = 0_usize.to_string();
        }
        // Add default values in the map.
        map_default_values.insert(
            field_name.clone(),
            (
                widget.widget.clone(),
                if widget.widget != "checkBox" {
                    widget.value.clone()
                } else {
                    widget.checked.to_string()
                },
            ),
        );
    }
    trans_meta.map_default_values = map_default_values;

    // trans_meta to Json-line.
    // ---------------------------------------------------------------------------------------------
    let trans_meta: String = match serde_json::to_string(&trans_meta) {
        Ok(json_string) => json_string,
        Err(err) => panic!("Model: `{}` : {}", model_name.to_string(), err),
    };
    // TransMapWidgets to Json-line.
    let trans_map_widgets: String = match serde_json::to_string(&trans_map_widgets) {
        Ok(json_string) => json_string,
        Err(err) => panic!("Model: `{}` : {}", model_name.to_string(), err.to_string()),
    };

    // Implementation of methods.
    // *********************************************************************************************
    let output = quote! {
        #ast

        // All methods that directly depend on the macro.
        // *****************************************************************************************
        impl ToModel for #model_name {
            // Get model key.
            // (To access data in the cache)
            // -------------------------------------------------------------------------------------
            fn key() -> String {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
                format!(
                    "{}__{}__{}",
                    SERVICE_NAME.trim(),
                    re.replace_all(stringify!(#model_name), "_$upper_chr"),
                    UNIQUE_PROJECT_KEY.trim().to_string()
                )
                .to_lowercase()
            }

            // Get metadata of Model.
            // -------------------------------------------------------------------------------------
            fn meta() -> Result<Meta, Box<dyn std::error::Error>> {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
                let mut meta = serde_json::from_str::<Meta>(&#trans_meta)?;
                let service_name: String = SERVICE_NAME.trim().to_string();
                // Add project name.
                meta.project_name = PROJECT_NAME.trim().to_string();
                // Add unique project key.
                meta.unique_project_key = UNIQUE_PROJECT_KEY.trim().to_string();
                // Add service name.
                meta.service_name = service_name.clone();
                // Add database name.
                if meta.database_name.is_empty() {
                    meta.database_name = format!(
                        "{}__{}__{}",
                        meta.project_name,
                        DATABASE_NAME.trim().to_string(),
                        meta.unique_project_key);
                }
                // Add database client name.
                if meta.db_client_name.is_empty() {
                    meta.db_client_name = DB_CLIENT_NAME.trim().to_string();
                }
                // Add a limit on the number of documents when querying the database.
                if meta.db_query_docs_limit == 0 {
                    meta.db_query_docs_limit = DB_QUERY_DOCS_LIMIT;
                }
                // Add collection name.
                meta.collection_name = format!(
                    "{}_{}",
                    service_name,
                    re.replace_all(&meta.model_name[..], "_$upper_chr")
                )
                .to_lowercase();

                Ok(meta)
            }

            // Get map of widgets for model fields.
            // Hint: <field name, Widget>
            // -------------------------------------------------------------------------------------
            fn widgets() -> Result<std::collections::HashMap<String, Widget>,
                Box<dyn std::error::Error>> {
                Ok(serde_json::from_str::<TransMapWidgets>(&#trans_map_widgets)?.map_widgets)
            }

            // Getter and Setter for field `hash`.
            // -------------------------------------------------------------------------------------
            fn get_hash(&self) -> Option<String> {
                self.hash.clone()
            }
            fn set_hash(&mut self, value: String) {
                self.hash = Some(value);
            }

            // Serialize model to json-line.
            // -------------------------------------------------------------------------------------
            fn self_to_json(&self)
                -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
                Ok(serde_json::to_value(self)?)
            }
        }

        // Caching information about Models for speed up work.
        // *****************************************************************************************
        impl CachingModel for #model_name {}

        // Validating Model fields for save and update.
        // *****************************************************************************************
        impl ValidationModel for #model_name {}

        // A set of methods for custom validation.
        // *****************************************************************************************
        #add_trait_custom_valid

        // Database Query API
        // *****************************************************************************************
        // Common database query methods.
        impl QCommon for #model_name {}
        // Query methods for a Model instance.
        impl QPaladins for #model_name {}

        // Rendering HTML-controls code for Form.
        // *****************************************************************************************
        impl HtmlControls for #model_name {}
    };

    // Hand the output tokens back to the compiler.
    TokenStream::from(output)
}

// FORM - MACRO FOR CONVERTING STRUCTURE TO MANGO-ORM FORM
// #################################################################################################
#[allow(non_snake_case)]
#[proc_macro_attribute]
pub fn Form(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut ast = parse_macro_input!(input as DeriveInput);
    impl_create_form(&args, &mut ast)
}

// Parsing fields and attributes of a structure, creating implementation of methods.
// *************************************************************************************************
fn impl_create_form(args: &Vec<NestedMeta>, ast: &mut DeriveInput) -> TokenStream {
    // Clear the field type from `Option <>`.
    let re_clear_field_type = regex::RegexBuilder::new(r"^Option < ([a-z\d\s<>]+) >$")
        .case_insensitive(true)
        .build()
        .unwrap();
    let form_name: &Ident = &ast.ident;
    let mut trans_map_widgets: TransMapWidgets = Default::default();
    let mut fields_name: Vec<String> = Vec::new();
    let mut add_trait_custom_valid = quote! {impl AdditionalValidation for #form_name {}};

    // Get Form attributes.
    // *********************************************************************************************
    for nested_meta in args {
        if let NestedMeta::Meta(meta) = nested_meta {
            if let syn::Meta::NameValue(mnv) = meta {
                if mnv.path.is_ident("is_use_add_valid") {
                    if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                        if lit_bool.value {
                            add_trait_custom_valid = quote! {};
                        }
                    } else {
                        panic!(
                            "Form: `{}` : Could not determine value for \
                            parameter `is_use_add_valid`. Use the `bool` type.",
                            form_name.to_string(),
                        )
                    }
                }
            }
        }
    }

    // Get Form fields.
    // *********************************************************************************************
    if let syn::Data::Struct(ref mut data) = &mut ast.data {
        if let syn::Fields::Named(ref mut fields) = &mut data.fields {
            let fields = &mut fields.named;

            // Loop over fields.
            // -------------------------------------------------------------------------------------
            for field in fields {
                let mut field_name = String::new();
                let mut field_type = String::new();

                // Get field name.
                if let Some(ident) = &field.ident {
                    field_name = ident.to_string();
                }

                // Add field name to list.
                fields_name.push(field_name.clone());

                // Get field type.
                if let syn::Type::Path(ty) = &field.ty {
                    field_type = quote! {#ty}.to_string();
                    let cap = &re_clear_field_type
                        .captures_iter(field_type.as_str())
                        .next();
                    if cap.is_some() {
                        field_type = cap.as_ref().unwrap()[1].to_string();
                    } else {
                        panic!(
                            "Model: `{}` > Field: `{}` : Change field type to `Option < {} >`.",
                            form_name.to_string(),
                            field_name,
                            field_type
                        )
                    }
                }

                // Get the attribute of the field `field_attrs`.
                let attrs: Option<&Attribute> = get_field_attr(&field, "field_attrs");
                let mut widget = Widget {
                    id: get_id(form_name.to_string(), field_name.clone()),
                    name: field_name.clone(),
                    ..Default::default()
                };
                // Allow Validation - Whether the Widget supports the current field type.
                let mut check_field_type = true;

                // Get field attributes.
                if attrs.is_some() {
                    match attrs.unwrap().parse_meta() {
                        Ok(meta) => {
                            if let syn::Meta::List(meta_list) = meta {
                                for nested_meta in meta_list.nested {
                                    if let NestedMeta::Meta(meta) = nested_meta {
                                        if let syn::Meta::NameValue(mnv) = meta {
                                            let attr_name =
                                                &mnv.path.get_ident().unwrap().to_string()[..];
                                            get_param_value(
                                                attr_name,
                                                &mnv,
                                                &mut widget,
                                                form_name.to_string().as_ref(),
                                                field_name.as_ref(),
                                                field_type.as_ref(),
                                                &mut check_field_type,
                                                "Form",
                                            );
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => panic!("{}", err.to_string()),
                    }
                }
                // Match widget type and field type.
                if check_field_type {
                    let widget_name = widget.widget.clone();
                    let widget_info = get_widget_info(&widget_name).unwrap_or_else(|err| {
                        panic!(
                            "Form: `{}` > Field: `{}` : {}",
                            form_name.to_string(),
                            field_name,
                            err.to_string()
                        )
                    });
                    if widget_info.0 != field_type {
                        panic!(
                            "Model: `{}` > Field: `{}` > Type: {}: \
                            The widget type `{}` is not the same \
                            as the field type.",
                            form_name.to_string(),
                            field_name,
                            field_type,
                            widget_info.0
                        )
                    }
                }
                // Add widget to map.
                trans_map_widgets
                    .map_widgets
                    .insert(field_name.clone(), widget);
                // Delete field attributes.
                // ( To avoid conflicts with the compiler )
                field.attrs = Vec::new();
            }
        } else {
            panic!(
                "Form: `{}` : Expected a struct with named fields.",
                form_name.to_string()
            )
        }
    }

    // Post processing.
    // *********************************************************************************************
    // Checking default values.
    for field_name in fields_name.clone() {
        let widget = trans_map_widgets.map_widgets.get(&field_name[..]).unwrap();
        if widget.widget.contains("Dyn") {
            panic!(
                "Form: `{}` > Field: `{}` : \
                Forms are not supported by dynamic widgets.",
                form_name.to_string(),
                field_name,
            )
        }
    }
    // TransMapWidgets to Json-string
    let trans_map_widgets: String = match serde_json::to_string(&trans_map_widgets) {
        Ok(json_string) => json_string,
        Err(err) => panic!("Form: `{}` : {}", form_name.to_string(), err),
    };
    // fields_name to Json-string
    let fields_name: String = match serde_json::to_string(&fields_name) {
        Ok(json_string) => json_string,
        Err(err) => panic!("Form: `{}` : {}", form_name.to_string(), err),
    };

    // Implementation of methods.
    // *********************************************************************************************
    let output = quote! {
        #ast

        impl ToForm for #form_name {
            // Get form key.
            // (To access data in the cache)
            // -------------------------------------------------------------------------------------
            fn key() -> String {
                let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
                format!(
                    "{}__{}__{}",
                    SERVICE_NAME.trim(),
                    re.replace_all(stringify!(#form_name), "_$upper_chr"),
                    UNIQUE_PROJECT_KEY.trim().to_string()
                )
                .to_lowercase()
            }

            // Get form name
            // -------------------------------------------------------------------------------------
            fn form_name() -> String {
                stringify!(#form_name).to_string()
            }

            // Get fields name list.
            // -------------------------------------------------------------------------------------
            fn fields_name() -> Result<Vec<String>, Box<dyn std::error::Error>> {
                Ok(serde_json::from_str::<Vec<String>>(#fields_name)?)
            }

            // Get map of widgets for model fields.
            // Hint: <field name, Widget>
            // -------------------------------------------------------------------------------------
            fn widgets() -> Result<std::collections::HashMap<String, Widget>,
                Box<dyn std::error::Error>> {
                Ok(serde_json::from_str::<TransMapWidgets>(&#trans_map_widgets)?.map_widgets)
            }

            // Serialize Form to json-line.
            // -------------------------------------------------------------------------------------
            fn self_to_json(&self)
                -> Result<serde_json::value::Value, Box<dyn std::error::Error>> {
                Ok(serde_json::to_value(self)?)
            }
        }

        // Caching information about Models and Forms for speed up work.
        // *****************************************************************************************
        impl CachingForm for #form_name {}

        // Validating Form fields for save and update.
        // *****************************************************************************************
        impl ValidationForm for #form_name {}

        // A set of methods for custom validation.
        // *****************************************************************************************
        #add_trait_custom_valid

        // Rendering HTML-controls code for Form.
        // *****************************************************************************************
        impl HtmlControls for #form_name {}
    };
    // Hand the output tokens back to the compiler.
    TokenStream::from(output)
}

// AUXILIARY STRUCTURES AND FUNCTIONS
// #################################################################################################
// Get field attribute.
// *************************************************************************************************
fn get_field_attr<'a>(field: &'a syn::Field, attr_name: &'a str) -> Option<&'a Attribute> {
    let attr: Option<&Attribute> = field
        .attrs
        .iter()
        .find(|attr| attr.path.is_ident(attr_name));
    attr
}

// Get ID for Widget.
// *************************************************************************************************
fn get_id(model_name: String, field_name: String) -> String {
    let re = regex::Regex::new(r"(?P<upper_chr>[A-Z])").unwrap();
    format!(
        "{}--{}",
        re.replace_all(model_name.as_ref(), "-$upper_chr"),
        field_name.replace('_', "-")
    )[1..]
        .to_lowercase()
}

// Transporting of metadate to implementation of methods.
// *************************************************************************************************
#[derive(Serialize)]
struct Meta {
    pub model_name: String,
    pub project_name: String,
    pub unique_project_key: String,
    pub service_name: String,
    pub database_name: String,
    pub db_client_name: String,
    pub db_query_docs_limit: u32,
    pub collection_name: String,
    pub fields_count: usize,
    pub fields_name: Vec<String>,
    pub is_add_docs: bool,
    pub is_up_docs: bool,
    pub is_del_docs: bool,
    pub map_field_type: std::collections::HashMap<String, String>,
    pub map_widget_type: std::collections::HashMap<String, String>,
    // <field_name, (widget_type, value)>
    pub map_default_values: std::collections::HashMap<String, (String, String)>,
    // List of field names that will not be saved to the database.
    pub ignore_fields: Vec<String>,
}

impl Default for Meta {
    fn default() -> Self {
        Meta {
            model_name: String::new(),
            project_name: String::new(),
            unique_project_key: String::new(),
            service_name: String::new(),
            database_name: String::new(),
            db_client_name: String::new(),
            db_query_docs_limit: 0_u32,
            collection_name: String::new(),
            fields_count: 0_usize,
            fields_name: Vec::new(),
            is_add_docs: true,
            is_up_docs: true,
            is_del_docs: true,
            map_field_type: std::collections::HashMap::new(),
            map_widget_type: std::collections::HashMap::new(),
            map_default_values: std::collections::HashMap::new(),
            // List of field names that will not be saved to the database
            ignore_fields: Vec::new(),
        }
    }
}

// Widget attributes.
// *************************************************************************************************
#[derive(Serialize)]
struct Widget {
    pub id: String, // "model-name--field-name" ( The value is determined automatically )
    pub label: String,
    pub widget: String,
    pub input_type: String, // The value is determined automatically
    pub name: String,
    pub value: String,
    pub accept: String, // Hint: accept="image/jpeg,image/png,image/gif"
    pub placeholder: String,
    pub pattern: String, // Validating a field using a client-side regex
    pub minlength: usize,
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub unique: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub step: String,
    pub min: String,
    pub max: String,
    pub other_attrs: String, // "autofocus tabindex=\"some number\" size=\"some number\" ..."
    pub css_classes: String, // "class-name class-name ..."
    pub options: Vec<(String, String)>, // Hint: <value, Title> - <option value="value1">Title 1</option>
    pub hint: String,
    pub warning: String,    // The value is determined automatically
    pub error: String,      // The value is determined automatically
    pub common_msg: String, // Messages common to the entire Form
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            id: String::new(),
            label: String::new(),
            widget: String::from("inputText"),
            input_type: String::from("text"),
            name: String::new(),
            value: String::new(),
            accept: String::new(),
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            checked: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: String::from("1"),
            min: String::new(),
            max: String::new(),
            other_attrs: String::new(),
            css_classes: String::new(),
            options: Vec::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            common_msg: String::new(),
        }
    }
}

// For transporting of Widgets map to implementation of methods.
// Hint: <field name, Widget>
// *************************************************************************************************
#[derive(Default, Serialize)]
struct TransMapWidgets {
    pub map_widgets: std::collections::HashMap<String, Widget>,
}

// Get widget info.
// *************************************************************************************************
fn get_widget_info<'a>(
    widget_name: &'a str,
) -> Result<(&'a str, &'a str), Box<dyn std::error::Error>> {
    let info: (&'a str, &'a str) = match widget_name {
        "checkBox" => ("bool", "checkbox"),
        "inputColor" => ("String", "color"),
        "inputDate" => ("String", "date"),
        "inputDateTime" => ("String", "datetime"),
        "inputEmail" => ("String", "email"),
        "inputFile" => ("String", "file"),
        "inputImage" => ("String", "file"),
        "numberI32" => ("i32", "number"),
        "numberU32" => ("u32", "number"),
        "numberI64" => ("i64", "number"),
        "numberF64" => ("f64", "number"),
        "inputPassword" => ("String", "password"),
        "radioText" => ("String", "radio"),
        "radioI32" => ("i32", "radio"),
        "radioU32" => ("u32", "radio"),
        "radioI64" => ("i64", "radio"),
        "radioF64" => ("f64", "radio"),
        "rangeI32" => ("i32", "range"),
        "rangeU32" => ("u32", "range"),
        "rangeI64" => ("i64", "range"),
        "rangeF64" => ("f64", "range"),
        "inputPhone" => ("String", "tel"),
        "inputText" => ("String", "text"),
        "inputUrl" => ("String", "url"),
        "inputIP" => ("String", "text"),
        "inputIPv4" => ("String", "text"),
        "inputIPv6" => ("String", "text"),
        "textArea" => ("String", "textarea"),
        "selectText" => ("String", "select"),
        "selectTextDyn" => ("String", "select"),
        "selectTextMult" => ("Vec < String >", "select"),
        "selectTextMultDyn" => ("Vec < String >", "select"),
        "selectI32" => ("i32", "select"),
        "selectI32Dyn" => ("i32", "select"),
        "selectI32Mult" => ("Vec < i32 >", "select"),
        "selectI32MultDyn" => ("Vec < i32 >", "select"),
        "selectU32" => ("u32", "select"),
        "selectU32Dyn" => ("u32", "select"),
        "selectU32Mult" => ("Vec < u32 >", "select"),
        "selectU32MultDyn" => ("Vec < u32 >", "select"),
        "selectI64" => ("i64", "select"),
        "selectI64Dyn" => ("i64", "select"),
        "selectI64Mult" => ("Vec < i64 >", "select"),
        "selectI64MultDyn" => ("Vec < i64 >", "select"),
        "selectF64" => ("f64", "select"),
        "selectF64Dyn" => ("f64", "select"),
        "selectF64Mult" => ("Vec < f64 >", "select"),
        "selectF64MultDyn" => ("Vec < f64 >", "select"),
        "hiddenText" => ("String", "hidden"),
        "hiddenI32" => ("i32", "hidden"),
        "hiddenU32" => ("u32", "hidden"),
        "hiddenI64" => ("i64", "hidden"),
        "hiddenF64" => ("f64", "hidden"),
        _ => Err("Invalid widget type.")?,
    };
    Ok(info)
}

// Get parameter value from model field attribute.
// *************************************************************************************************
fn get_param_value<'a>(
    attr_name: &'a str,
    mnv: &MetaNameValue,
    widget: &mut Widget,
    model_name: &'a str,
    field_name: &'a str,
    field_type: &'a str,
    check_field_type: &mut bool,
    model_or_form: &'a str,
) {
    match attr_name {
        "label" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.label = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `label`. \
                    Example: \"Some text\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "accept" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.accept = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `accept`. \
                    Example: \"image/jpeg,image/png\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "widget" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                let widget_name = lit_str.value();
                let widget_info = get_widget_info(widget_name.as_ref()).unwrap_or_else(|err| {
                    panic!(
                        "{}: `{}` > Field: `{}` : {}",
                        model_or_form,
                        model_name,
                        field_name,
                        err.to_string()
                    )
                });
                if widget_info.0 != field_type {
                    panic!(
                        "{}: `{}` > Field: `{}` : \
                        The widget type is not the same as the field type.",
                        model_or_form, model_name, field_name,
                    )
                }
                widget.widget = widget_name.clone();
                widget.input_type = widget_info.1.to_string();
                *check_field_type = false;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `widget`. \
                    Example: \"inputEmail\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "value" => match field_type {
            "i32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.value = lit_int.base10_parse::<i32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `value`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "u32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.value = lit_int.base10_parse::<u32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `value`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "i64" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.value = lit_int.base10_parse::<i64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `value`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "f64" => {
                if let syn::Lit::Float(lit_float) = &mnv.lit {
                    widget.value = lit_float.base10_parse::<f64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `value`. \
                        Example: 10.2",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "String" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    widget.value = lit_str.value().trim().to_string()
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `value`. \
                        Example: \"Some text\"",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            _ => panic!(
                "{}: `{}` > Field: `{}` > Type: {} : \
                Unsupported field type for `default` parameter.",
                model_or_form,
                model_name.to_string(),
                field_name,
                field_type
            ),
        },
        "placeholder" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.placeholder = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `placeholder`. \
                    Example: \"Some text\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "pattern" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.pattern = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `pattern`. \
                    Example: \"some regular expression\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "minlength" => {
            if let syn::Lit::Int(lit_int) = &mnv.lit {
                widget.minlength = lit_int.base10_parse::<usize>().unwrap();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `minlength`. \
                    Example: 10",
                    model_or_form, model_name, field_name
                )
            }
        }
        "maxlength" => {
            if let syn::Lit::Int(lit_int) = &mnv.lit {
                widget.maxlength = lit_int.base10_parse::<usize>().unwrap();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `maxlength`. \
                    Example: 10",
                    model_or_form, model_name, field_name
                )
            }
        }
        "required" => {
            if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                widget.required = lit_bool.value;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `required`. \
                    Example: true. Default = false.",
                    model_or_form, model_name, field_name
                )
            }
        }
        "checked" => {
            if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                widget.checked = lit_bool.value;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `checked`. \
                    Example: true. Default = false.",
                    model_or_form, model_name, field_name
                )
            }
        }
        "unique" => {
            if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                widget.unique = lit_bool.value;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `unique`. \
                    Example: true. Default = false.",
                    model_or_form, model_name, field_name
                )
            }
        }
        "disabled" => {
            if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                widget.disabled = lit_bool.value;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `disabled`. \
                    Example: true. Default = false.",
                    model_or_form, model_name, field_name
                )
            }
        }
        "readonly" => {
            if let syn::Lit::Bool(lit_bool) = &mnv.lit {
                widget.readonly = lit_bool.value;
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `readonly`. \
                    Example: true. Default = false.",
                    model_or_form, model_name, field_name
                )
            }
        }
        "step" => match field_type {
            "i32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.step = lit_int.base10_parse::<i32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `step`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "u32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.step = lit_int.base10_parse::<u32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `step`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "i64" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.step = lit_int.base10_parse::<i64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `step`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "f64" => {
                if let syn::Lit::Float(lit_float) = &mnv.lit {
                    widget.step = lit_float.base10_parse::<f64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `step`. \
                        Example: 10.2",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "String" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    widget.step = lit_str.value().trim().to_string()
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `step`.
                        Example: not supported.",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            _ => panic!(
                "{}: `{}` > Field: `{}` > Type: {} : \
                Unsupported field type for `step` parameter.",
                model_or_form, model_name, field_name, field_type
            ),
        },
        "min" => match field_type {
            "i32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.min = lit_int.base10_parse::<i32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `min`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "u32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.min = lit_int.base10_parse::<u32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `min`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "i64" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.min = lit_int.base10_parse::<i64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `min`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "f64" => {
                if let syn::Lit::Float(lit_float) = &mnv.lit {
                    widget.min = lit_float.base10_parse::<f64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `min`. \
                        Example: 10.2",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "String" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    widget.min = lit_str.value().trim().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `min`. \
                        Example: \"1970-02-28\" or \"1970-02-28T00:00\"",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            _ => panic!(
                "{}: `{}` > Field: `{}` > Type: {} : \
                Unsupported field type for `min` parameter.",
                model_or_form, model_name, field_name, field_type
            ),
        },
        "max" => match field_type {
            "i32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.max = lit_int.base10_parse::<i32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `max`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "u32" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.max = lit_int.base10_parse::<u32>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `max`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "i64" => {
                if let syn::Lit::Int(lit_int) = &mnv.lit {
                    widget.max = lit_int.base10_parse::<i64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `max`. \
                        Example: 10",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "f64" => {
                if let syn::Lit::Float(lit_float) = &mnv.lit {
                    widget.max = lit_float.base10_parse::<f64>().unwrap().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `max`. \
                        Example: 10.2",
                        model_or_form, model_name, field_name, field_type,
                    )
                }
            }
            "String" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    widget.max = lit_str.value().trim().to_string();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `max`. \
                        Example: \"1970-02-28\" or \"1970-02-28T00:00\"",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            _ => panic!(
                "{}: `{}` > Field: `{}` > Type: {} : \
                Unsupported field type for `max` parameter.",
                model_or_form, model_name, field_name, field_type
            ),
        },
        "options" => match field_type {
            "i32" | "Vec < i32 >" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    let json = lit_str.value().replace('_', "");
                    let raw_options: Vec<(i32, String)> =
                        serde_json::from_str(json.as_str()).unwrap();
                    widget.options = raw_options
                        .iter()
                        .map(|item| (item.0.to_string(), item.1.clone()))
                        .collect();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `options`. \
                        Example: [[10, \"Title 1\"], [20, \"Title 2\"], ...]",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "u32" | "Vec < u32 >" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    let json = lit_str.value().replace('_', "");
                    let raw_options: Vec<(u32, String)> =
                        serde_json::from_str(json.as_str()).unwrap();
                    widget.options = raw_options
                        .iter()
                        .map(|item| (item.0.to_string(), item.1.clone()))
                        .collect();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `options`. \
                        Example: [[10, \"Title 1\"], [20, \"Title 2\"], ...]",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "i64" | "Vec < i64 >" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    let json = lit_str.value().replace('_', "");
                    let raw_options: Vec<(i64, String)> =
                        serde_json::from_str(json.as_str()).unwrap();
                    widget.options = raw_options
                        .iter()
                        .map(|item| (item.0.to_string(), item.1.clone()))
                        .collect();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `options`. \
                        Example: [[10, \"Title 1\"], [20, \"Title 2\"], ...]",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "f64" | "Vec < f64 >" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    let json = lit_str.value().replace('_', "");
                    let raw_options: Vec<(f64, String)> =
                        serde_json::from_str(json.as_str()).unwrap();
                    widget.options = raw_options
                        .iter()
                        .map(|item| (item.0.to_string(), item.1.clone()))
                        .collect();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `options`. \
                        Example: [[10.1, \"Title 1\"], [20.2, \"Title 2\"], ...]",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            "String" | "Vec < String >" => {
                if let syn::Lit::Str(lit_str) = &mnv.lit {
                    let json = lit_str.value();
                    widget.options = serde_json::from_str(json.as_str()).unwrap();
                } else {
                    panic!(
                        "{}: `{}` > Field: `{}` > Type: {} : \
                        Could not determine value for parameter `options`. \
                        Example: [[\"value\", \"Title 1\"], [value, \"Title 2\"], ...]",
                        model_or_form, model_name, field_name, field_type
                    )
                }
            }
            _ => panic!(
                "{}: `{}` > Field: `{}` > Type: {} : \
                Unsupported field type for `options` parameter.",
                model_or_form, model_name, field_name, field_type
            ),
        },
        "other_attrs" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.other_attrs = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `other_attrs`. \
                    Example: \"autofocus multiple size=\\\"some number\\\"\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "css_classes" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.css_classes = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `css_classes`. \
                    Example: \"class_name, class_name\"",
                    model_or_form, model_name, field_name
                )
            }
        }
        "hint" => {
            if let syn::Lit::Str(lit_str) = &mnv.lit {
                widget.hint = lit_str.value().trim().to_string();
            } else {
                panic!(
                    "{}: `{}` > Field: `{}` : \
                    Could not determine value for parameter `hint`. \
                    Example: \"Some text\".",
                    model_or_form, model_name, field_name
                )
            }
        }
        "id" => panic!(
            "{}: `{}` > Field: `{}` : The `id` parameter is determined automatically.",
            model_or_form, model_name, field_name
        ),
        "name" => panic!(
            "{}: `{}` > Field: `{}` : The `name` parameter is determined automatically.",
            model_or_form, model_name, field_name
        ),
        "input_type" => panic!(
            "{}: `{}` > Field: `{}` : The `input_type` parameter is determined automatically.",
            model_or_form, model_name, field_name
        ),
        "warning" => panic!(
            "{}: `{}` > Field: `{}` : The `warning` parameter is determined automatically.",
            model_or_form, model_name, field_name
        ),
        "error" => panic!(
            "{}: `{}` > Field: `{}` : The `error` parameter is determined automatically.",
            model_or_form, model_name, field_name
        ),
        _ => panic!(
            "{}: `{}` > Field: `{}` : Undefined field attribute `{}`.",
            model_or_form,
            model_name.to_string(),
            field_name,
            attr_name
        ),
    };
}
