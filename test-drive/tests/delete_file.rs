use mango_orm::*;
use mango_orm::{migration::Monitor, test_tool::del_test_db};
use metamorphose::Model;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::path::Path;

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const PROJECT_NAME: &str = "project_name";
    pub const UNIQUE_PROJECT_KEY: &str = "gKpqf5VS5gkNxfr";
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        #[serde(default)]
        #[field_attrs(
            widget = "inputFile",
            value = r#"{
                "path":"./media/hello_world.odt",
                "url":"/media/hello_world.odt"
            }"#
        )]
        pub file: Option<String>,
    }

    // Test migration
    // *********************************************************************************************
    // Model list
    pub fn model_list() -> Result<Vec<Meta>, Box<dyn std::error::Error>> {
        Ok(vec![TestModel::meta()?])
    }
    // Test, migration service `Mango`
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients
        MONGODB_CLIENT_STORE.write()?.insert(
            "default".to_string(),
            mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
        );
        // Remove test databases
        // ( Test databases may remain in case of errors )
        del_test_db(PROJECT_NAME, UNIQUE_PROJECT_KEY, &model_list()?)?;
        // Migration
        let monitor = Monitor {
            project_name: PROJECT_NAME,
            unique_project_key: UNIQUE_PROJECT_KEY,
            // Register models
            models: model_list()?,
        };
        monitor.migrat()?;
        // Add metadata and widgects map to cache.
        TestModel::to_cache()?;
        //
        Ok(())
    }
}

// TEST
// #################################################################################################
#[test]
fn test_model_file_fields() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        file: Some(
            r#"{"path":"./media/hello_world_2.odt","url":"/media/hello_world_2.odt"}"#.to_string(),
        ),
        ..Default::default()
    };

    // ---------------------------------------------------------------------------------------------
    let path = Path::new("./media/hello_world_2.odt");
    assert!(path.exists());
    assert!(test_model.save(None, None)?.is_valid());
    assert!(test_model.delete(None)?.is_valid());
    assert!(!path.exists());

    // ---------------------------------------------------------------------------------------------
    del_test_db(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
