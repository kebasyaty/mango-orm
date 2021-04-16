use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "2YxrJbPqvX_g2L4A";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "radioU32", value = 1)]
        pub radio: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "numberU32")]
        pub number: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "rangeU32", value = 5, min = 1, max = 12)]
        pub range: Option<u32>,
        #[serde(default)]
        #[field_attrs(widget = "hiddenU32", value = 3, min = 1, max = 12)]
        pub hidden: Option<u32>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_number_u32_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        radio: Some(20_u32),
        number: Some(105_u32),
        range: Some(9_u32),
        hidden: Some(11_u32),
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
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
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
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
