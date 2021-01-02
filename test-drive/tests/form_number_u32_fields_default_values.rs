use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_zXw4gvdY_1sp9j3Q";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxU32", default = 0, unique = true)]
        pub checkbox: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "radioU32", default = 1)]
        pub radio: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "numberU32")]
        pub number: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "rangeU32", default = 5, min = 1, max = 12)]
        pub range: Option<u32>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_with_default_values() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // checkbox
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        0_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("checkbox").unwrap().value.is_empty());
    // radio
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    // number
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::form_key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // checkbox
    let map_wigets = result.wig();
    assert!(map_wigets.get("checkbox").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        0_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    // radio
    let map_wigets = result.wig();
    assert!(map_wigets.get("radio").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let map_wigets = result.wig();
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = result.wig();
    assert!(map_wigets.get("range").unwrap().value.is_empty());
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::form_key()[..]).unwrap();
    }

    Ok(())
}
