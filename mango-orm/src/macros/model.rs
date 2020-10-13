//! # Create Model
//!
//!  `model` - Macro for converting Structure to Model.

// MACRO
// #################################################################################################
/// Macro for converting Structure to Model
#[macro_export]
macro_rules! model {
    ($service:expr, $database:expr,
        // $(#[$sattr:meta])*
        struct $sname:ident { $($fname:ident : $ftype:ty),* }
        $(#[$iattr:meta])* $($impls:item)+) => {

        #[derive(Serialize, Deserialize, Default, Clone, Debug)]
        pub struct $sname {
            $(pub $fname : $ftype),*
        }

        $(#[$iattr])*
        $($impls)+

        impl $sname {
            // Info Model
            // *************************************************************************************
            // Get model name
            pub fn model_name() -> Result<&'static str, Box<dyn Error>> {
                Ok(stringify!($sname))
            }

            // Get array of field names
            pub fn field_names() -> Result<&'static [&'static str], Box<dyn Error>> {
                Ok(&[$(stringify!($fname)),*])
            }

            // // Get a map with field types
            pub fn field_types() -> Result<HashMap<&'static str, &'static str>, Box<dyn Error>> {
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                Ok(FIELD_NAMES.iter().map(|item| item.to_owned())
                .zip([$(stringify!($ftype)),*].iter().map(|item| item.to_owned())).collect())
            }

            // Metadata (database name, collection name, etc)
            pub fn meta() -> Result<Meta, Box<dyn Error>> {
                if $service.len() > 0 && $database.len() > 0 {
                    Ok(Meta {
                        database: $database.to_lowercase(),
                        collection: format!("{}__{}",
                            $service.to_lowercase(),
                            stringify!($sname).to_lowercase()
                        )
                    })
                } else {
                    Err(format!("Model: `{}` -> Method: `field_types()` : Service name (App name) and database name should not be empty.",
                        stringify!($sname)))?
                }
            }

            // Form - Widgets, attributes (HashMap, Json), Html
            // *************************************************************************************
            // Get full map of Widgets (with widget for id field)
            pub fn widgets_full_map() -> Result<HashMap<&'static str, Widget>, Box<dyn Error>> {
                let mut map: HashMap<&'static str, Widget> = Self::widgets()?;
                if map.get("hash").is_none() {
                    map.insert(
                        "hash",
                        Widget {
                            value: FieldType::Hash,
                            hidden: true,
                            ..Default::default()
                        }
                    );
                }
                Ok(map)
            }

            // Add (if required) default form data to cache
            pub async fn form_cache() -> Result<(async_mutex::MutexGuard<'static, HashMap<&'static str,
                mango_orm::models::FormCache>>, &'static str), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let key: &'static str = Box::leak(format!("{}_{}",
                    $service.to_lowercase(),
                    stringify!($sname).to_lowercase()
                ).into_boxed_str());
                let mut store: async_mutex::MutexGuard<'_, HashMap<&'static str,
                    mango_orm::models::FormCache>> = FORM_CACHE.lock().await;
                let mut cache: Option<&FormCache> = store.get(key);
                if cache.is_none() {
                    // Add a map of pure attributes of Form for page templates
                    let widgets: HashMap<&str, Widget> = Self::widgets_full_map()?;
                    let mut clean_attrs: HashMap<String, Transport> = HashMap::new();
                    let mut widget_map: HashMap<String, &str> = HashMap::new();
                    for (field, widget) in &widgets {
                        clean_attrs.insert(field.to_string(), widget.clean_attrs(field)?);
                        widget_map.insert(field.to_string(), widget.value.get_enum_type());
                    }
                    // Add default data
                    let form_cache = FormCache{
                        attrs_map: clean_attrs,
                        widget_map: widget_map,
                        ..Default::default()
                    };
                    // Save default data to cache
                    store.insert(key, form_cache);
                }
                Ok((store, key))
            }

            // Get a map of pure attributes of Form for page templates
            pub async fn form_map() -> Result<HashMap<String, Transport>, Box<dyn Error>> {
                let (store, key) = Self::form_cache().await?;
                let cache: Option<&FormCache> = store.get(key);
                if cache.is_some() {
                    let clean_attrs: HashMap<String, Transport> = cache.unwrap().attrs_map.clone();
                    Ok(clean_attrs)
                } else {
                    Err(format!("Model: `{}` -> Method: `form_map()` : Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Get Form attributes in Json format for page templates
            pub async fn form_json() -> Result<String, Box<dyn Error>> {
                let (mut store, key) = Self::form_cache().await?;
                let cache: Option<&FormCache> = store.get(key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    if cache.attrs_json.len() == 0 {
                        // Create Json-string
                        let mut form_cache: FormCache = cache.clone();
                        let attrs: HashMap<String, Transport> = form_cache.attrs_map.clone();
                        let mut json_text = String::new();
                        for (field, trans) in attrs {
                            let tmp = serde_json::to_string(&trans).unwrap();
                            if json_text.len() > 0 {
                                json_text = format!("{},\"{}\":{}", json_text, field, tmp);
                            } else {
                                json_text = format!("\"{}\":{}", field, tmp);
                            }
                        }
                        // Update data
                        form_cache.attrs_json = format!("{{{}}}", json_text);
                        // Save data to cache
                        store.insert(key, form_cache.clone());
                        // Return result
                        return Ok(form_cache.attrs_json);
                    }
                    Ok(cache.attrs_json.clone())
                } else {
                    Err(format!("Model: `{}` -> Method: `form_json()` : Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Get Html Form of Model for page templates
            pub async fn form_html() ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let (mut store, key) = Self::form_cache().await?;
                let model_name: &str = &stringify!($sname).to_lowercase();
                let mut build_controls = false;
                let mut attrs: HashMap<String, Transport> = HashMap::new();
                //
                let cache: Option<&FormCache> = store.get(key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    let is_cached: bool = cache.form_html.len() == 0;
                    if is_cached {
                        build_controls = true;
                        attrs = cache.attrs_map.clone();
                    }
                    let controls = Self::html(
                        attrs,
                        model_name,
                        build_controls
                    )?;
                    if is_cached {
                         // Clone cache
                         let mut form_cache: FormCache = cache.clone();
                        // Update cache
                        form_cache.form_html = controls.clone();
                        // Save to cache
                        store.insert(key, form_cache.clone());
                        // Return result
                        return Ok(controls);
                    }
                    Ok(cache.form_html.clone())
                } else {
                    Err(format!("Model: `{}` -> Method: `form_html()` : Did not receive data from cache.",
                        stringify!($sname)))?
                }
            }

            // Validation of database queries
            // *************************************************************************************
            // Validation of `maxlength`
            fn check_maxlength(maxlength: usize, data: &str ) -> Result<(), Box<dyn Error>>  {
                if maxlength > 0 && data.encode_utf16().count() > maxlength {
                    Err(format!("Exceeds limit, maxlength={}.", maxlength))?
                }
                Ok(())
            }

            // Validation of `unique`
            async fn check_unique(
                is_update: bool, is_unique: bool, field: &String, data: &str,
                coll: &Collection) -> Result<(), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                if !is_update && is_unique {
                    let filter: Document = doc!{ field.to_string() : data };
                    let count: i64 = coll.count_documents(filter, None).await?;
                    if count > 0 {
                        Err("Is not unique.")?
                    }
                }
                Ok(())
            }

            // Accumulation of errors
            fn accumula_err(attrs: &Transport, err: &String) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut tmp = attrs.error.clone();
                tmp = if tmp.len() > 0_usize { format!("{}<br>", tmp) } else { String::new() };
                Ok(format!("{}{}", tmp, err))
            }

            // Additional validation for some fields (email, password, url, ip, etc...)
            fn additional_validation(field_type: &str, data: &str) ->
                Result<(), Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                match field_type {
                    "InputEmail" => {
                        if !validate_email(data) {
                            Err("Invalid email address.")?
                        }
                    }
                    "InputColor" => {
                        let re = RegexBuilder::new(
                            r"^(?:#|0x)(?:[a-f0-9]{3}|[a-f0-9]{6})\b|(?:rgb|hsl)a?\([^\)]*\)$")
                            .case_insensitive(true).build()?;
                        if !re.is_match(data) {
                            Err("Invalid Color code.")?
                        }
                    }
                    "InputUrl" => {
                        if !validate_url(data) {
                            Err("Invalid Url.")?
                        }
                    }
                    "InputIP" => {
                        if !validate_ip(data) {
                            Err("Invalid IP address.")?
                        }
                    }
                    "InputIPv4" => {
                        if !validate_ip_v4(data) {
                            Err("Invalid IPv4 address.")?
                        }
                    }
                    "InputIPv6" => {
                        if !validate_ip_v6(data) {
                            Err("Invalid IPv6 address.")?
                        }
                    }
                    "InputPassword" => {
                        let re = RegexBuilder::new(
                            r"^[a-z0-9@#$%^&+=*!~)(]{8,}$")
                            .case_insensitive(true).build()?;
                        if !re.is_match(data) {
                            Err("Allowed characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br> \
                                 Minimum size 8 characters")?
                        }
                    }
                    _ => return Ok(()),
                }
                Ok(())
            }

            // Generate password hash and add to result document
            pub fn create_password_hash(field_data: &str) -> Result<String, Box<dyn Error>> {
                    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                            abcdefghijklmnopqrstuvwxyz\
                                            0123456789@#$%^&+=*!~)(";
                    const SALT_LEN: usize = 12;
                    let mut rng = rand::thread_rng();
                    let password: &[u8] = field_data.as_bytes();
                    let salt: String = (0..SALT_LEN)
                        .map(|_| {
                            let idx = rng.gen_range(0, CHARSET.len());
                            CHARSET[idx] as char
                        })
                        .collect();
                    let salt: &[u8] = salt.as_bytes();
                    let config = Config::default();
                    let hash: String = argon2::hash_encoded(password, salt, &config)?;
                    Ok(hash)
            }

            // Validation of Form
            pub async fn check(&self, client: &Client, output_format: OutputType) ->
                Result<OutputData, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                static MODEL_NAME: &'static str = stringify!($sname);
                let (mut store, key) = Self::form_cache().await?;
                let meta: Meta = Self::meta()?;
                let mut stop_err = false;
                let is_update: bool = self.hash.len() > 0;
                let mut attrs_map: HashMap<String, Transport> = HashMap::new();
                let ignore_fields: Vec<&'static str> = Self::ignore_fields()?;
                let coll: Collection = client.database(&meta.database).collection(&meta.collection);
                // Get data from model
                let mut doc_tmp: Document = to_document(self).unwrap();
                // Get data for model from database (if available)
                let mut doc_update: Document = if is_update {
                    let object_id: ObjectId = ObjectId::with_string(&self.hash)
                        .unwrap_or_else(|err| { panic!("{:?}", err) });
                    let filter: Document = doc!{"_id": object_id};
                    coll.find_one(filter, None).await?.unwrap()
                } else {
                    doc! {}
                };
                // Document for the final result
                let mut doc_res: Document = doc! {};

                // Validation of field by attributes (maxlength, unique, min, max, etc...)
                // ---------------------------------------------------------------------------------
                let cache: Option<&FormCache> = store.get(key);
                if cache.is_some() {
                    let cache: &FormCache = cache.unwrap();
                    static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                    attrs_map = cache.attrs_map.clone();
                    let widget_map: HashMap<String, &'static str> = cache.widget_map.clone();
                    // Apply custom check
                    {
                        let error_map: HashMap<&'static str, &'static str> = self.custom_check()?;
                        if !error_map.is_empty() { stop_err = true; }
                        for (field_name, err_msg) in error_map {
                            let attrs: &mut Transport = attrs_map.get_mut(field_name).unwrap();
                            attrs.error = Self::accumula_err(&attrs, &err_msg.to_string()).unwrap();
                        }
                    }
                    // Loop over fields
                    for field in FIELD_NAMES {
                        // Filter out specific fields
                        if field == &"hash" || ignore_fields.contains(field) {
                            continue;
                        }
                        // Get field value for validation
                        let value: Option<&Bson> = doc_tmp.get(field);
                        //
                        if value.is_some() {
                            let value: &Bson = value.unwrap();
                            let field: &String = &field.to_string();
                            let field_type: &str = widget_map.get(field).unwrap();
                            // Field validation
                            match field_type {
                                // Validation of text type fields
                                // -----------------------------------------------------------------
                                "InputText" | "InputEmail" | "TextArea" | "InputColor" |
                                    "InputUrl" | "InputIP" | "InputIPv4" | "InputIPv6" |
                                    "InputPassword" => {
                                    let field_data: &str = value.as_str().unwrap();
                                    let attrs: &mut Transport = attrs_map.get_mut(field).unwrap();
                                    // Validation for a required field
                                    if attrs.required && field_data.len() == 0 {
                                        stop_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &"Required field.".to_owned()).unwrap();
                                        attrs.value = field_data.to_string();
                                        continue;
                                    }
                                    // Add data from the field to the final document and in attribute map.
                                    if is_update {
                                        let value_update: Option<&Bson> = doc_update.get(field);
                                        if value_update.is_some() {
                                            let value_update: &Bson = value_update.unwrap();
                                            let field_data_update: &str = value_update.as_str().unwrap();
                                            if field_data.len() > 0 {
                                                attrs.value = field_data.to_string();
                                                doc_res.insert(field.to_string(), Bson::String(field_data.to_string()));
                                            } else if !attrs.required {
                                                attrs.value = field_data_update.to_string();
                                                doc_res.insert(field.to_string(), Bson::String(field_data_update.to_string()));
                                                continue;
                                            }
                                        } else {
                                            Err(format!("Model: `{}` -> Field: `{}` -> Method: `save()` : This field is missing from the database.",
                                            MODEL_NAME, field))?
                                        }
                                    } else {
                                        attrs.value = field_data.to_string();
                                        doc_res.insert(field.to_string(), Bson::String(field_data.to_string()));
                                    }
                                    // Checking `maxlength`, `min length`, `max length`
                                    Self::check_maxlength(attrs.maxlength, field_data).unwrap_or_else(|err| {
                                        stop_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &err.to_string()).unwrap();
                                    });
                                    // Validation of range (`min` <> `max`)
                                    // (Hint: The `validate_length()` method did not provide the desired result)
                                    {
                                        let min: f64 = attrs.min.parse().unwrap();
                                        let max: f64 = attrs.max.parse().unwrap();
                                        let len: f64 = field_data.encode_utf16().count() as f64;
                                        if (min > 0_f64 || max > 0_f64) &&
                                            !validate_range(Validator::Range{min: Some(min), max: Some(max)}, len) {
                                            stop_err = true;
                                            let msg = format!("Length {}, is out of range (min={} <> max={}).", len, min, max);
                                            attrs.error = Self::accumula_err(&attrs, &msg).unwrap();
                                        }
                                    }
                                    // Validation of `unique`
                                    Self::check_unique(is_update, attrs.unique, field, field_data, &coll).await.unwrap_or_else(|err| {
                                        stop_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &err.to_string()).unwrap();
                                    });

                                    // Additional validation (email, password, url, ip, etc...)
                                    // -------------------------------------------------------------
                                    Self::additional_validation(field_type, field_data).unwrap_or_else(|err| {
                                        stop_err = true;
                                        attrs.error =
                                            Self::accumula_err(&attrs, &err.to_string()).unwrap();
                                    });

                                    // Additional actions
                                    // -------------------------------------------------------------
                                    if !stop_err {
                                        if field_data.len() > 0 && field_type == "InputPassword" {
                                            // Generate password hash and add to result document
                                            let hash: String = Self::create_password_hash(field_data)?;
                                            doc_res.insert(field.to_string(), Bson::String(hash));
                                        }
                                    }
                                }
                                _ => {
                                    Err(format!("Model: `{}` -> Field: `{}` -> Method: `save()` : Unsupported data type.",
                                        MODEL_NAME, field))?
                                }
                            }
                        } else {
                            Err(format!("Model: `{}` -> Field: `{}` -> Method: `save()` : This field is missing.",
                                MODEL_NAME, field))?
                        }
                    }
                } else {
                    Err(format!("Model: `{}` -> Method: `save()` : Did not receive data from cache.",
                        MODEL_NAME))?
                }

                // Post processing
                // ---------------------------------------------------------------------------------
                let result: OutputData = match output_format {
                    // Get Hash-line
                    OutputType::Hash => {
                        let data: String = Self::to_hash(&attrs_map)?;
                        OutputData::Hash((data, !stop_err, doc_res))
                    }
                    // Get Attribute Map
                    OutputType::Map => OutputData::Map((attrs_map, !stop_err, doc_res)),
                    // Get Json-line
                    OutputType::Json => {
                        let data: String = Self::to_json(&attrs_map)?;
                        OutputData::Json((data, !stop_err, doc_res))
                    }
                    // Get Html-line
                    OutputType::Html => {
                        let data: String = Self::to_html(attrs_map)?;
                        OutputData::Html((data, !stop_err, doc_res))
                    }
                };

                Ok(result)
            }

            // Post processing database queries
            // *************************************************************************************
            // Get Hash-line
            pub fn to_hash(attrs_map: &HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut errors = String::new();
                for (field, trans) in attrs_map {
                    let tmp = if errors.len() > 0_usize {
                        format!("{} ; ", errors)
                    } else {
                        String::new()
                    };
                    if trans.error.len() > 0_usize {
                        errors = format!("{}Field: `{}` - {}", tmp, field, trans.error);
                    }
                }
                if errors.len() == 0 {
                    Ok(attrs_map
                        .get(&"hash".to_string())
                        .unwrap()
                        .value
                        .clone())
                } else {
                    Err(errors.replace("<br>", " | "))?
                }
            }

            // Get Json-line
            pub fn to_json(attrs_map: &HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let mut json_text = String::new();
                for (field, trans) in attrs_map {
                    let tmp = serde_json::to_string(&trans).unwrap();
                    if json_text.len() > 0 {
                        json_text = format!("{},\"{}\":{}", json_text, field, tmp);
                    } else {
                        json_text = format!("\"{}\":{}", field, tmp);
                    }
                }
                Ok(format!("{{{}}}", json_text))
            }

            // Get Html-line
            pub fn to_html(attrs_map: HashMap<String, Transport>) ->
                Result<String, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let controls = Self::html(
                    attrs_map,
                    &stringify!($sname).to_lowercase(),
                    true
                )?;
                Ok(controls)
            }

            // Database Query API
            // *************************************************************************************
            // Save to database as a new document or
            // update an existing document.
            // (Returns the hash-line of the identifier)
            pub async fn save(& mut self, client: &Client, output_format: OutputType) ->
                Result<OutputData, Box<dyn Error>> {
                // ---------------------------------------------------------------------------------
                let verified_data: OutputData = self.check(client, OutputType::Map).await?;
                let mut attrs_map: HashMap<String, Transport> = verified_data.map();
                let meta: Meta = Self::meta()?;
                let is_update: bool = self.hash.len() > 0;
                let coll: Collection = client.database(&meta.database).collection(&meta.collection);

                // Save to database
                // ---------------------------------------------------------------------------------
                if verified_data.bool() {
                    if !is_update {
                        let result: results::InsertOneResult = coll.insert_one(verified_data.doc(), None).await?;
                        self.hash = result.inserted_id.as_object_id().unwrap().to_hex();
                    } else {
                        let object_id: ObjectId = ObjectId::with_string(&self.hash)
                            .unwrap_or_else(|err| { panic!("{}", err.to_string()) });
                        let query: Document = doc!{"_id": object_id};
                        coll.update_one(query, verified_data.doc(), None).await?;
                    }
                }

                // Add hash-line
                // ---------------------------------------------------------------------------------
                attrs_map.get_mut(&"hash".to_string()).unwrap().value = self.hash.clone();

                // Post processing
                // ---------------------------------------------------------------------------------
                let result: OutputData = match output_format {
                    // Get Hash-line
                    OutputType::Hash => {
                        let data: String = Self::to_hash(&attrs_map)?;
                        OutputData::Hash((data, verified_data.bool(), verified_data.doc()))
                    }
                    // Get Attribute Map
                    OutputType::Map => OutputData::Map((attrs_map, verified_data.bool(), verified_data.doc())),
                    // Get Json-line
                    OutputType::Json => {
                        let data: String = Self::to_json(&attrs_map)?;
                        OutputData::Json((data, verified_data.bool(), verified_data.doc()))
                    }
                    // Get Html-line
                    OutputType::Html => {
                        let data: String = Self::to_html(attrs_map)?;
                        OutputData::Html((data, verified_data.bool(), verified_data.doc()))
                    }
                };

                Ok(result)
            }

            // Migrating Model
            // *************************************************************************************
            // Check model changes and (if required) apply to the database
            pub async fn migrat<'a>(client: &Client, keyword: &'a str) {
                static MODEL_NAME: &'static str = stringify!($sname);
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                //
                if !FIELD_NAMES.contains(&"hash") {
                    panic!(
                        "Service: `{}` -> Model: `{}` -> Method: `migrat()` : `hash`- Required field.",
                        $service, MODEL_NAME
                    )
                }
                // List field names without `id` field
                let field_names_no_hash: Vec<&'static str> = FIELD_NAMES.iter()
                    .map(|field| field.clone()).filter(|field| field != &"hash").collect();
                // Checking for the presence of fields
                if field_names_no_hash.len() == 0 {
                    panic!("Service: `{}` -> Model: `{}` -> Method: `migrat()` : The model structure has no fields.",
                        $service, MODEL_NAME);
                }
                // Create a map with field types
                let map_field_types: HashMap<&'static str, &'static str> =
                    FIELD_NAMES.iter().map(|item| item.to_owned())
                    .zip([$(stringify!($ftype)),*].iter().map(|item| item.to_owned())).collect();
                // Metadata of model (database name, collection name, etc)
                let meta: Meta = Self::meta().unwrap();
                // Technical database for `models::Monitor`
                let mango_orm_keyword = format!("mango_orm_{}", keyword);
                // Checking the status of Widgets
                let map_widgets: HashMap<&'static str, Widget> = Self::widgets_full_map().unwrap();
                // List of existing databases
                let database_names: Vec<String> =
                    client.list_database_names(None, None).await.unwrap();
                // Map of default values and value types from `value` attribute -
                // (String, String) -> index 0 = type ; index 1 = value
                let mut default_values: HashMap<&'static str, (&'static str, String)> = HashMap::new();

                // Checking Widgets
                // ---------------------------------------------------------------------------------
                // Looping over fields and attributes
                for (field, widget) in map_widgets {
                    // Checking for the correct field name
                    if !FIELD_NAMES.contains(&field) {
                        panic!(
                            "Service: `{}` -> Model: `{}` -> widgets() : `{}` - Incorrect field name.",
                            $service, MODEL_NAME, field
                        )
                    }
                    // Add in map default value
                    default_values.insert(field, (widget.value.get_data_type(), widget.value.get_raw_data()));
                    // Checking attribute states
                    match widget.value {
                        // Hash
                        // -------------------------------------------------------------------------
                        FieldType::Hash => {
                            let enum_field_type = "Hash".to_string();
                            let data_field_type = "String".to_string();
                            if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }

                        // InputCheckBoxText
                        // InputCheckBoxI32
                        // InputCheckBoxU32
                        // InputCheckBoxI64
                        // InputCheckBoxF64
                        // -------------------------------------------------------------------------
                        FieldType::InputCheckBoxText(_) | FieldType::InputCheckBoxI32(_) | FieldType::InputCheckBoxU32(_) | FieldType::InputCheckBoxI64(_) | FieldType::InputCheckBoxF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            match widget.value {
                                FieldType::InputCheckBoxText(_) => {
                                    enum_field_type = "InputCheckBoxText".to_string();
                                    data_field_type = "String".to_string();
                                }
                                FieldType::InputCheckBoxI32(_) => {
                                    enum_field_type = "InputCheckBoxI32".to_string();
                                    data_field_type = "i32".to_string();
                                }
                                FieldType::InputCheckBoxU32(_) => {
                                    enum_field_type = "InputCheckBoxU32".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::InputCheckBoxI64(_) => {
                                    enum_field_type = "InputCheckBoxI64".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::InputCheckBoxF64(_) => {
                                    enum_field_type = "InputCheckBoxF64".to_string();
                                    data_field_type = "f64".to_string();
                                }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `maxlength` = only 0 (zero).",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `other_attrs` - must not contain the word `checked`.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }
                        }

                        // InputColor
                        // InputDate
                        // InputDateTime
                        // InputEmail
                        // InputPassword
                        // InputText
                        // InputUrl
                        // InputIP
                        // InputIPv4
                        // InputIPv6
                        // TextArea
                        // -------------------------------------------------------------------------
                        FieldType::InputColor(_) | FieldType::InputDate(_) | FieldType::InputDateTime(_) | FieldType::InputEmail(_) | FieldType::InputPassword(_) | FieldType::InputText(_) | FieldType::InputUrl(_) | FieldType::InputIP(_) | FieldType::InputIPv4(_) | FieldType::InputIPv6(_) | FieldType::TextArea(_) => {
                            let mut enum_field_type = String::new();
                            match widget.value {
                                FieldType::InputColor(_) => { enum_field_type = "InputColor".to_string(); }
                                FieldType::InputDate(_) => { enum_field_type = "InputDate".to_string(); }
                                FieldType::InputDateTime(_) => { enum_field_type = "InputDateTime".to_string(); }
                                FieldType::InputEmail(_) => { enum_field_type = "InputEmail".to_string(); }
                                FieldType::InputPassword(_) => { enum_field_type = "InputPassword".to_string(); }
                                FieldType::InputText(_) => { enum_field_type = "InputText".to_string(); }
                                FieldType::InputUrl(_) => { enum_field_type = "InputUrl".to_string(); }
                                FieldType::TextArea(_) => { enum_field_type = "TextArea".to_string(); }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if widget.min.get_enum_type() != "U32" ||  widget.max.get_enum_type() != "U32" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The fields `min` and `max` must be of types `StepMinMax::U32`.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }

                        // InputFile
                        // InputImage
                        // -------------------------------------------------------------------------
                        FieldType::InputFile | FieldType::InputImage => {
                            let mut enum_field_type = String::new();
                            match widget.value {
                                FieldType::InputFile => { enum_field_type = "InputFile".to_string(); }
                                FieldType::InputImage => { enum_field_type = "InputImage".to_string(); }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }

                        // InputNumberI32
                        // InputNumberU32
                        // InputNumberI64
                        // InputNumberF64
                        // -------------------------------------------------------------------------
                        FieldType::InputNumberI32(_) | FieldType::InputNumberU32(_) | FieldType::InputNumberI64(_) | FieldType::InputNumberF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            let mut step_min_max_enum_type = String::new();
                            match widget.value {
                                FieldType::InputNumberI32(_) => {
                                    enum_field_type = "InputNumberI32".to_string();
                                    data_field_type = "i32".to_string();
                                    step_min_max_enum_type = "I32".to_string();
                                }
                                FieldType::InputNumberU32(_) => {
                                    enum_field_type = "InputNumberU32".to_string();
                                    data_field_type = "i64".to_string();
                                    step_min_max_enum_type = "U32".to_string();
                                }
                                FieldType::InputNumberI64(_) => {
                                    enum_field_type = "InputNumberI64".to_string();
                                    data_field_type = "i64".to_string();
                                    step_min_max_enum_type = "I64".to_string();
                                }
                                FieldType::InputNumberF64(_) => {
                                    enum_field_type = "InputNumberF64".to_string();
                                    data_field_type = "f64".to_string();
                                    step_min_max_enum_type = "F64".to_string();
                                }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }  else if widget.step.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `step` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            } else if widget.min.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `min` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            } else if widget.max.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `max` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            }
                        }

                        // InputRadioText
                        // InputRadioI32
                        // InputRadioU32
                        // InputRadioI64
                        // InputRadioF64
                        // -------------------------------------------------------------------------
                        FieldType::InputRadioText(_) | FieldType::InputRadioI32(_) | FieldType::InputRadioU32(_) | FieldType::InputRadioI64(_) | FieldType::InputRadioF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            match widget.value {
                                FieldType::InputRadioText(_) => {
                                    enum_field_type = "InputRadioText".to_string();
                                    data_field_type = "String".to_string();
                                }
                                FieldType::InputRadioI32(_) => {
                                    enum_field_type = "InputRadioI32".to_string();
                                    data_field_type = "i32".to_string();
                                }
                                FieldType::InputRadioU32(_) => {
                                    enum_field_type = "InputRadioU32".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::InputRadioI64(_) => {
                                    enum_field_type = "InputRadioI64".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::InputRadioF64(_) => {
                                    enum_field_type = "InputRadioF64".to_string();
                                    data_field_type = "f64".to_string();
                                }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `maxlength` = only 0 (zero).",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.other_attrs.contains("checked") {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `other_attrs` - must not contain the word `checked`.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` - must not be an empty vec![]",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }
                        }

                        // InputRangeI32
                        // InputRangeU32
                        // InputRangeI64
                        // InputRangeF64
                        // -------------------------------------------------------------------------
                        FieldType::InputRangeI32(_) | FieldType::InputRangeU32(_) | FieldType::InputRangeI64(_) | FieldType::InputRangeF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            let mut step_min_max_enum_type = String::new();
                            match widget.value {
                                FieldType::InputRangeI32(_) => {
                                    enum_field_type = "InputRangeI32".to_string();
                                    data_field_type = "i32".to_string();
                                    step_min_max_enum_type = "I32".to_string();
                                }
                                FieldType::InputRangeU32(_) => {
                                    enum_field_type = "InputRangeU32".to_string();
                                    data_field_type = "i64".to_string();
                                    step_min_max_enum_type = "U32".to_string();
                                }
                                FieldType::InputRangeI64(_) => {
                                    enum_field_type = "InputRangeI64".to_string();
                                    data_field_type = "i64".to_string();
                                    step_min_max_enum_type = "I64".to_string();
                                }
                                FieldType::InputRangeF64(_) => {
                                    enum_field_type = "InputRangeI64".to_string();
                                    data_field_type = "f64".to_string();
                                    step_min_max_enum_type = "F64".to_string();
                                }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }  else if widget.step.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `step` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            } else if widget.min.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `min` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            } else if widget.max.get_data_type() != data_field_type {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType = `{}` : `max` = `{}`.",
                                    $service, MODEL_NAME, field, enum_field_type, step_min_max_enum_type
                                )
                            }
                        }

                        // SelectText
                        // SelectI32
                        // SelectU32
                        // SelectI64
                        // SelectF64
                        // -------------------------------------------------------------------------
                         FieldType::SelectText(_) | FieldType::SelectI32(_) | FieldType::SelectU32(_) | FieldType::SelectI64(_) | FieldType::SelectF64(_) => {
                            let mut enum_field_type = String::new();
                            let mut data_field_type = String::new();
                            match widget.value {
                                FieldType::SelectText(_) => {
                                    enum_field_type = "SelectText".to_string();
                                    data_field_type = "String".to_string();
                                }
                                FieldType::SelectI32(_) => {
                                    enum_field_type = "SelectI32".to_string();
                                    data_field_type = "i32".to_string();
                                }
                                FieldType::SelectU32(_) => {
                                    enum_field_type = "SelectU32".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::SelectI64(_) => {
                                    enum_field_type = "SelectI64".to_string();
                                    data_field_type = "i64".to_string();
                                }
                                FieldType::SelectF64(_) => {
                                    enum_field_type = "SelectF64".to_string();
                                    data_field_type = "f64".to_string();
                                }
                                _ => panic!("Invalid field type")
                            }
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `relation_model` = only blank string.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() == 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `{}` : `select` - Should not be empty.",
                                    $service, MODEL_NAME, field, enum_field_type
                                )
                            }  else if data_field_type != map_field_types[field] {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `{}`.",
                                    $service, MODEL_NAME, field, map_field_types[field]
                                )
                            }
                        }

                        // ForeignKey
                        // -------------------------------------------------------------------------
                        FieldType::ForeignKey => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `ForeignKey` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `ForeignKey` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }

                        // ManyToMany
                        // -------------------------------------------------------------------------
                        FieldType::ManyToMany => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `ManyToMany` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `ManyToMany` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }

                        // OneToOne
                        // -------------------------------------------------------------------------
                        FieldType::OneToOne => {
                            if widget.relation_model == String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `OneToOne` : `relation_model` = <CategoryName>::meta().collection.to_string().",
                                    $service, MODEL_NAME, field
                                )
                            }  else if widget.step.get_enum_type() != widget.min.get_enum_type() || widget.step.get_enum_type() != widget.max.get_enum_type() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `step`, `min` and `max` fields must have the same types.",
                                    $service, MODEL_NAME, field
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets -> For `value` = FieldType `OneToOne` : `select` = only blank vec![].",
                                    $service, MODEL_NAME, field
                                )
                            } else if map_field_types[field] != "String" {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> Field: `{}` : Field type is not equal to `String`.",
                                    $service, MODEL_NAME, field
                                )
                            }
                        }
                        _ => panic!("Service: `{}` -> Model: `{}` -> Field: `{}` : `field_type` - Non-existent field type.",
                        $service, MODEL_NAME, field),
                    }
                    // Checking the values of the fields `step`,` min` and `max`
                    match widget.step.get_enum_type() {
                        "I32" => {
                            let step: i32 =  widget.step.get_raw_data().parse().unwrap();
                            let min: i32 =  widget.min.get_raw_data().parse().unwrap();
                            let max: i32 =  widget.max.get_raw_data().parse().unwrap();
                            if step > 0_i32 || min > 0_i32 || max > 0_i32 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `min` attribute must not be greater than `max`.",
                                        $service, MODEL_NAME, field
                                    )
                                } else if step > 0_i32 && (max - min) % step != 0_i32 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The value of the `step` attribute does not match the condition (max - min) % step == 0.",
                                        $service, MODEL_NAME, field
                                    )
                                }
                            }
                        }
                        "U32" | "I64" => {
                            let step: i64 =  widget.step.get_raw_data().parse().unwrap();
                            let min: i64 =  widget.min.get_raw_data().parse().unwrap();
                            let max: i64 =  widget.max.get_raw_data().parse().unwrap();
                            if step > 0_i64 || min > 0_i64 || max > 0_i64 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `min` attribute must not be greater than `max`.",
                                        $service, MODEL_NAME, field
                                    )
                                } else if step > 0_i64 && (max - min) % step != 0_i64 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The value of the `step` attribute does not match the condition (max - min) % step == 0.",
                                        $service, MODEL_NAME, field
                                    )
                                }
                            }
                        }
                        "F64" => {
                            let step: f64 =  widget.step.get_raw_data().parse().unwrap();
                            let min: f64 =  widget.min.get_raw_data().parse().unwrap();
                            let max: f64 =  widget.max.get_raw_data().parse().unwrap();
                            if step > 0_f64 || min > 0_f64 || max > 0_f64 {
                                if min > max {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The `min` attribute must not be greater than `max`.",
                                        $service, MODEL_NAME, field
                                    )
                                } else if step > 0_f64 && (max - min) % step != 0_f64 {
                                    panic!(
                                        "Service: `{}` -> Model: `{}` -> Field: `{}` -> widgets : The value of the `step` attribute does not match the condition (max - min) % step == 0.",
                                        $service, MODEL_NAME, field
                                    )
                                }
                            }
                        }
                        _ => {
                            panic!(
                                "Service: `{}` -> Model: `{}` -> Field: `{}` : Non-existent field type.",
                                $service, MODEL_NAME, field
                            )
                        }
                    }
                }

                // Check the field changes in the Model and (if required)
                // update documents in the current Collection
                // ---------------------------------------------------------------------------------
                // Get a list of current model field names from the technical database `mango_orm_keyword`
                let filter: Document = doc! {
                    "database": &meta.database,
                    "collection": &meta.collection
                };
                let model: Option<Document> = client.database(&mango_orm_keyword)
                    .collection("models").find_one(filter, None).await.unwrap();
                if model.is_some() {
                    let mango_orm_fnames: Vec<String> = {
                        let model: Document = model.unwrap();
                        let fields: Vec<Bson> = model.get_array("fields").unwrap().to_vec();
                        fields.into_iter().map(|item: Bson| item.as_str().unwrap().to_string()).collect()
                    };
                    // Check if the set of fields in the collection of the current Model needs to be updated
                    let mut run_documents_modification: bool = false;
                    if field_names_no_hash.len() != mango_orm_fnames.len() {
                        run_documents_modification = true;
                    } else {
                        for item in field_names_no_hash {
                            if mango_orm_fnames.iter().any(|item2| item2 != &item) {
                                run_documents_modification = true;
                                break;
                            }
                        }
                    }
                    // Start (if necessary) updating the set of fields in the current collection
                    if run_documents_modification {
                        // Get the database and collection of the current Model
                        let db: Database = client.database(&meta.database);
                        let collection: Collection = db.collection(&meta.collection);
                        // Get cursor to all documents of the current Model
                        let mut cursor: Cursor = collection.find(None, None).await.unwrap();
                        // Iterate through all documents in a current (model) collection
                        while let Some(result) = cursor.next().await {
                            let curr_doc: Document = result.unwrap();
                            // Create temporary blank document
                            let mut tmp_doc = doc! {};
                            // Loop over all fields of the model
                            for field in FIELD_NAMES {
                                if field == &"hash" {
                                    continue;
                                }
                                // If the field exists, get its value
                                if curr_doc.contains_key(field) {
                                    for item in curr_doc.iter() {
                                        if item.0 == field {
                                            tmp_doc.insert(field.to_string(), item.1);
                                            break;
                                        }
                                    }
                                } else {
                                    // If no field exists, get default value
                                    let value = &default_values[field];
                                    tmp_doc.insert(field.to_string(), match value.0 {
                                        "String" => Bson::String(value.1.clone()),
                                        "i32" => Bson::Int32(value.1.parse::<i32>().unwrap()),
                                        "u32" | "i64" => Bson::Int64(value.1.parse::<i64>().unwrap()),
                                        "f64" => Bson::Double(value.1.parse::<f64>().unwrap()),
                                        "bool" => Bson::Boolean(value.1.parse::<bool>().unwrap()),
                                        "none" => Bson::Null,
                                        _ => {
                                            panic!("Service: `{}` -> Model: `{}` -> Method: `migrat()` : Invalid data type.",
                                                $service, MODEL_NAME)
                                        }
                                    });
                                }
                            }
                            // Save updated document
                            let query = doc! {"_id": curr_doc.get_object_id("_id").unwrap()};
                            let update = UpdateModifications::Document(tmp_doc);
                            collection.update_one(query, update, None).await.unwrap();
                        }
                    }
                }

                // Create a new database (if doesn't exist) and add new collection
                // ---------------------------------------------------------------------------------
                // Get the database for the current collection of Model
                let db: Database = client.database(&meta.database);
                // If there is no collection for the current Model, create it
                if !database_names.contains(&meta.database) ||
                    !db.list_collection_names(None).await.unwrap().contains(&meta.collection) {
                    db.create_collection(&meta.collection, None).await.unwrap();
                }

                // Update the state of models for `models::Monitor`
                // ---------------------------------------------------------------------------------
                // Get the technical database `mango_orm_keyword` for the current model
                let db: Database = client.database(&mango_orm_keyword);
                // Check if there is a technical database of the project, if not, causes panic
                if !database_names.contains(&mango_orm_keyword) ||
                    !db.list_collection_names(None).await.unwrap().contains(&"models".to_owned()) {
                    panic!("For migration not used `models::Monitor.refresh()`.");
                } else {
                    let collection = db.collection("models");
                    let filter = doc! {"database": &meta.database, "collection": &meta.collection};
                    let doc = doc!{
                        "database": &meta.database,
                        "collection": &meta.collection,
                        "fields": FIELD_NAMES.iter().map(|item| item.to_string())
                            .filter(|item| item != "hash").collect::<Vec<String>>(),
                        "status": true
                    };
                    // Check if there is model state in the database
                    if collection.count_documents(filter.clone(), None).await.unwrap() == 0_i64 {
                        // Add model state information
                        collection.insert_one(doc, None).await.unwrap();
                    } else {
                        // Update model state information
                        let update = UpdateModifications::Document(doc);
                        collection.update_one(filter, update, None).await.unwrap();
                    }
                }
            }
        }
    }
}

// TESTS
// #################################################################################################
#[cfg(test)]
mod tests {
    //
}
