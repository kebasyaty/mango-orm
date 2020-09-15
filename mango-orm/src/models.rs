//! # Models
//!
//! `Meta` - Metadata of models.
//! `Model` - Defining common behavior of models.
//! `Monitor` - Creation and updating of a technical database for monitoring the state of models.

use crate::widgets::{Transport, Widget};
use async_trait::async_trait;
use mongodb::Client;
use std::collections::HashMap;

// MODELS ==========================================================================================
/// Metadata
#[derive(Debug)]
pub struct Meta {
    pub database: String,
    pub collection: String,
}
// Model -------------------------------------------------------------------------------------------
/// Defining common behavior of models
#[async_trait]
pub trait Model {
    // Get raw attributes for further processing
    fn raw_attrs() -> HashMap<&'static str, Widget>;
    // Get pure attributes for a page templating engine
    fn form_attrs() -> HashMap<String, Transport> {
        let raw_attrs = Self::raw_attrs();
        let mut clean_attrs = HashMap::new();
        for (field, widget) in &raw_attrs {
            clean_attrs.insert(field.to_string(), widget.get_clean_attrs(field));
        }
        clean_attrs
    }
}
// For Migration -----------------------------------------------------------------------------------
/// Creation and updating of a technical database for monitoring the state of models
pub struct Monitor<'a> {
    pub keyword: &'a str,
    pub client: &'a Client,
}
impl<'a> Monitor<'a> {
    // Refresh models state
    pub async fn refresh(&self) {
        let db_name = format!("mango_{}", self.keyword);
        let database_names: Vec<String> =
            self.client.list_database_names(None, None).await.unwrap();
        if !database_names.contains(&db_name) {
            self.client
                .database(&db_name)
                .create_collection("models", None)
                .await
                .unwrap();
        }
    }
    // Reorganize databases state
    pub async fn napalm(&self) {
        println!("{}", self.keyword);
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
