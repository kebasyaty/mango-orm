//! # Migration of Models to database.
//!
//! Structs:
//! `ModelState` - Creation and updating of a technical database for monitoring the state of models.
//! `Monitor` - Creation and updating of a technical database for monitoring the state of models.
//!
//! Methods:
//! `mango_tech_name` - Get the name of the technical database for a project.
//! `refresh` - Refresh models state.
//! `napalm` - Reorganize databases state.
//! `migrat` - Check the changes in the models and (if necessary) apply to the database.
//!

use crate::{
    forms::{FileData, ImageData},
    store::MONGODB_CLIENT_STORE,
};
use mongodb::{
    bson, bson::document::Document, options::UpdateModifications, sync::Client, sync::Collection,
    sync::Cursor, sync::Database,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// MIGRATION
// #################################################################################################
// Creation and updating of a technical database for monitoring the state of models.
#[derive(Serialize, Deserialize)]
pub struct ModelState {
    pub database: String,
    pub collection: String,
    pub fields: Vec<String>,
    pub map_widgets: HashMap<String, String>,
    pub status: bool,
}

pub struct Monitor<'a> {
    pub project_name: &'a str,
    pub unique_project_key: &'a str,
    pub models: Vec<crate::models::Meta>,
}

impl<'a> Monitor<'a> {
    // Get the name of the technical database for a project.
    // *********************************************************************************************
    pub fn mango_tech_name(&self) -> Result<String, Box<dyn std::error::Error>> {
        // PROJECT_NAME Validation.
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 21
        let re = Regex::new(r"^[a-zA-Z][_a-zA-Z\d]{1,21}$")?;
        if !re.is_match(self.project_name) {
            Err(format!("PROJECT_NAME - Valid characters: _ a-z A-Z 0-9 ; \
                         Max size: 21 ; \
                         First character: a-z A-Z"))?
        }
        // UNIQUE_PROJECT_KEY Validation.
        // UNIQUE_PROJECT_KEY - It is recommended not to change.
        // Valid characters: a-z A-Z 0-9
        // Size: 8-16
        // Example: "7rzgacfqQB3B7q7T"
        let re = Regex::new(r"^[a-zA-Z\d]{8,16}$")?;
        if !re.is_match(self.unique_project_key) {
            Err(format!("UNIQUE_PROJECT_KEY - Valid characters: a-z A-Z 0-9 ; \
                         Size: 8-16."))?
        }
        //
        Ok(format!("mango_tech__{}__{}", self.project_name, self.unique_project_key))
    }

    // Refresh models state.
    // *********************************************************************************************
    /*
        if {
            If there is no technical database, it will be created.
        } else {
            Resets the Model's status to `false`.
        }
    */
    fn refresh(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get cache MongoDB clients.
        let client_store: std::sync::RwLockReadGuard<HashMap<String, Client>> =
            MONGODB_CLIENT_STORE.read()?;
        //
        for meta in self.models.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Get the name of the technical database for a project.
            let db_mango_tech: String = self.mango_tech_name()?;
            // Collection for monitoring the state of Models.
            let collection_models_name: &str = "monitor_models";
            // Used to store selection items, for 
            // widgets like selectTextDyn, selectTextMultDyn, etc.
            let collection_dyn_widgets_name: &str = "dynamic_widgets";
            //Get a list of databases.
            let database_names: Vec<String> = client.list_database_names(None, None)?;
            // Create a technical database for the project if it doesn't exist.
            if !database_names.contains(&db_mango_tech) {
                // Create a collection for models.
                client
                    .database(&db_mango_tech)
                    .create_collection(collection_models_name, None)?;
                // Create a collection for widget types of `select`.
                // (selectTextDyn, selectTextMultDyn, etc.)
                client
                    .database(&db_mango_tech)
                    .create_collection(collection_dyn_widgets_name, None)?;
            } else {
                // Reset models state information.
                let mango_tech_db: Database = client.database(&db_mango_tech);
                let collection_models: Collection = mango_tech_db.collection(collection_models_name);
                let mut cursor: Cursor = collection_models.find(None, None)?;

                while let Some(result) = cursor.next() {
                    let document = result?;
                    let mut model_state: ModelState = bson::de::from_document(document)?;
                    model_state.status = false;
                    let query: Document = bson::doc! {
                        "database": &model_state.database,
                        "collection": &model_state.collection
                    };
                    let update = bson::ser::to_document(&model_state)?;
                    collection_models.update_one(query, update, None)?;
                }
            }
        }
        //
        Ok(())
    }

    // Reorganize databases state.
    // (full delete of orphaned collections and databases)
    // *********************************************************************************************
    fn napalm(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Get cache MongoDB clients.
        let client_store: std::sync::RwLockReadGuard<HashMap<String, Client>> =
            MONGODB_CLIENT_STORE.read()?;
        //
        for meta in self.models.iter() {
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            // Get the name of the technical database for a project.
            let db_mango_tech: String = self.mango_tech_name()?;
            let collection_models_name: &str = "monitor_models";
            let collection_dyn_widgets_name: &str = "dynamic_widgets";
            let mango_tech_db: Database = client.database(&db_mango_tech);
            let collection_models: Collection = mango_tech_db.collection(collection_models_name);
            let collection_dyn_widgets: Collection = mango_tech_db.collection(collection_dyn_widgets_name);
            // Delete orphaned Collections.
            let cursor: Cursor = collection_models.find(None, None)?;
            let results: Vec<Result<Document, mongodb::error::Error>> = cursor.collect();
            for result in results {
                let document = result?;
                let model_state: ModelState = bson::de::from_document(document)?;
                if !model_state.status {
                    // Delete Collection (left without a model).
                    client
                        .database(&model_state.database)
                        .collection(&model_state.collection)
                        .drop(None)?;
                    // Delete a document with a record about the state of
                    // the model from the technical base.
                    let query: Document = bson::doc! {
                        "database": &model_state.database,
                        "collection": &model_state.collection
                    };
                    collection_models.delete_one(query.clone(), None)?;
                    collection_dyn_widgets.delete_one(query, None)?;
                }
            }
        }
        //
        Ok(())
    }

    // Migrating Models.
    // *********************************************************************************************
    // Check the changes in the models and (if necessary) apply to the database.
    pub fn migrat(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Run refresh models state.
        self.refresh()?;
        // Get cache MongoDB clients.
        let client_store: std::sync::RwLockReadGuard<HashMap<String, Client>> =
            MONGODB_CLIENT_STORE.read()?;

        // Get model metadata
        for meta in self.models.iter() {
            // Service_name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{1,31}$").unwrap().is_match(meta.service_name.as_str()) {
                Err(format!("Model: `{}` : Service_name - Valid characters: _ a-z A-Z 0-9 \
                             ; Max size: 31 ; First character: _ a-z A-Z", meta.model_name))?;
            }
            // Database name validation.
            if !Regex::new(r"^[_a-zA-Z][_a-zA-Z\d]{14,62}$").unwrap().is_match(meta.database_name.as_str()) {
                Err(format!("Model: `{}` : Database name - Valid characters: _ a-z A-Z 0-9 \
                             ; Max size: 21 ; First character: _ a-z A-Z", meta.model_name))?;
            }
            //
            let client: &Client = client_store.get(&meta.db_client_name).unwrap();
            let fields_name: Vec<&str> =
                meta.fields_name.iter().map(|item| item.as_str()).collect();
            let ignore_fields: Vec<&str> = meta
                .ignore_fields
                .iter()
                .map(|item| item.as_str())
                .collect();
            // List field names without `hash` and ignored fields.
            let trunc_list_fields_name: Vec<&str> = fields_name
                .iter()
                .filter(|item| **item != "hash" && !ignore_fields.contains(item))
                .map(|item| *item)
                .collect();
            // Get the name of the technical database for a project.
            let db_mango_tech: String = self.mango_tech_name()?;
            let database_names: Vec<String> = client.list_database_names(None, None)?;
            // Map of default values and value types from `value (default)` attribute -
            // <field_name, (widget_type, value)>
            let map_default_values: HashMap<String, (String, String)> =
                meta.map_default_values.clone();
            // Get map of widgets types.
            let map_widget_type = meta.map_widget_type.clone();
            // Get truncated map of widgets types.
            let trunc_map_widget_type: HashMap<String, String> = map_widget_type.clone();
            trunc_map_widget_type.clone().retain(|k, _| k != "hash" && !ignore_fields.contains(&k.as_str()));
            // Get a map of widgets from the technical database,
            // from the `monitor_models` collection for current Model.
            let monitor_map_widget_type: HashMap<String, String>;

            // Check the field changes in the Model and (if required)
            // update documents in the current Collection.
            // -------------------------------------------------------------------------------------
            // Get a list of current model field names from the technical database
            // `mango_orm_keyword`.
            let filter: Document = mongodb::bson::doc! {
                "database": &meta.database_name,
                "collection": &meta.collection_name
            };
            let model: Option<Document> = client
                .database(&db_mango_tech)
                .collection("monitor_models")
                .find_one(filter, None)?;
            if model.is_some() {
                let model: Document = model.unwrap();
                // Get a list of fields from the technical database,
                // from the `monitor_models` collection for current Model.
                let monitor_models_fields_name: Vec<String> = {
                    let fields: Vec<mongodb::bson::Bson> =
                        model.get_array("fields")?.to_vec();
                    fields
                        .into_iter()
                        .map(|item| item.as_str().unwrap().to_string())
                        .collect()
                };
                // Get a map of widgets from the technical database,
                // from the `monitor_models` collection for current Model.
                monitor_map_widget_type = {
                    model.get_document("map_widgets")
                        .unwrap().iter()
                        .map(|item| (item.0.clone(), item.1.as_str().unwrap().to_string()))
                        .collect()
                };
                // Check if the set of fields in the collection of
                // the current Model needs to be updated.
                let mut changed_fields: Vec<&str> = Vec::new();
                for field in trunc_list_fields_name.iter() {
                    if !monitor_models_fields_name.contains(&field.to_string()) || 
                        (trunc_map_widget_type.get(*field).unwrap() != 
                            monitor_map_widget_type.get(*field).unwrap_or(&String::new())) {
                        changed_fields.push(field);
                    }
                }
                // Start (if necessary) updating the set of fields in the current collection.
                if !changed_fields.is_empty() {
                    // Get the database and collection of the current Model.
                    let db: Database = client.database(&meta.database_name);
                    let collection: mongodb::sync::Collection =
                        db.collection(&meta.collection_name);
                    // Get cursor to all documents of the current Model.
                    let mut cursor: mongodb::sync::Cursor = collection.find(None, None)?;
                    // Iterate through all documents in a current (model) collection.
                    while let Some(result) = cursor.next() {
                        let doc_from_db: mongodb::bson::document::Document = result.unwrap();
                        // Create temporary blank document.
                        let mut tmp_doc = mongodb::bson::document::Document::new();
                        // Loop over all fields of the model.
                        for field in fields_name.iter() {
                            if *field == "hash" || ignore_fields.contains(&field) {
                                continue;
                            }
                            // If the field exists, get its value.
                            if !changed_fields.contains(field) {
                                let value_from_db: Option<&mongodb::bson::Bson> =
                                    doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    Err(format!("Service: `{}` > Model: `{}` > Field: `{}` > \
                                                 Method: `migrat()` : \
                                                 Can't get field value from database.",
                                        meta.service_name, meta.model_name, field))?;
                                }
                            } else {
                                // If no field exists, get default value.
                                let value = map_default_values.get(*field).unwrap();
                                tmp_doc.insert(
                                    field.to_string(),
                                    match value.0.as_str() {
                                        "checkBoxText" | "radioText" | "inputColor"
                                        | "inputEmail" | "inputPassword" | "inputPhone"
                                        | "inputText" | "inputUrl" | "inputIP" | "inputIPv4"
                                        | "inputIPv6" | "textArea" | "selectText" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::String(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputDate" => {
                                            // Example: "1970-02-28".
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !crate::store::REGEX_IS_DATE.is_match(&val) {
                                                    Err(format!("Service: `{}` > Model: `{}` > \
                                                                 Method: `widgets()` : Incorrect date \
                                                                 format. Example: 1970-02-28",
                                                        meta.service_name, meta.model_name))?
                                                }
                                                let val = format!("{}T00:00", val);
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            &val,
                                                            "%Y-%m-%dT%H:%M",
                                                        )?, chrono::Utc,
                                                    );
                                                mongodb::bson::Bson::DateTime(dt)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputDateTime" => {
                                            // Example: "1970-02-28T00:00".
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                if !crate::store::REGEX_IS_DATETIME.is_match(&val) {
                                                    Err(format!("Service: `{}` > Model: `{}` > \
                                                                 Method: `widgets()` : \
                                                                 Incorrect date and time format. \
                                                                 Example: 1970-02-28T00:00",
                                                        meta.service_name, meta.model_name
                                                    ))?
                                                }
                                                let dt: chrono::DateTime<chrono::Utc> =
                                                    chrono::DateTime::<chrono::Utc>::from_utc(
                                                        chrono::NaiveDateTime::parse_from_str(
                                                            &val,
                                                            "%Y-%m-%dT%H:%M",
                                                        )?, chrono::Utc,
                                                    );
                                                mongodb::bson::Bson::DateTime(dt)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "radioI32" | "numberI32"
                                        | "rangeI32" | "selectI32" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Int32(
                                                    val.parse::<i32>()?
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "radioU32" | "numberU32" | "rangeU32"
                                        | "selectU32" | "checkBoxI64" | "radioI64"
                                        | "numberI64" | "rangeI64" | "selectI64" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Int64(
                                                    val.parse::<i64>()?
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "radioF64" | "numberF64" | "rangeF64" | "selectF64" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Double(
                                                    val.parse::<f64>()?
                                                )
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "checkBox" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                mongodb::bson::Bson::Boolean(
                                                    val.parse::<bool>()?
                                                )
                                            } else {
                                                mongodb::bson::Bson::Boolean(false)
                                            }
                                        }
                                        "inputFile" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let mut file_data = 
                                                    serde_json::from_str::<FileData>(val.as_str())?;
                                                // Define flags to check.
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    Err(format!("Model: `{}` > Field: `{}` > Method: \
                                                                 `migrat()` : Check the `path` and `url` \
                                                                 attributes in the `default` field parameter.",
                                                        meta.model_name, field)
                                                    )?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = std::path::Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` > Method: \
                                                                 `migrat()` : File is missing - {}",
                                                        meta.model_name, field, path)
                                                    )?
                                                }
                                                // Get file metadata.
                                                let metadata: std::fs::Metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as u32;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Create doc.
                                                let result = mongodb::bson::ser::to_document(&file_data)?;
                                                mongodb::bson::Bson::Document(result)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "inputImage" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let mut file_data = 
                                                    serde_json::from_str::<ImageData>(val.as_str())?;
                                                // Define flags to check.
                                                let is_emty_path = file_data.path.is_empty();
                                                let is_emty_url = file_data.url.is_empty();
                                                if (!is_emty_path && is_emty_url)
                                                    || (is_emty_path && !is_emty_url) {
                                                    Err(format!("Model: `{}` > Field: `{}` > Method: \
                                                                 `migrat()` : Check the `path` and `url` \
                                                                 attributes in the `default` field parameter.",
                                                        meta.model_name, field
                                                    ))?
                                                }
                                                // Create path for validation of file.
                                                let path: String = file_data.path.clone();
                                                let f_path = std::path::Path::new(path.as_str());
                                                if !f_path.exists() || !f_path.is_file() {
                                                    Err(format!("Model: `{}` > Field: `{}` > Method: \
                                                                 `migrat()` : File is missing - {}",
                                                        meta.model_name, field, path
                                                    ))?
                                                }
                                                // Get file metadata.
                                                let metadata: std::fs::Metadata = f_path.metadata()?;
                                                // Get file size in bytes.
                                                file_data.size = metadata.len() as u32;
                                                // Get file name.
                                                file_data.name = f_path.file_name().unwrap().to_str().unwrap().to_string();
                                                // Get image width and height.
                                                let dimensions: (u32, u32) = image::image_dimensions(path)?;
                                                file_data.width = dimensions.0;
                                                file_data.height = dimensions.1;
                                                // Create doc.
                                                let result = mongodb::bson::ser::to_document(&file_data)?;
                                                mongodb::bson::Bson::Document(result)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "selectTextMult" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let val = serde_json::from_str::<Vec<String>>(val.as_str())?
                                                    .iter().map(|item| mongodb::bson::Bson::String(item.clone()))
                                                    .collect::<Vec<mongodb::bson::Bson>>();
                                                mongodb::bson::Bson::Array(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "selectI32Mult" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let val = serde_json::from_str::<Vec<i32>>(val.as_str())?
                                                    .iter().map(|item| mongodb::bson::Bson::Int32(item.clone()))
                                                    .collect::<Vec<mongodb::bson::Bson>>();
                                                mongodb::bson::Bson::Array(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                         "selectU32Mult" | "selectI64Mult"  => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let val = serde_json::from_str::<Vec<i64>>(val.as_str())?
                                                    .iter().map(|item| mongodb::bson::Bson::Int64(item.clone()))
                                                    .collect::<Vec<mongodb::bson::Bson>>();
                                                mongodb::bson::Bson::Array(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "selectF64Mult" => {
                                            let val: String = value.1.clone();
                                            if !val.is_empty() {
                                                let val = serde_json::from_str::<Vec<f64>>(val.as_str())?
                                                    .iter().map(|item| mongodb::bson::Bson::Double(item.clone()))
                                                    .collect::<Vec<mongodb::bson::Bson>>();
                                                mongodb::bson::Bson::Array(val)
                                            } else {
                                                mongodb::bson::Bson::Null
                                            }
                                        }
                                        "selectTextDyn" | "selectTextMultDyn" | "selectI32Dyn"
                                        | "selectI32MultDyn" | "selectU32Dyn" | "selectU32MultDyn"
                                        | "selectI64Dyn" | "selectI64MultDyn" | "selectF64Dyn"
                                        | "selectF64MultDyn" => {
                                            mongodb::bson::Bson::Null
                                        }
                                        _ => {
                                            Err(format!("Service: `{}` > Model: `{}` > Method: \
                                                         `migrat()` : Invalid Widget type.",
                                                meta.service_name, meta.model_name
                                            ))?
                                        }
                                    },
                                );
                            }
                        }
                        // Insert the reserved fields.
                        for field in vec!["created_at", "updated_at"] {
                            if doc_from_db.contains_key(field) {
                                let value_from_db: Option<&mongodb::bson::Bson> =
                                    doc_from_db.get(field);
                                if value_from_db.is_some() {
                                    tmp_doc.insert(field.to_string(), value_from_db.unwrap());
                                } else {
                                    Err(format!("Service: `{}` > Model: `{}` > \
                                                 Method: `migrat()` : \
                                                 Cannot get field value from database for \
                                                 field `{}`.",
                                        meta.service_name, meta.model_name, field
                                    ))?
                                }
                            } else {
                                Err(format!("Service: `{}` > Model: `{}` > Method: `migrat()` : \
                                             Key `{}` was not found in the document from \
                                             the database.",
                                    meta.service_name, meta.model_name, field
                                ))?
                            }
                        }
                        // Save updated document.
                        let query = mongodb::bson::doc! {"_id": doc_from_db.get_object_id("_id")?};
                        collection.update_one(query, tmp_doc, None)?;
                    }
                }
            } else {
                monitor_map_widget_type = HashMap::new();
            }

            // Create a new database (if doesn't exist) and add new collection.
            // -------------------------------------------------------------------------------------
            // Get the database for the current collection of Model.
            let db: Database = client.database(&meta.database_name);
            // If there is no collection for the current Model, create it.
            if !database_names.contains(&meta.database_name)
                || !db.list_collection_names(None)?.contains(&meta.collection_name)
            {
                db.create_collection(&meta.collection_name, None)?;
            }

            // Get the technical database `db_mango_tech` for the current model.
            // -------------------------------------------------------------------------------------
            let db: Database = client.database(&db_mango_tech);

            // Update the state of models for `models::Monitor`.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_mango_tech)
                || !db.list_collection_names(None)?.contains(&"monitor_models".to_owned())
            {
                Err(format!("In the `refresh()` method, \
                             no technical database has been created for the project."))?
            } else {
                let collection: Collection = db.collection("monitor_models");
                let filter: Document = mongodb::bson::doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name
                };
                let doc: Document = mongodb::bson::doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name,
                    "fields": trunc_list_fields_name.iter().map(|item| item.to_string())
                        .collect::<Vec<String>>(),
                    "map_widgets": bson::ser::to_bson(&trunc_map_widget_type.clone())?,
                    "status": true
                };
                // Check if there is model state in the database.
                if collection.count_documents(filter.clone(), None)?  == 0_i64 {
                    // Add model state information.
                    collection.insert_one(doc, None)?;
                } else {
                    // Full update model state information.
                    let update = UpdateModifications::Document(doc);
                    collection.update_one(filter, update, None)?;
                }
            }

            // Document management to support model fields with dynamic widgets.
            // -------------------------------------------------------------------------------------
            // Check if there is a technical database of the project, if not, causes panic.
            if !database_names.contains(&db_mango_tech)
                || !db.list_collection_names(None)?.contains(&"dynamic_widgets".to_owned())
            {
                Err(format!("In the `refresh()` method, \
                             no technical database has been created for the project."))?
            } else {
                let collection: Collection = db.collection("dynamic_widgets");
                let filter: Document = mongodb::bson::doc! {
                    "database": &meta.database_name,
                    "collection": &meta.collection_name
                };
                // Check if there is a document in the database for 
                // storing the values of dynamic widgets of model.
                if collection.count_documents(filter.clone(), None)? == 0_i64 {
                    // Init new document.
                    let mut new_doc: Document = mongodb::bson::doc! {
                        "database": &meta.database_name,
                        "collection": &meta.collection_name,
                        "fields": {}
                    };
                    // Add empty arrays to the new document.
                    let mut fields_doc:  Document = mongodb::bson::document::Document::new();
                    for (field, widget) in map_widget_type.clone() {
                        if widget.contains("Dyn") {
                            fields_doc.insert(field, mongodb::bson::Bson::Array(Vec::new()));
                        }
                    }
                    // Insert new document.
                    new_doc.insert("fields".to_string(), fields_doc);
                    collection.insert_one(new_doc, None)?;
                } else {
                    // Get an existing document.
                    let mut exist_doc = collection.find_one(filter.clone(), None)?.unwrap();
                    // Get a document with `dynamic_widgets` fields.
                    let fields_doc = exist_doc.get_document_mut("fields")?;
                    // Get a list of fields from the technical database,
                    // from the `dynamic_widgets` collection for current Model.
                    let dyn_fields_from_db: Vec<String> = fields_doc.keys().map(|item| item.into()).collect();
                    // Create an empty list for fields with dynamic widget types.
                    let mut dyn_fields_from_model: Vec<String> = Vec::new();
                    // Add new (if any) fields in `fields_doc`.
                    for (field, widget) in trunc_map_widget_type.clone() {
                        if widget.contains("Dyn") {
                            dyn_fields_from_model.push(field.clone());
                            // If the new field or widgets do not match,
                            // initialize with an empty array.
                            if !dyn_fields_from_db.contains(&field) || 
                                (widget != *monitor_map_widget_type.get(field.as_str()).unwrap_or(&String::new())) {
                                fields_doc.insert(field, mongodb::bson::Bson::Array(Vec::new()));
                            }
                        }
                    }
                    // Remove orphaned fields.
                    for field in dyn_fields_from_db {
                        if !dyn_fields_from_model.contains(&field) {
                            fields_doc.remove(&field).unwrap();
                        }
                    }
                    // Full update existing document.
                    collection.update_one(filter, exist_doc, None)?;
                }
            }
        }

        // Unlock.
        drop(client_store);
        // Run reorganize databases state.
        self.napalm()?;
        //
        Ok(())
    }
}
