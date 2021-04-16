use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "uucTUjTd_2bvQ8Ye";
    pub const SERVICE_NAME: &str = "service_name";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "radioF64", value = 1.0)]
        pub radio: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "numberF64")]
        pub number: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "rangeF64", value = 5.0, min = 1.0, max = 12.0)]
        pub range: Option<f64>,
        #[serde(default)]
        #[field_attrs(widget = "hiddenF64", value = 3.0, min = 1.0, max = 12.0)]
        pub hidden: Option<f64>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_number_f64_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        radio: Some(20_f64),
        number: Some(105_f64),
        range: Some(9_f64),
        hidden: Some(11_f64),
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
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        20_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    // number
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    let map_wigets = result.wig();
    assert_eq!(
        105_f64,
        map_wigets.get("number").unwrap().value.parse::<f64>()?
    );
    // range
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        9_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    // hidden
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
    );
    let map_wigets = result.wig();
    assert_eq!(
        11_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
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
    let result = test_form.check()?;
    let map_wigets = result.wig();
    assert_eq!(
        20_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        1_f64,
        map_wigets.get("radio").unwrap().value.parse::<f64>()?
    );
    // number
    let result = test_form.check()?;
    let map_wigets = result.wig();
    assert_eq!(
        105_f64,
        map_wigets.get("number").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("number").unwrap().value.is_empty());
    // range
    let result = test_form.check()?;
    let map_wigets = result.wig();
    assert_eq!(
        9_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        5_f64,
        map_wigets.get("range").unwrap().value.parse::<f64>()?
    );
    // hidden
    let result = test_form.check()?;
    let map_wigets = result.wig();
    assert_eq!(
        11_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        3_f64,
        map_wigets.get("hidden").unwrap().value.parse::<f64>()?
    );

    // Validating cache
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
