use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "uhweU3qqpkz4D3B";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
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
}

// TEST
// #################################################################################################
#[test]
fn test_form_radio_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
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
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
    // radio_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("radio_text").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1.1", map_wigets.get("radio_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("radio_f64").unwrap().options
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
    // radio_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("radio_text").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1.1", map_wigets.get("radio_f64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("radio_f64").unwrap().options
    );

    // Validating cache
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
