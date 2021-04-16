use mango_orm::*;
use metamorphose::{Form, Model};
use serde::{Deserialize, Serialize};

// Get settings of service/sub-application.
use crate::settings::{
    default::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
    PROJECT_NAME, UNIQUE_PROJECT_KEY,
};

#[Model]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dynamic {
    // text
    #[serde(default)]
    #[field_attrs(widget = "selectTextDyn")]
    pub select_text_dyn: Option<String>,
    #[serde(default)]
    #[field_attrs(widget = "selectTextMultDyn")]
    pub select_text_mult_dyn: Option<Vec<String>>,
    // i32
    #[serde(default)]
    #[field_attrs(widget = "selectI32Dyn")]
    pub select_i32_dyn: Option<i32>,
    #[serde(default)]
    #[field_attrs(widget = "selectI32MultDyn")]
    pub select_i32_mult_dyn: Option<Vec<i32>>,
    // u32
    #[serde(default)]
    #[field_attrs(widget = "selectU32Dyn")]
    pub select_u32_dyn: Option<u32>,
    #[serde(default)]
    #[field_attrs(widget = "selectU32MultDyn")]
    pub select_u32_mult_dyn: Option<Vec<u32>>,
    // i64
    #[serde(default)]
    #[field_attrs(widget = "selectI64Dyn")]
    pub select_i64_dyn: Option<i64>,
    #[serde(default)]
    #[field_attrs(widget = "selectI64MultDyn")]
    pub select_i64_mult_dyn: Option<Vec<i64>>,
    // f64
    #[serde(default)]
    #[field_attrs(widget = "selectF64Dyn")]
    pub select_f64_dyn: Option<f64>,
    #[serde(default)]
    #[field_attrs(widget = "selectF64MultDyn")]
    pub select_f64_mult_dyn: Option<Vec<f64>>,
}

#[Model(is_use_add_valid = true, ignore_fields = "confirm_password")]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct User {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "Username",
        placeholder = "Enter your username",
        unique = true,
        required = true,
        maxlength = 150,
        hint = "Valid characters: a-z A-Z 0-9 _ @ + .<br>Max size: 150"
    )]
    pub username: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "First name",
        placeholder = "Enter your First name",
        maxlength = 150
    )]
    pub first_name: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        label = "Last name",
        placeholder = "Enter your Last name",
        maxlength = 150
    )]
    pub last_name: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputEmail",
        label = "E-mail",
        placeholder = "Please enter your email",
        required = true,
        unique = true,
        maxlength = 74,
        hint = "Your actual E-mail"
    )]
    pub email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputPassword",
        label = "Password",
        placeholder = "Enter your password",
        required = true,
        minlength = 8,
        hint = "Valid characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br>Min size: 8"
    )]
    pub password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputPassword",
        label = "Confirm password",
        placeholder = "Repeat your password",
        hint = "Repeat your password"
    )]
    pub confirm_password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is staff?",
        hint = "User can access the admin site?"
    )]
    pub is_staff: Option<bool>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is active?",
        hint = "Is this an active account?"
    )]
    pub is_active: Option<bool>,
}

impl AdditionalValidation for User {
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();

        // Get clean data
        let password = self.password.clone().unwrap_or_default();
        let confirm_password = self.confirm_password.clone().unwrap_or_default();
        let username = self.username.clone().unwrap_or_default();

        // Fields validation
        if password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        if !regex::RegexBuilder::new(r"^[a-z\d_@+.]+$")
            .case_insensitive(true)
            .build()
            .unwrap()
            .is_match(username.as_str())
        {
            error_map.insert(
                "username",
                "Invalid characters present.<br>\
                    Valid characters: a-z A-Z 0-9 _ @ + .",
            );
        }

        Ok(error_map)
    }
}

#[Model(
    is_use_add_valid = true,
    db_client_name = "default_2",
    ignore_fields = "confirm_email, confirm_password"
)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserProfile {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        value = "Some text",
        required = false,
        minlength = 3,
        maxlength = 40
    )]
    pub username: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, unique = true, maxlength = 74)]
    pub email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, maxlength = 74)]
    pub confirm_email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub confirm_password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputDate")]
    pub date: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputDateTime")]
    pub datetime: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "numberI32")]
    pub num_i32: Option<i32>,
    //
    #[serde(default)]
    #[field_attrs(widget = "numberU32")]
    pub num_u32: Option<u32>,
    //
    #[serde(default)]
    #[field_attrs(widget = "numberI64")]
    pub num_i64: Option<i64>,
    //
    #[serde(default)]
    #[field_attrs(widget = "numberF64")]
    pub num_f64: Option<f64>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputFile",
        value = r#"{
            "path":"./test-drive/media/hello_world.odt",
            "url":"/media/hello_world.odt"
        }"#
    )]
    pub file: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "inputImage",
        value = r#"{
            "path":"./test-drive/media/no-image-found.png",
            "url":"/media/no-image-found.png"
        }"#
    )]
    pub image: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is staff?",
        hint = "User can access the admin site?"
    )]
    pub is_staff: Option<bool>,
    //
    #[serde(default)]
    #[field_attrs(
        widget = "checkBox",
        label = "is active?",
        hint = "Is this an active account?"
    )]
    pub is_active: Option<bool>,
    //
    #[serde(default)]
    #[field_attrs(widget = "selectTextMult")]
    pub select_text_mult: Option<Vec<String>>,
    //
    #[serde(default)]
    #[field_attrs(widget = "selectI32Mult")]
    pub select_i32_mult: Option<Vec<i32>>,
    //
    #[serde(default)]
    #[field_attrs(widget = "selectU32Mult")]
    pub select_u32_mult: Option<Vec<u32>>,
    //
    #[serde(default)]
    #[field_attrs(widget = "selectI64Mult")]
    pub select_i64_mult: Option<Vec<i64>>,
    //
    #[serde(default)]
    #[field_attrs(widget = "selectF64Mult")]
    pub select_f64_mult: Option<Vec<f64>>,
}

// Methods for additional validation.
// Hint: For custom use, add the Model attribute `is_use_add_valid = true`.
// Hint: Remember to use for validate of ignored fields.
impl AdditionalValidation for UserProfile {
    // Example of additional validation for ignored fields
    // ---------------------------------------------------------------------------------------------
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // Hint: error_map.insert("field_name", "Error message.")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        // Get clean data
        let email = self.email.clone().unwrap_or_default();
        let confirm_email = self.confirm_email.clone().unwrap_or_default();
        let password = self.password.clone().unwrap_or_default();
        let confirm_password = self.confirm_password.clone().unwrap_or_default();
        // Fields validation
        if email != confirm_email {
            error_map.insert(
                "confirm_email",
                "Email address confirmation does not match.",
            );
        }
        if password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        Ok(error_map)
    }
}

#[Form(is_use_add_valid = true)]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserForm {
    #[serde(default)]
    #[field_attrs(
        widget = "inputText",
        value = "Some text",
        required = false,
        minlength = 3,
        maxlength = 40
    )]
    pub username: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, unique = true, maxlength = 74)]
    pub email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputEmail", required = true, maxlength = 74)]
    pub confirm_email: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub password: Option<String>,
    //
    #[serde(default)]
    #[field_attrs(widget = "inputPassword", required = true, minlength = 8)]
    pub confirm_password: Option<String>,
}

// Methods for additional validation.
// Hint: For custom use, add the Model attribute `is_use_add_valid = true`.
impl AdditionalValidation for UserForm {
    // Example of additional validation for ignored fields
    // ---------------------------------------------------------------------------------------------
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // error_map.insert("field_name", "Error message.")
        let mut error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        // Get clean data
        let email = self.email.clone().unwrap_or_default();
        let confirm_email = self.confirm_email.clone().unwrap_or_default();
        let password = self.password.clone().unwrap_or_default();
        let confirm_password = self.confirm_password.clone().unwrap_or_default();
        // Fields validation
        if email != confirm_email {
            error_map.insert(
                "confirm_email",
                "Email address confirmation does not match.",
            );
        }
        if password != confirm_password {
            error_map.insert("confirm_password", "Password confirmation does not match.");
        }
        Ok(error_map)
    }
}
