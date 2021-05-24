![Logo](https://github.com/kebasyaty/mango-orm/raw/master/images/logo.svg)

# mango-orm
### ORM-like API MongoDB for Rust
#### For maximum convenience use  [mango-panel](https://github.com/kebasyaty/mango-panel "mango-panel") 

## Requirements
- mongodb
- serde
- chrono
- image
- lazy_static
- rand
- regex
- rust-argon2
- serde_json
- validator
- metamorphose

## Installation
#### Cargo.toml
    [dependencies]
    mango-orm = "0.4.75-beta"
    metamorphose = "0.2.56-beta"
    chrono = "0.4"
    image = "0.23"
    lazy_static = "1.0"
    rand = "0.7"
    regex = "1.3"
    rust-argon2 = "0.8"
    serde_json = "1.0"
    validator = "0.11"
    
    [dependencies.mongodb]
    default-features = false
    features = ["sync"]
    version = "1.0"
    
    [dependencies.serde]
    features = ["derive"]
    version = "1.0"

## Example Usage:
#### settings.rs
    // General settings for the project.
    // Project name.
    // Hint: PROJECT_NAM it is recommended not to change.
    // Valid characters: _ a-z A-Z 0-9
    // Max size: 21
    // First character: a-z A-Z
    pub const PROJECT_NAME: &str = "store";
    
    // Unique project key.
    // Hint: UNIQUE_PROJECT_KEY it is recommended not to change.
    // Valid characters: a-z A-Z 0-9
    // Size: 8-16
    // Example: "7rzgacfqQB3B7q7T"
    pub const UNIQUE_PROJECT_KEY: &str = "Gm5EKQZRAWvV6BW";
    
    // Settings for user accounts.
    pub mod users {
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 31
        // First character: a-z A-Z
        pub const SERVICE_NAME: &str = "admin";
        // Valid characters: _ a-z A-Z 0-9
        // Max size: 21
        // First character: a-z A-Z
        pub const DATABASE_NAME: &str = "users";
        pub const DB_CLIENT_NAME: &str = "default";
        pub const DB_QUERY_DOCS_LIMIT: u32 = 1000;
    }

#### migration.rs
    use crate::{models, settings};
    use mango_orm::{CachingModel, Monitor, ToModel, MONGODB_CLIENT_STORE};
    
    // Migration Service `Mango`.
    pub fn mango_migration() -> Result<(), Box<dyn std::error::Error>> {
        // Caching MongoDB clients.
        {
            let mut client_store = MONGODB_CLIENT_STORE.write()?;
            client_store.insert(
                "default".to_string(),
                mongodb::sync::Client::with_uri_str("mongodb://localhost:27017")?,
            );
        }
        // Monitor initialization.
        let monitor = Monitor {
            project_name: settings::PROJECT_NAME,
            unique_project_key: settings::UNIQUE_PROJECT_KEY,
            // Register models.
            models: vec![
                models::UserProfile::meta()?,
            ],
        };
        monitor.migrat()?;
        Ok(())
    }

#### models.rs
    use mango_orm::*;
    use metamorphose::Model;
    use regex::RegexBuilder;
    use serde::{Deserialize, Serialize};
    
    use crate::settings::{
        users::{DATABASE_NAME, DB_CLIENT_NAME, DB_QUERY_DOCS_LIMIT, SERVICE_NAME},
        PROJECT_NAME, UNIQUE_PROJECT_KEY,
    };
    
    // User profiles
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
            thumbnails = r#"[["xs",150],["sm",300]]"#    // all sizes: "xs","sm","md","lg"
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
    
    impl AdditionalValidation for UserProfile {
        fn add_validation<'a>(
            &self,
        ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
            // Hint: error_map.insert("field_name", "Error message.")
            let mut error_map: std::collections::HashMap<&'a str, &'a str> =
                std::collections::HashMap::new();
    
            // Get clean data
            let hash = self.hash.clone().unwrap_or_default();
            let password = self.password.clone().unwrap_or_default();
            let confirm_password = self.confirm_password.clone().unwrap_or_default();
            let username = self.username.clone().unwrap_or_default();
    
            // Fields validation
            if hash.is_empty() && password != confirm_password {
                error_map.insert("confirm_password", "Password confirmation does not match.");
            }
            if !RegexBuilder::new(r"^[a-z\d_@+.]+$")
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

#### main.rs
    use mango_orm::*;
    
    mod models;
    mod settings;
    mod migration;
    
    fn main() -> Result<(), Box<dyn std::error::Error>> {
        // Run migration.
        migration::mango_migration()?;
        //
        let mut user = models::UserProfile {
        username: Some("testname.to_string()),
        email: Some("test@test.test".to_string()),
        password: Some("12345678".to_string()),
        confirm_password: Some("12345678".to_string()),
        is_staff: Some(false),
        is_active: Some(true),
        ..Default::default() // or initialize the `hash` field - { hash: Some(String::new()) }
        };
        
        let result = user.save(None, None)?;
        println!("is_valid: {}", result.is_valid());
        println!("Hash: {}", result.hash()?);
        println!("Widget map:\n{:?}", result.wig());
        println!("Json-line:\n{}", result.json()?);
        println!("Html:\n{}", result.html());
        println!("For admin panale: {}", result.json_for_admin()?);
        ...
        //
        Ok(())
    }

## License
#### This project is licensed under the [MIT](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-MIT "MIT") and [Apache Version 2.0](https://github.com/kebasyaty/mango-orm/blob/master/LICENSE-APACHE "Apache Version 2.0")
