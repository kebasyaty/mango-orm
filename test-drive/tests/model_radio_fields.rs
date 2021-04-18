use mango_orm::*;
use mango_orm::{migration::Monitor, test_tool::del_test_db};
use metamorphose::Model;
use mongodb::{
    bson::{doc, oid::ObjectId},
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
    pub const UNIQUE_PROJECT_KEY: &str = "h4xFDD1fTZxvQY3";
    pub const SERVICE_NAME: &str = "service_name";
    pub const DATABASE_NAME: &str = "database_name";
    pub const DB_CLIENT_NAME: &str = "default";
    const DB_QUERY_DOCS_LIMIT: u32 = 1000;

    // Create models
    // *********************************************************************************************
    #[Model]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestModel {
        // text
        #[serde(default)]
        #[field_attrs(
            widget = "radioText",
            value = "volvo",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub radio_text: Option<String>,
        // i32
        #[serde(default)]
        #[field_attrs(
            widget = "radioI32",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_i32: Option<i32>,
        // u32
        #[serde(default)]
        #[field_attrs(
            widget = "radioU32",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_u32: Option<u32>,
        // i64
        #[serde(default)]
        #[field_attrs(
            widget = "radioI64",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub radio_i64: Option<i64>,
        // f64
        #[serde(default)]
        #[field_attrs(
            widget = "radioF64",
            value = 1.1,
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub radio_f64: Option<f64>,
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
fn test_model_radio_fields() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        // text
        radio_text: Some("audi".to_string()),
        // i32
        radio_i32: Some(4),
        // u32
        radio_u32: Some(4),
        // i64
        radio_i64: Some(4),
        // f64
        radio_f64: Some(4.4),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // radio_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("radio_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("volvo", map_wigets.get("radio_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("radio_text").unwrap().options
    );
    // radio_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_i32").unwrap().options
    );
    // radio_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_u32").unwrap().options
    );
    // radio_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_i64").unwrap().options
    );
    // radio_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("radio_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("radio_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("radio_f64").unwrap().options
    );

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
        // text
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_text"));
        assert_eq!("audi", doc.get_str("radio_text")?);
        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_i32"));
        assert_eq!(4, doc.get_i32("radio_i32")?);
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_u32"));
        assert_eq!(4, doc.get_i64("radio_u32")?);
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_i64"));
        assert_eq!(4, doc.get_i64("radio_i64")?);
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_f64"));
        assert_eq!(4.4, doc.get_f64("radio_f64")?);
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let tmp_hash = test_model.hash.clone().unwrap();
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    assert_eq!(tmp_hash, test_model.hash.clone().unwrap());
    // radio_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("radio_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("volvo", map_wigets.get("radio_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("radio_text").unwrap().options
    );
    // radio_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_i32").unwrap().options
    );
    // radio_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_u32").unwrap().options
    );
    // radio_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("radio_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("radio_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("radio_i64").unwrap().options
    );
    // radio_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("radio_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("radio_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("radio_f64").unwrap().options
    );

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
        // text
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_text"));
        assert_eq!("audi", doc.get_str("radio_text")?);
        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_i32"));
        assert_eq!(4, doc.get_i32("radio_i32")?);
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_u32"));
        assert_eq!(4, doc.get_i64("radio_u32")?);
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_i64"));
        assert_eq!(4, doc.get_i64("radio_i64")?);
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("radio_f64"));
        assert_eq!(4.4, doc.get_f64("radio_f64")?);
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
