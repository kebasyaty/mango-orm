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
    pub const UNIQUE_PROJECT_KEY: &str = "NN3hhLvf6uNfCAz";
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
            widget = "selectText",
            value = "Volvo",
            options = r#"["Volvo","Saab","Mercedes","Audi"]"#
        )]
        pub select_text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextMult",
            options = r#"["Volvo","Saab","Mercedes","Audi"]"#
        )]
        pub select_text_mult: Option<Vec<String>>,
        // i32
        #[serde(default)]
        #[field_attrs(widget = "selectI32", value = 1, options = "[1,2,3,4]")]
        pub select_i32: Option<i32>,
        #[serde(default)]
        #[field_attrs(widget = "selectI32Mult", options = "[1,2,3,4]")]
        pub select_i32_mult: Option<Vec<i32>>,
        // u32
        #[serde(default)]
        #[field_attrs(widget = "selectU32", value = 1, options = "[1,2,3,4]")]
        pub select_u32: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "selectU32Mult", options = "[1,2,3,4]")]
        pub select_u32_mult: Option<Vec<u32>>,
        // i64
        #[serde(default)]
        #[field_attrs(widget = "selectI64", value = 1, options = "[1,2,3,4]")]
        pub select_i64: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "selectI64Mult", options = "[1,2,3,4]")]
        pub select_i64_mult: Option<Vec<i64>>,
        // f64
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64",
            value = 1.1,
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64: Option<f64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectF64Mult",
            options = r#"[
                [1.1,"Volvo"],
                [2.2,"Saab"],
                [3.3,"Mercedes"],
                [4.4,"Audi"]
            ]"#
        )]
        pub select_f64_mult: Option<Vec<f64>>,
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
fn test_model_select_fields() -> Result<(), Box<dyn std::error::Error>> {
    // ---------------------------------------------------------------------------------------------
    app_name::mango_migration()?;
    // ^ ^ ^ ---------------------------------------------------------------------------------------

    let mut test_model = app_name::TestModel {
        // text
        select_text: Some("Audi".to_string()),
        select_text_mult: Some(vec!["Saab".to_string(), "Audi".to_string()]),

        // i32
        select_i32: Some(4),
        select_i32_mult: Some(vec![2, 4]),

        // u32
        select_u32: Some(4),
        select_u32_mult: Some(vec![2, 4]),

        // i64
        select_i64: Some(4),
        select_i64_mult: Some(vec![2, 4]),

        // f64
        select_f64: Some(4.4),
        select_f64_mult: Some(vec![2.2, 4.4]),

        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_model.save(None, None)?;
    // Validating create
    assert!(result.is_valid(), "{}", result.hash()?);
    // Validation of `hash`
    assert!(test_model.hash.is_some());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("Audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("Volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["Volvo","Volvo"],["Saab","Saab"],["Mercedes","Mercedes"],["Audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_text_mult").unwrap().value,
        r#"["Saab","Audi"]"#
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["Volvo","Volvo"],["Saab","Saab"],["Mercedes","Mercedes"],["Audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_u32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i64_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );

    // select_f64_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_f64_mult").unwrap().value,
        "[2.2,4.4]"
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
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
        assert!(!doc.is_null("select_text"));
        assert_eq!("Audi", doc.get_str("select_text")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("Saab".to_string()),
                mongodb::bson::Bson::String("Audi".to_string())
            ],
            doc.get_array("select_text_mult")?
        );
        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i32"));
        assert_eq!(4, doc.get_i32("select_i32")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult")?
        );
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_u32"));
        assert_eq!(4, doc.get_i64("select_u32")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult")?
        );
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i64"));
        assert_eq!(4, doc.get_i64("select_i64")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult")?
        );
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_f64"));
        assert_eq!(4.4, doc.get_f64("select_f64")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult")?
        );
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
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("Audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("Volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["Volvo","Volvo"],["Saab","Saab"],["Mercedes","Mercedes"],["Audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_text_mult").unwrap().value,
        r#"["Saab","Audi"]"#
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["Volvo","Volvo"],["Saab","Saab"],["Mercedes","Mercedes"],["Audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_u32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i64_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","1"],["2","2"],["3","3"],["4","4"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestModel::form_wig()?;
    assert_eq!("1.1", map_wigets.get("select_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64").unwrap().options
    );
    // select_f64_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_f64_mult").unwrap().value,
        "[2.2,4.4]"
    );
    let map_wigets = app_name::TestModel::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
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
        assert!(!doc.is_null("select_text"));
        assert_eq!("Audi", doc.get_str("select_text")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::String("Saab".to_string()),
                mongodb::bson::Bson::String("Audi".to_string())
            ],
            doc.get_array("select_text_mult")?
        );
        // i32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i32"));
        assert_eq!(4, doc.get_i32("select_i32")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int32(2), mongodb::bson::Bson::Int32(4)],
            doc.get_array("select_i32_mult")?
        );
        // u32
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_u32"));
        assert_eq!(4, doc.get_i64("select_u32")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_u32_mult")?
        );
        // i64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_i64"));
        assert_eq!(4, doc.get_i64("select_i64")?);
        assert_eq!(
            &vec![mongodb::bson::Bson::Int64(2), mongodb::bson::Bson::Int64(4)],
            doc.get_array("select_i64_mult")?
        );
        // f64
        // -----------------------------------------------------------------------------------------
        assert!(!doc.is_null("select_f64"));
        assert_eq!(4.4, doc.get_f64("select_f64")?);
        assert_eq!(
            &vec![
                mongodb::bson::Bson::Double(2.2),
                mongodb::bson::Bson::Double(4.4)
            ],
            doc.get_array("select_f64_mult")?
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
