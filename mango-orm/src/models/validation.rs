//! # Validating.
//!
//! Trait:
//! `Validation` - Validating Model fields for save and update.
//! Methods:
//! `check_minlength` - Validation of `minlength`.
//! `check_maxlength` - Validation of `maxlength`.
//! `accumula_err` - Accumulation of errors.
//! `regex_validation` - Validation in regular expression (email, password, etc...).
//! `check_unique` - Validation of `unique`.
//!
//! Trait:
//! `AdditionalValidation` - Methods for additional validation.
//! Methods:
//! `add_validation` - To create a custom validation.
//!

use crate::{
    forms::Widget,
    store::{REGEX_IS_COLOR_CODE, REGEX_IS_DATE, REGEX_IS_DATETIME, REGEX_IS_PASSWORD},
};

/// Validating Model fields for save and update.
// *************************************************************************************************
pub trait ValidationModel {
    /// Validation of `minlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(format!("Exceeds limit, minlength={}.", minlength))?
        }
        Ok(())
    }

    /// Validation of `maxlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(format!("Exceeds limit, maxlength={}.", maxlength))?
        }
        Ok(())
    }

    /// Accumulation of errors.
    // ---------------------------------------------------------------------------------------------
    fn accumula_err(widget: &Widget, err: &String) -> Result<String, Box<dyn std::error::Error>> {
        let mut tmp = widget.error.clone();
        tmp = if !tmp.is_empty() {
            format!("{}<br>", tmp)
        } else {
            String::new()
        };
        Ok(format!("{}{}", tmp, err))
    }

    /// Validation in regular expression (email, password, etc...).
    // ---------------------------------------------------------------------------------------------
    fn regex_validation(field_type: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        match field_type {
            "inputEmail" => {
                if !validator::validate_email(value) {
                    Err("Invalid email address.")?
                }
            }
            "inputColor" => {
                if !REGEX_IS_COLOR_CODE.is_match(value) {
                    Err("Invalid Color code.")?
                }
            }
            "inputUrl" => {
                if !validator::validate_url(value) {
                    Err("Invalid Url.")?
                }
            }
            "inputIP" => {
                if !validator::validate_ip(value) {
                    Err("Invalid IP address.")?
                }
            }
            "inputIPv4" => {
                if !validator::validate_ip_v4(value) {
                    Err("Invalid IPv4 address.")?
                }
            }
            "inputIPv6" => {
                if !validator::validate_ip_v6(value) {
                    Err("Invalid IPv6 address.")?
                }
            }
            "inputPassword" => {
                if !REGEX_IS_PASSWORD.is_match(value) {
                    Err("Size 8-256 chars ; Allowed chars: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (")?
                }
            }
            "inputDate" => {
                if !REGEX_IS_DATE.is_match(value) {
                    Err("Incorrect date format.<br>\
                         Example: 1970-02-28")?
                }
            }
            "inputDateTime" => {
                if !REGEX_IS_DATETIME.is_match(value) {
                    Err("Incorrect date and time format.<br>\
                         Example: 1970-02-28T00:00")?
                }
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    /// Validation of `unique`.
    // ---------------------------------------------------------------------------------------------
    fn check_unique(
        hash: &str,
        field_name: &str,
        bson_field_value: &mongodb::bson::Bson,
        coll: &mongodb::sync::Collection,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let object_id = mongodb::bson::oid::ObjectId::with_string(hash);
        let mut filter = mongodb::bson::doc! { field_name: bson_field_value };
        if let Ok(id) = object_id {
            // If the document is will updated.
            filter = mongodb::bson::doc! {
                "$and": [
                    { "_id": { "$ne": id } },
                    filter
                ]
            };
        }
        let count: i64 = coll.count_documents(filter, None)?;
        if count > 0 {
            Err("Is not unique.")?
        }
        Ok(())
    }
}

/// Methods for additional validation.
/// Hint: For custom use, add the Model/Form attribute `is_use_add_valid = true`.
/// Hint (for models): Remember to use for validate of ignored fields.
// *************************************************************************************************
pub trait AdditionalValidation {
    // Default implementation as a stub.
    fn add_validation<'a>(
        &self,
    ) -> Result<std::collections::HashMap<&'a str, &'a str>, Box<dyn std::error::Error>> {
        // error_map.insert("field_name", "Error message.")
        let error_map: std::collections::HashMap<&'a str, &'a str> =
            std::collections::HashMap::new();
        Ok(error_map)
    }
}
