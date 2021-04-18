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
    pub const UNIQUE_PROJECT_KEY: &str = "2GhzTpa5HyVsEwL";
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
        #[field_attrs(widget = "radioF64", value = 1.0, unique = true)]
        pub radio: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "numberF64", unique = true)]
        pub number: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeF64", value = 5.0, min = 1.0, max = 12.0, unique = true)]
        pub range: Option<f64>,
        #[serde(default)]
        #[field_attrs(
            widget = "hiddenF64",
            value = 3.0,
            min = 1.0,
            max = 12.0,
            unique = true
        )]
        pub hidden: Option<f64>,
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
fn test_model_number_f64_fields() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        radio: Some(20_f64),
        number: Some(105_f64),
        range: Some(9_f64),
        hidden: Some(11_f64),
        ..Default::default()
    };
    let mut test_model_2 = app_name::TestModel {
        radio: Some(20_f64),
        number: Some(105_f64),
        range: Some(9_f64),
        hidden: Some(11_f64),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None)?;
    let result_2 = test_model_2.save(None, None)?;
    // Validating create
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // Validation of `unique`
    assert!(!result_2.is_valid());
    // Validation of `hash`
    assert!(test_model_2.hash.is_none());
    // Validating values in widgets
    // radio
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        20_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    // number
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result_2.wig();
    assert_eq!(
        105_f64,
        map_wigets.get("number").unwrap().value.parse::<f64>()?
    );
    // range
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        9_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    // hidden
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result_2.wig();
    assert_eq!(
        11_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
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
        let doc = coll.find_one(filter, None)?.unwrap();
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        assert_eq!(20_f64, doc.get_f64("radio")?);
        assert_eq!(105_f64, doc.get_f64("number")?);
        assert_eq!(9_f64, doc.get_f64("range")?);
        assert_eq!(11_f64, doc.get_f64("hidden")?);
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
    // radio
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        20_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    // number
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        105_f64,
        map_wigets.get("number").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        9_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    // hidden
    let result = test_model.save(None, None)?;
    let map_wigets = result.wig();
    assert_eq!(
        11_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!(
        3_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
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
        let doc = coll.find_one(filter, None)?.unwrap();
        assert_eq!(1_i64, coll.count_documents(None, None)?);
        assert_eq!(20_f64, doc.get_f64("radio")?);
        assert_eq!(105_f64, doc.get_f64("number")?);
        assert_eq!(9_f64, doc.get_f64("range")?);
        assert_eq!(11_f64, doc.get_f64("hidden")?);
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
