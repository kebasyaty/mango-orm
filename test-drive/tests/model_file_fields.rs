use mango_orm::*;
use mango_orm::{forms::FileData, migration::Monitor, test_tool::del_test_db};
use metamorphose::Model;
use mongodb::{
    bson::{de::from_document, doc, oid::ObjectId},
    sync::Client,
};
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const PROJECT_NAME: &str = "project_name";
    pub const UNIQUE_PROJECT_KEY: &str = "qhYR6caApSj5Ph1";
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
            r#"{"path":"./media/hello_world_2.odt","url":"/media/hello_world_2.odt","is_delete":false}"#.to_string(),
        ),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let file_data = FileData {
        path: "./media/hello_world_2.odt".to_string(),
        url: "/media/hello_world_2.odt".to_string(),
        name: "hello_world_2.odt".to_string(),
        size: 9989_u32,
    };
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validating values in widgets
    // file
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("file").unwrap().value,
        serde_json::to_string(&file_data)?
    );
    /*
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/hello_world.odt","url":"/media/hello_world.odt"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("file").unwrap().value.as_str()
        )?
    );
    */

    // Validating values in database
    {
        let form_store = FORM_STORE.read()?;
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let form_cache: &FormCache = form_store.get(&app_name::TestModel::key()[..]).unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("file"));
        assert_eq!(
            file_data,
            from_document::<FileData>(doc.get_document("file")?.clone())?
        );
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(None, None)?;
    // Validating update
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // Validating values
    // file
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("file").unwrap().value,
        serde_json::to_string(&file_data)?
    );
    /*
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            r#"{"path":"./media/hello_world.odt","url":"/media/hello_world.odt"}"#
        )?,
        serde_json::from_str::<std::collections::HashMap<String, String>>(
            map_wigets.get("file").unwrap().value.as_str()
        )?
    );
    */

    // Validating values in database
    {
        let form_store = FORM_STORE.read()?;
        let client_store = MONGODB_CLIENT_STORE.read()?;
        let form_cache: &FormCache = form_store.get(&app_name::TestModel::key()[..]).unwrap();
        let meta: &Meta = &form_cache.meta;
        let client: &Client = client_store.get(meta.db_client_name.as_str()).unwrap();
        let object_id = ObjectId::with_string(test_model.hash.clone().unwrap().as_str())?;
        let coll = client
            .database(meta.database_name.as_str())
            .collection(meta.collection_name.as_str());
        let filter = doc! {"_id": object_id};
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        let doc = coll.find_one(filter, None)?.unwrap();
        assert!(!doc.is_null("file"));
        assert_eq!(
            file_data,
            from_document::<FileData>(doc.get_document("file")?.clone())?
        );
    }

    // ---------------------------------------------------------------------------------------------
    del_test_db(
        app_name::PROJECT_NAME,
        app_name::UNIQUE_PROJECT_KEY,
        &app_name::model_list()?,
    )?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------
    Ok(())
}
