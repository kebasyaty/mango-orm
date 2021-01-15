use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const SERVICE_NAME: &str = "TEST_8YHVewY_fENu4rfh";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(widget = "checkBoxBool")]
        pub checkbox: Option<bool>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_with_filling_values() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        checkbox: Some(true),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool());
    // checkbox
    let map_wigets = result.wig();
    assert_eq!(true, map_wigets.get("checkbox").unwrap().checked);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(false, map_wigets.get("checkbox").unwrap().checked);

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    // Update
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.bool(), "{}");
    // checkbox
    let map_wigets = result.wig();
    assert_eq!(true, map_wigets.get("checkbox").unwrap().checked);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(false, map_wigets.get("checkbox").unwrap().checked);

    // Validating cache
    {
        let form_store = FORM_CACHE.lock()?;
        let _client_store = DB_MAP_CLIENT_NAMES.lock()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}