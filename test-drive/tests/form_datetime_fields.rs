use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "PSug_V7hs7XU3ZUq";
    pub const SERVICE_NAME: &str = "service_name";

    // Create form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(
            widget = "inputDateTime",
            value = "1970-02-28T00:00",
            min = "1970-01-01T00:00",
            max = "1970-03-01T00:00",
            unique = true
        )]
        pub date: Option<String>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_datetime_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        date: Some("1970-02-27T00:00".to_string()),
        ..Default::default()
    };

    // Create
    // ---------------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
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
    // date
    let map_wigets = result.wig();
    assert_eq!(
        "1970-02-27T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "1970-02-28T00:00".to_string(),
        map_wigets.get("date").unwrap().value
    );

    // Validating cache
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
