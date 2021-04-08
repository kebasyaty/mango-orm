//! # Caching.
//! Caching information about Models for speed up work.
//!
//! Trait:
//! `Caching` - Methods caching information about Models for speed up work.
//!
//! Methods:
//! `to_cache` - Add metadata and widgects map to cache.
//! `form_wig` - Get an widgets map for page template.
//! `form_json` - Get Form attributes in Json format for page templates.
//! `form_json_for_admin` - Json-line for admin panel.
//! `form_html` - Get Html Form of Model for page templates.
//! `get_cache_data_for_query` - Get cached Model data.
//! `db_update_dyn_widgets` - Accepts json-line to update data, for dynamic widgets.
//!

use crate::{
    forms::Widget,
    models::{Meta, ToModel},
    store::{FormCache, FORM_STORE, MONGODB_CLIENT_STORE},
};

// Caching information about Models for speed up work.
// #################################################################################################
pub trait CachingModel: ToModel {
    // Add metadata and widgects map to cache.
    // *********************************************************************************************
    fn to_cache() -> Result<(), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get write access in cache.
        let mut form_store = FORM_STORE.write()?;
        // Create `FormCache` default and add map of widgets and metadata of model.
        let meta: Meta = Self::meta()?;
        // Get MongoDB client for current model.
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let client_cache: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
        // Get a widget map.
        let mut map_widgets: std::collections::HashMap<String, Widget> = Self::widgets()?;
        // Enrich the widget map with values for dynamic widgets.
        Self::vitaminize(
            meta.project_name.as_str(),
            meta.unique_project_key.as_str(),
            meta.collection_name.as_str(),
            &client_cache,
            &mut map_widgets,
        )?;
        // Init new FormCache.
        let new_form_cache = FormCache {
            meta,
            map_widgets,
            ..Default::default()
        };
        // Save structure `FormCache` to store.
        form_store.insert(key, new_form_cache);
        //
        Ok(())
    }

    // Get an widgets map for page template.
    // *********************************************************************************************
    fn form_wig() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::to_cache()?;
            // Reaccess.
            form_store = FORM_STORE.read()?;
        }
        // Get data and return the result.
        if let Some(form_cache) = form_store.get(key.as_str()) {
            Ok(form_cache.map_widgets.clone())
        } else {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` -> Method: `form_wig()` : Failed to get data from cache.",
                meta.model_name
            ))?
        }
    }

    // Get Form attributes in Json format for page templates.
    // *********************************************************************************************
    fn form_json() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::to_cache()?;
            // Reaccess.
            form_store = FORM_STORE.read()?;
        }
        // Generate data and return the result.
        if let Some(form_cache) = form_store.get(key.as_str()) {
            if form_cache.form_json.is_empty() {
                drop(form_store);
                let mut form_store = FORM_STORE.write()?;
                let form_cache = form_store.get(key.as_str()).unwrap();
                let json = serde_json::to_string(&form_cache.map_widgets.clone())?;
                let mut new_form_cache = form_cache.clone();
                new_form_cache.form_json = json.clone();
                form_store.insert(key, new_form_cache);
                return Ok(json);
            }
            Ok(form_cache.form_json.clone())
        } else {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` -> Method: `form_json()` : Failed to get data from cache.",
                meta.model_name
            ))?
        }
    }

    // Json-line for admin panel.
    // ( converts a widget map to a list, in the order of the Model fields )
    // ---------------------------------------------------------------------------------------------
    fn form_json_for_admin() -> Result<String, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let (form_cache, _client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        let map_widgets = form_cache.map_widgets.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        // Get a list of widgets in the order of the model fields.
        for field_name in meta.fields_name.iter() {
            let widget = map_widgets.get(field_name).unwrap().clone();
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    // Get Html Form of Model for page templates.
    // *********************************************************************************************
    fn form_html() -> Result<String, Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::to_cache()?;
            // Reaccess.
            form_store = FORM_STORE.read()?;
        }
        // Generate data and return the result.
        if let Some(form_cache) = form_store.get(key.as_str()) {
            if form_cache.form_html.is_empty() {
                drop(form_store);
                let mut form_store = FORM_STORE.write()?;
                let form_cache = form_store.get(key.as_str()).unwrap();
                let html =
                    Self::to_html(&form_cache.meta.fields_name, form_cache.map_widgets.clone());
                let mut new_form_cache = form_cache.clone();
                new_form_cache.form_html = html.clone();
                form_store.insert(key, new_form_cache);
                return Ok(html);
            }
            Ok(form_cache.form_html.clone())
        } else {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` -> Method: `form_html()` : Failed to get data from cache.",
                meta.model_name
            ))?
        }
    }

    // Get cached Model data.
    // *********************************************************************************************
    fn get_cache_data_for_query(
    ) -> Result<(FormCache, mongodb::sync::Client), Box<dyn std::error::Error>> {
        // Get a key to access Model data in the cache.
        let key: String = Self::key();
        // Get read access from cache.
        let mut form_store = FORM_STORE.read()?;
        // Check if there is metadata for the Model in the cache.
        if !form_store.contains_key(key.as_str()) {
            // Unlock.
            drop(form_store);
            // Add map of widgets to cache.
            Self::to_cache()?;
            // Reaccess.
            form_store = FORM_STORE.read()?;
        }
        // Generate data and return the result.
        if let Some(form_cache) = form_store.get(key.as_str()) {
            // Get model metadata from cache.
            let meta: &Meta = &form_cache.meta;
            // Get MongoDB client for current model.
            let client_store = MONGODB_CLIENT_STORE.read()?;
            let client: &mongodb::sync::Client = client_store.get(&meta.db_client_name).unwrap();
            //
            Ok((form_cache.clone(), client.clone()))
        } else {
            let meta = Self::meta()?;
            Err(format!(
                "Model: `{}` -> Method: `get_cache_data_for_query()` : Failed to get data from cache.",
                meta.model_name
            ))?
        }
    }

    // Accepts json-line to update data, for dynamic widgets.
    // Hint: Used in conjunction with the admin panel.
    // Example (json-line): {"field_name":[["value","Title"]]}
    // or
    // r#"{
    //    "field_name":[["value","Title"]],
    //    "field_name_2":[["value","Title 2"]],
    //    "field_name_3":[["value","Title 3"]]
    // }"#
    // *********************************************************************************************
    fn db_update_dyn_widgets(json_line: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Refresh the state in the technical database.
        // -----------------------------------------------------------------------------------------
        // Validation json-line.
        let re = regex::RegexBuilder::new(r#"^\{[\s]*(?:"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*)*\]))(?:,[\s]*"[a-z][a-z\d]*(?:_[a-z\d]+)*":(?:\[(?:(?:\["[-_.\s\w]+","[-_.\s\w]+"\])(?:,\["[-_.\s\w]+","[-_.\s\w]+"\])*)*\]))*[\s]*\}$"#)
            .case_insensitive(true)
            .build()
            .unwrap();
        if !re.is_match(json_line) {
            Err(format!(
                r#"Model: {} > Method: `db_update_dyn_widgets()` : \
                   The `json_line` parameter was not validation. \
                   Example: {{"field_name":[["value","Title"]]}}"#,
                Self::meta()?.model_name
            ))?
        }

        // Get cached Model data.
        let (form_cache, client_cache) = Self::get_cache_data_for_query()?;
        // Get Model metadata.
        let meta: Meta = form_cache.meta;
        let mango_tech_keyword = format!(
            "mango_tech__{}__{}",
            meta.project_name.clone(),
            meta.unique_project_key.clone()
        );
        let db = client_cache.database(&mango_tech_keyword);
        let coll = db.collection("dynamic_widgets");
        let query = mongodb::bson::doc! {
            "database": meta.database_name.clone(),
            "collection": meta.collection_name.clone()
        };
        let new_dyn_data: serde_json::Value = serde_json::from_str(json_line)?;
        let new_dyn_data =
            serde_json::from_value::<mongodb::bson::document::Document>(new_dyn_data)?;
        let mut curr_dyn_date = coll.find_one(query.clone(), None)?.unwrap();
        let dyn_date = curr_dyn_date.get_document_mut("fields").unwrap();

        for (field_name, bson_val) in new_dyn_data {
            dyn_date.insert(field_name.as_str(), bson_val);
        }

        let update = mongodb::bson::doc! {
            "$set": { "fields": dyn_date.clone() }
        };
        coll.update_one(query, update, None)?;

        // Clean up orphaned (if any) data.
        // -----------------------------------------------------------------------------------------
        let db = client_cache.database(meta.database_name.as_str());
        let coll = db.collection(meta.collection_name.as_str());
        let mut cursor = coll.find(None, None)?;
        // Iterate over all documents in the collection.
        while let Some(db_doc) = cursor.next() {
            let mut is_changed = false;
            let mut curr_doc = db_doc.clone()?;
            // Iterate over all fields in the document.
            for (field_name, widget_type) in meta.map_widget_type.clone() {
                // Choosing the only dynamic widgets.
                if widget_type.contains("Dyn") {
                    if curr_doc.is_null(field_name.as_str()) {
                        continue;
                    }
                    // Get a list of values to match.
                    let dyn_vec: Vec<String> = dyn_date
                        .get_array(field_name.as_str())?
                        .iter()
                        .map(|item| item.as_array().unwrap()[0].as_str().unwrap().to_string())
                        .collect();
                    // Selecting widgets with multi-selection support.
                    if widget_type.contains("Mult") {
                        let mut new_arr_bson = Vec::<mongodb::bson::Bson>::new();
                        if widget_type.contains("Text") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_str().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("I32") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_i32().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("U32") || widget_type.contains("I64") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_i64().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        } else if widget_type.contains("F64") {
                            let arr_bson = curr_doc.get_array(field_name.as_str())?;
                            new_arr_bson = arr_bson
                                .iter()
                                .map(|item| item.clone())
                                .filter(|item| {
                                    dyn_vec.contains(&item.as_f64().unwrap().to_string())
                                })
                                .collect();
                            if new_arr_bson != *arr_bson {
                                is_changed = true;
                            }
                        }
                        if is_changed {
                            if !new_arr_bson.is_empty() {
                                curr_doc
                                    .insert(field_name, mongodb::bson::Bson::Array(new_arr_bson));
                            } else {
                                curr_doc.insert(field_name, mongodb::bson::Bson::Null);
                            }
                        }
                    } else {
                        let mut val = String::new();
                        // Select widgets with support for one selection.
                        if widget_type.contains("Text") {
                            val = curr_doc.get_str(field_name.as_str())?.to_string();
                        } else if widget_type.contains("I32") {
                            val = curr_doc.get_i32(field_name.as_str())?.to_string();
                        } else if widget_type.contains("U32") || widget_type.contains("I64") {
                            val = curr_doc.get_i64(field_name.as_str())?.to_string();
                        } else if widget_type.contains("F64") {
                            val = curr_doc.get_f64(field_name.as_str())?.to_string();
                        }
                        if !dyn_vec.contains(&val) {
                            curr_doc.insert(field_name, mongodb::bson::Bson::Null);
                            is_changed = true;
                        }
                    }
                }
            }
            if is_changed {
                // Update values for dynamic widgets.
                // ---------------------------------------------------------------------------------
                let query = mongodb::bson::doc! {"_id": curr_doc.get_object_id("_id")?};
                coll.update_one(query, curr_doc, None)?;
            }
        }

        // Update metadata and widgects map to cache.
        // -----------------------------------------------------------------------------------------
        Self::to_cache()?;
        //
        Ok(())
    }
}
