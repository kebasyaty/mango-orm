use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "b2vp9Dv5gVJ2HkU_";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Forms
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        // text
        #[serde(default)]
        #[field_attrs(
            widget = "selectText",
            value = "volvo",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectTextMult",
            options = r#"[
                ["volvo","Volvo"],
                ["saab","Saab"],
                ["mercedes","Mercedes"],
                ["audi","Audi"]
            ]"#
        )]
        pub select_text_mult: Option<Vec<String>>,
        // i32
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32: Option<i32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI32Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i32_mult: Option<Vec<i32>>,
        // u32
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32: Option<u32>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectU32Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_u32_mult: Option<Vec<u32>>,
        // i64
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64",
            value = 1,
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
        pub select_i64: Option<i64>,
        #[serde(default)]
        #[field_attrs(
            widget = "selectI64Mult",
            options = r#"[
                [1,"Volvo"],
                [2,"Saab"],
                [3,"Mercedes"],
                [4,"Audi"]
            ]"#
        )]
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
}

// TEST
// #################################################################################################
#[test]
fn test_form_select_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        // text
        select_text: Some("audi".to_string()),
        select_text_mult: Some(vec!["saab".to_string(), "audi".to_string()]),
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
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_text_mult").unwrap().value,
        r#"["saab","audi"]"#
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_u32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i64_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
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
    // select_text
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("audi", map_wigets.get("select_text").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("volvo", map_wigets.get("select_text").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text").unwrap().options
    );
    // select_text_mult
    let map_wigets = result.wig();
    assert_eq!(
        map_wigets.get("select_text_mult").unwrap().value,
        r#"["saab","audi"]"#
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_text_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["volvo","Volvo"],["saab","Saab"],["mercedes","Mercedes"],["audi","Audi"]]"#
        )?,
        map_wigets.get("select_text_mult").unwrap().options
    );
    // select_i32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i32").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32").unwrap().options
    );
    // select_i32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_i32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i32_mult").unwrap().options
    );
    // select_u32
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_u32").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_u32").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32").unwrap().options
    );
    // select_u32_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_u32_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_u32_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_u32_mult").unwrap().options
    );
    // select_i64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4", map_wigets.get("select_i64").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("1", map_wigets.get("select_i64").unwrap().value);
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64").unwrap().options
    );
    // select_i64_mult
    let map_wigets = result.wig();
    assert_eq!(map_wigets.get("select_i64_mult").unwrap().value, "[2,4]");
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_i64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1","Volvo"],["2","Saab"],["3","Mercedes"],["4","Audi"]]"#
        )?,
        map_wigets.get("select_i64_mult").unwrap().options
    );
    // select_f64
    // ---------------------------------------------------------------------------------------------
    let map_wigets = result.wig();
    assert_eq!("4.4", map_wigets.get("select_f64").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
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
    let map_wigets = app_name::TestForm::form_wig()?;
    assert!(map_wigets.get("select_f64_mult").unwrap().value.is_empty());
    assert_eq!(
        serde_json::from_str::<Vec<(String, String)>>(
            r#"[["1.1","Volvo"],["2.2","Saab"],["3.3","Mercedes"],["4.4","Audi"]]"#
        )?,
        map_wigets.get("select_f64_mult").unwrap().options
    );

    // Validating cache
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
