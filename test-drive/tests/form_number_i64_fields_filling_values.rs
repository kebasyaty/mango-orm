use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_7zzbT7QukN_TRa5h";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxI64", default = 0, unique = true)]
        pub checkbox: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "radioI64", default = 1)]
        pub radio: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "numberI64")]
        pub number: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeI64", default = 5, min = 1, max = 12)]
        pub range: Option<i64>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_with_filling_values() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        checkbox: Some(12_i64),
        radio: Some(20_i64),
        number: Some(105_i64),
        range: Some(9_i64),
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
    assert_eq!(
        12_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    // radio
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        20_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result.wig();
    assert_eq!(
        105_i64,
        map_wigets.get("number").unwrap().value.parse::<i64>()?
    );
    // range
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        9_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );

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
    assert_eq!(
        12_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        0_i64,
        map_wigets.get("checkbox").unwrap().value.parse::<i64>()?
    );
    // radio
    let map_wigets = result.wig();
    assert_eq!(
        20_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        1_i64,
        map_wigets.get("radio").unwrap().value.parse::<i64>()?
    );
    // number
    let map_wigets = result.wig();
    assert_eq!(
        105_i64,
        map_wigets.get("number").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let map_wigets = result.wig();
    assert_eq!(
        9_i64,
        map_wigets.get("range").unwrap().value.parse::<i64>()?
    );
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
