use mango_orm::*;
use metamorphose::Form;
use serde::{Deserialize, Serialize};

// APP NAME
// #################################################################################################
mod app_name {
    use super::*;

    // Test application settings
    // *********************************************************************************************
    pub const UNIQUE_PROJECT_KEY: &str = "ZJ2KWgH_jPasnXY9";
    pub const SERVICE_NAME: &str = "service_name";

    // Create Form
    // *********************************************************************************************
    #[Form]
    #[derive(Serialize, Deserialize, Default)]
    pub struct TestForm {
        #[serde(default)]
        #[field_attrs(
            widget = "inputText",
            value = "Lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub text: Option<String>,
        #[serde(default)]
        #[field_attrs(
            widget = "hiddenText",
            value = "Lorem ipsum",
            minlength = 2,
            maxlength = 60,
            unique = true
        )]
        pub hidden: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "radioText", value = "Lorem ipsum")]
        pub radio: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputColor", value = "#000000")]
        pub color: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputEmail", maxlength = 74)]
        pub email: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPassword", minlength = 8)]
        pub password: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputPhone")]
        pub phone: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputUrl")]
        pub url: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIP", value = "127.0.0.1")]
        pub ip: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv4", value = "127.0.0.1")]
        pub ipv4: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "inputIPv6", value = "::ffff:7f00:1")]
        pub ipv6: Option<String>,
        #[serde(default)]
        #[field_attrs(widget = "textArea", value = "Lorem ipsum")]
        pub textarea: Option<String>,
    }
}

// TEST
// #################################################################################################
#[test]
fn test_form_text_fields() -> Result<(), Box<dyn std::error::Error>> {
    let test_form = app_name::TestForm {
        text: Some("Lorem ipsum dolor sit amet".to_string()),
        hidden: Some("Lorem ipsum dolor sit amet".to_string()),
        radio: Some("Lorem ipsum dolor sit amet".to_string()),
        color: Some("#ffffff".to_string()),
        email: Some("no_reply@email.net".to_string()),
        password: Some("12345678".to_string()),
        phone: Some("+00000000000".to_string()),
        url: Some("https://www.google.com/".to_string()),
        ip: Some("172.217.14.196".to_string()),
        ipv4: Some("172.217.14.196".to_string()),
        ipv6: Some("::ffff:acd9:ec4".to_string()),
        textarea: Some("Lorem ipsum dolor sit amet".to_string()),
        ..Default::default()
    };

    // Create
    // -----------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
    // text
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // hidden
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    // radio
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "#ffffff".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(
        "no_reply@email.net".to_string(),
        map_wigets.get("email").unwrap().value
    );
    // password
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(
        "+00000000000".to_string(),
        map_wigets.get("phone").unwrap().value
    );
    // url
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(
        "https://www.google.com/".to_string(),
        map_wigets.get("url").unwrap().value
    );
    // ip
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ip").unwrap().value
    );
    // ipv4
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "::ffff:acd9:ec4".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("textarea").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    // Update
    // -----------------------------------------------------------------------------------------
    let result = test_form.check()?;
    // Validating
    assert!(result.is_valid());
    // text
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("text").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("text").unwrap().value
    );
    // hidden
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("hidden").unwrap().value
    );
    // radio
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("radio").unwrap().value
    );
    // color
    let map_wigets = result.wig();
    assert_eq!(
        "#ffffff".to_string(),
        map_wigets.get("color").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "#000000".to_string(),
        map_wigets.get("color").unwrap().value
    );
    // email
    let map_wigets = result.wig();
    assert_eq!(
        "no_reply@email.net".to_string(),
        map_wigets.get("email").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("email").unwrap().value);
    // password
    let map_wigets = result.wig();
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("password").unwrap().value);
    // phone
    let map_wigets = result.wig();
    assert_eq!(
        "+00000000000".to_string(),
        map_wigets.get("phone").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("phone").unwrap().value);
    // url
    let map_wigets = result.wig();
    assert_eq!(
        "https://www.google.com/".to_string(),
        map_wigets.get("url").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(String::new(), map_wigets.get("url").unwrap().value);
    // ip
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ip").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!("127.0.0.1".to_string(), map_wigets.get("ip").unwrap().value);
    // ipv4
    let map_wigets = result.wig();
    assert_eq!(
        "172.217.14.196".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "127.0.0.1".to_string(),
        map_wigets.get("ipv4").unwrap().value
    );
    // ipv6
    let map_wigets = result.wig();
    assert_eq!(
        "::ffff:acd9:ec4".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "::ffff:7f00:1".to_string(),
        map_wigets.get("ipv6").unwrap().value
    );
    // textarea
    let map_wigets = result.wig();
    assert_eq!(
        "Lorem ipsum dolor sit amet".to_string(),
        map_wigets.get("textarea").unwrap().value
    );
    let map_wigets = app_name::TestForm::form_wig()?;
    assert_eq!(
        "Lorem ipsum".to_string(),
        map_wigets.get("textarea").unwrap().value
    );

    // Validating values in database
    {
        let form_store = FORM_STORE.read()?;
        let _form_cache: &FormCache = form_store.get(&app_name::TestForm::key()[..]).unwrap();
    }

    Ok(())
}
