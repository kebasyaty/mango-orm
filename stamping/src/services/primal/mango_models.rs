use async_trait::async_trait;
use futures::stream::StreamExt;
use mango_orm::{
    create_model,
    forms::Form,
    models::Meta,
    widgets::{FieldType, Transport, Widget},
};
use mongodb::{
    bson::{doc, document::Document, ser::to_document, Bson},
    options::UpdateModifications,
    Client, Collection, Cursor, Database,
};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

const SERVICE_NAME: &'static str = "test_service"; // SERVICE_NAME or APP_NAME or PROJECT_NAME etc...
const DATABASE_NAME: &'static str = "dev_db"; // SERVICE_NAME or APP_NAME or PROJECT_NAME etc...

// User --------------------------------------------------------------------------------------------
create_model! {
    SERVICE_NAME,
    DATABASE_NAME,
    struct User {
        username: String,
        email: String
    }
}
#[async_trait]
impl Form for User {
    // Example:
    // Customizing widgets by model fields
    fn widgets() -> HashMap<&'static str, Widget> {
        let mut raw_attrs = HashMap::new();
        raw_attrs.insert(
            "username",
            Widget {
                label: "Your Name:".to_string(),
                value: FieldType::InputText("Rust".to_string()),
                maxlength: 40,
                required: true,
                hint: "Please enter your real name.".to_string(),
                other_attrs: format!("placeholder=\"{}\"", "Your Name"),
                ..Default::default()
            },
        );
        raw_attrs.insert(
            "email",
            Widget {
                label: "Your Email:".to_string(),
                value: FieldType::InputEmail(String::new()),
                maxlength: 78,
                required: true,
                hint: "Enter your work email.".to_string(),
                unique: true,
                other_attrs: format!("placeholder=\"{}\"", "Your Email"),
                ..Default::default()
            },
        );
        raw_attrs
    }
}
