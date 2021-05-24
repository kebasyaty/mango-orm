![Logo](https://github.com/kebasyaty/mango-orm/raw/master/metamorphose/images/logo.svg)

# metamorphose

### Macros collection for converting Structure to Model, for a [mango-orm](https://github.com/kebasyaty/mango-orm "mango-orm") project.

## Macros
#### Model
> Macro for converting Structure to mango-orm Model.
> The model can access the database.
> The model can create, update, and delete documents in collections.

#### Form
> Macro for converting Structure to mango-orm Form.
> The form does not have access to the database.
> Form are needed where it makes no sense to use a model -
> To create a search form, to recover a password, to combine models, etc.

## Requirements
- quote
- regex
- serde_json
- syn
- serde

## Installation
#### Cargo.toml
    [dependencies]
    chrono = "0.4"
    mango-orm = "0.4.75-beta"
    metamorphose = "0.2.56-beta"
    lazy_static = "1"
    rand = "0.7"
    regex = "1"
    rust-argon2 = "0.8"
    serde_json = "1"
    validator = "0.11"
    
    [dependencies.mongodb]
    default-features = false
    features = ["sync"]
    version = "1"
    
    [dependencies.serde]
    features = ["derive"]
    version = "1"

## Examples Usage:
#### Model
    use mango_orm::*;
    use metamorphose::Model;
    use serde::{Deserialize, Serialize};
    
    // Get settings of service/sub-application.
    use crate::settings::{
        default::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
        PROJECT_NAME, UNIQUE_PROJECT_KEY,
    };
    
    #[Model(
        is_del_docs = false,
        is_use_add_valid = true,
        ignore_fields = "confirm_password"
    )]
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct UserProfile {
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
            widget = "inputImage",
            label = "Photo",
            value = r#"{
                    "path":"./media/no_avatar.png",
                    "url":"/media/no_avatar.png"
                }"#,
            placeholder = "Upload your photo",
            accept = "image/jpeg,image/png",
            hint = "Image in JPEG or PNG format",
            thumbnails = r#"[["xs",150],["sm",300]]"#
        )]
        pub photo: Option<String>,
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
            maxlength = 320,
            hint = "Your actual E-mail"
        )]
        pub email: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPhone",
            label = "Phone number",
            placeholder = "Please enter your phone number",
            unique = true,
            maxlength = 30,
            hint = "Your actual phone number"
        )]
        pub phone: Option<String>,
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
            required = true,
            minlength = 8,
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

#### Form
    use mango_orm::*;
    use metamorphose::Form;
    use serde::{Deserialize, Serialize};
    
    #[Form]
    #[derive(Serialize, Deserialize, Default, Debug)]
    pub struct RestorePassword {
        #[serde(default)]
        #[field_attrs(
            widget = "inputEmail",
            value = "Your Email",
            required = true,
            unique = true,
            maxlength = 74
        )]
        pub email: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPassword",
            value = "Your old password",
            required = true,
            minlength = 8
        )]
        pub old_password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPassword",
            value = "Your new password",
            required = true,
            minlength = 8
        )]
        pub new_password: Option<String>,
        //
        #[serde(default)]
        #[field_attrs(
            widget = "inputPassword",
            value = "Confirm the password",
            required = true,
            minlength = 8
        )]
        pub confirm_password: Option<String>,
    }

## License
#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
