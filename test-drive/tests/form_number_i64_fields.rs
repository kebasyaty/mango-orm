use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "7zzbT7QukN_TRa5h";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "radioI64", value = 1)]
        pub radio: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "numberI64")]
        pub number: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeI64", value = 5, min = 1, max = 12)]
        pub range: Option<i64>,
        #[serde(default)]
        #[field_attrs(widget = "hiddenI64", value = 3, min = 1, max = 12)]
        pub hidden: Option<i64>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_number_i64_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        radio: Some(20_i64),
        number: Some(105_i64),
        range: Some(9_i64),
        hidden: Some(11_i64),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
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
    // hidden
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        11_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
    );

    // Validating cache
    {
        let _form_store = FORM_STORE.read()?;
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
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
    // hidden
    let map_wigets = result.wig();
    assert_eq!(
        11_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_i64,
        map_wigets.get("hidden").unwrap().value.parse::<i64>()?
    );

    // Validating cache
    {
        let _form_store = FORM_STORE.read()?;
    }

    Ok(())
}
