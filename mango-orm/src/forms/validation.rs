//! # Validating.
//!
//! Trait:
//! `Validation` - Validating Form fields.
//! Methods:
//! `check_minlength` - Validation of `minlength`.
//! `check_maxlength` - Validation of `maxlength`.
//! `accumula_err` - Accumulation of errors.
//! `regex_validation` - Validation in regular expression (email, password, etc...).
//! `check` - Checking the Form before other proceeding.
//!

use crate::{
    forms::{
        caching::CachingForm, output_data::OutputDataForm, FileData, ImageData, ToForm, Widget,
    },
    models::validation::AdditionalValidation,
    store::{REGEX_IS_COLOR_CODE, REGEX_IS_DATE, REGEX_IS_DATETIME, REGEX_IS_PASSWORD},
};

// Validating Form fields for save and update.
// *************************************************************************************************
pub trait ValidationForm: ToForm + CachingForm + AdditionalValidation {
    // Validation of `minlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_minlength(minlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if minlength > 0 && value.encode_utf16().count() < minlength {
            Err(format!("Exceeds limit, minlength={}.", minlength))?
        }
        Ok(())
    }

    // Validation of `maxlength`.
    // ---------------------------------------------------------------------------------------------
    fn check_maxlength(maxlength: usize, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if maxlength > 0 && value.encode_utf16().count() > maxlength {
            Err(format!("Exceeds limit, maxlength={}.", maxlength))?
        }
        Ok(())
    }

    // Accumulation of errors.
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

    // Validation in regular expression (email, password, etc...).
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
                    Err(
                        "Allowed characters: a-z A-Z 0-9 @ # $ % ^ & + = * ! ~ ) (<br> \
                                 Minimum size 8 characters",
                    )?
                }
            }
            "inputDate" => {
                if !REGEX_IS_DATE.is_match(value) {
                    Err("Incorrect date format.<br>Example: 1970-02-28")?
                }
            }
            "inputDateTime" => {
                if !REGEX_IS_DATETIME.is_match(value) {
                    Err("Incorrect date and time format.<br>Example: 1970-02-28T00:00")?
                }
            }
            _ => return Ok(()),
        }
        Ok(())
    }

    // Checking the Form before other proceeding.
    // ---------------------------------------------------------------------------------------------
    fn check(&self) -> Result<OutputDataForm, Box<dyn std::error::Error>> {
        // Get cached Model data.
        let form_cache = Self::get_cache_data()?;
        // Get model name.
        let form_name: &str = &Self::form_name()[..];
        // User input error detection symptom.
        let mut is_err_symptom = false;
        // Get preliminary data from the model.
        let mut pre_json: serde_json::value::Value = self.self_to_json()?;

        // Validation of field by attributes (maxlength, unique, min, max, etc...).
        // -----------------------------------------------------------------------------------------
        let fields_name = Self::fields_name()?;
        let fields_name: Vec<&str> = fields_name.iter().map(|item| item.as_str()).collect();
        let mut final_map_widgets: std::collections::HashMap<String, Widget> =
            form_cache.map_widgets.clone();
        // Apply additional validation.
        {
            let error_map = self.add_validation()?;
            if !error_map.is_empty() {
                is_err_symptom = true;
                for (field_name, err_msg) in error_map {
                    if !fields_name.contains(&field_name) {
                        Err(format!(
                            "Form: `{}` >  Method: `add_validation()` : \
                                            The `{}` field is missing from the form.",
                            form_name, field_name
                        ))?
                    }
                    if let Some(widget) = final_map_widgets.get_mut(&field_name.to_owned()) {
                        widget.error = Self::accumula_err(&widget, &err_msg.to_string())?;
                    }
                }
            }
        }
        // Loop over fields for validation.
        for field_name in fields_name {
            // Get field value for validation.
            let pre_json_value: Option<&serde_json::value::Value> = pre_json.get(field_name);
            // Check field value.
            if pre_json_value.is_none() {
                Err(format!(
                    "Form: `{}` > Field: `{}` > Method: `check()` : This field is missing.",
                    form_name, field_name
                ))?
            }
            //
            let pre_json_value: &serde_json::value::Value = pre_json_value.unwrap();
            let final_widget: &mut Widget = final_map_widgets.get_mut(field_name).unwrap();
            let widget_type: &str = &final_widget.widget.clone()[..];
            // Field validation.
            match widget_type {
                // Validation of text type fields.
                // *********************************************************************************
                "checkBoxText" | "radioText" | "inputColor" | "inputEmail" | "inputPassword"
                | "inputPhone" | "inputText" | "inputUrl" | "inputIP" | "inputIPv4"
                | "inputIPv6" | "textArea" | "hiddenText" => {
                    // Get field value for validation.
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        if widget_type != "inputPassword" {
                            final_widget.value = clean_data.clone();
                        } else {
                            final_widget.value = String::new();
                        }
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default.
                            if widget_type != "inputPassword" {
                                if !final_widget.value.is_empty() {
                                    field_value = final_widget.value.trim().to_string();
                                    // To deserialize an instance of a form with default values.
                                    *pre_json.get_mut(field_name).unwrap() =
                                        serde_json::json!(field_value);
                                    final_widget.value = String::new();
                                } else {
                                    final_widget.value = String::new();
                                    continue;
                                }
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression.
                    // Checking `minlength`, `maxlength`, `min length`, `max length`.
                    // -----------------------------------------------------------------------------
                    Self::check_minlength(final_widget.minlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                        },
                    );
                    Self::check_maxlength(final_widget.maxlength, field_value).unwrap_or_else(
                        |err| {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                        },
                    );
                    // Validation of range (`min` <> `max`).
                    // Hint: The `validate_length()` method did not
                    // provide the desired result.
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.minlength.clone() as f64;
                    let max: f64 = final_widget.maxlength.clone() as f64;
                    let len: f64 = field_value.encode_utf16().count() as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            len,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Length {} is out of range (min={} <> max={}).",
                            len, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                    // Validation in regular expression (email, password, etc...).
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                }
                // Validation of date type fields.
                // *********************************************************************************
                "inputDate" | "inputDateTime" => {
                    // Get field value for validation.
                    let mut field_value: String = if !pre_json_value.is_null() {
                        let clean_data: String =
                            pre_json_value.as_str().unwrap().trim().to_string();
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = clean_data.clone();
                        clean_data
                    } else {
                        String::new()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if field_value.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default.
                            if !final_widget.value.is_empty() {
                                field_value = final_widget.value.trim().to_string();
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() =
                                    serde_json::json!(field_value);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Convert to &str
                    let field_value: &str = field_value.as_str();
                    // Validation in regular expression.
                    // -----------------------------------------------------------------------------
                    Self::regex_validation(widget_type, field_value).unwrap_or_else(|err| {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                    });
                    if is_err_symptom {
                        continue;
                    }
                    // Create Date and Time Object.
                    // -----------------------------------------------------------------------------
                    // Date to DateTime.
                    let dt_value: chrono::DateTime<chrono::Utc> = {
                        let field_value: String = if widget_type == "inputDate" {
                            format!("{}T00:00", field_value.to_string())
                        } else {
                            field_value.to_string()
                        };
                        chrono::DateTime::<chrono::Utc>::from_utc(
                            chrono::NaiveDateTime::parse_from_str(&field_value, "%Y-%m-%dT%H:%M")?,
                            chrono::Utc,
                        )
                    };
                    // Create dates for `min` and `max` attributes values to
                    // check, if the value of user falls within the range
                    // between these dates.
                    if final_widget.min != "0".to_string() && final_widget.max != "0".to_string() {
                        // Validation in regular expression (min).
                        Self::regex_validation(widget_type, final_widget.min.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        // Validation in regular expression (max).
                        Self::regex_validation(widget_type, final_widget.max.as_str())
                            .unwrap_or_else(|err| {
                                is_err_symptom = true;
                                final_widget.error =
                                    Self::accumula_err(&final_widget, &err.to_string()).unwrap();
                            });
                        if is_err_symptom {
                            continue;
                        }
                        // Date to DateTime (min).
                        let dt_min: chrono::DateTime<chrono::Utc> = {
                            let min_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", final_widget.min.clone())
                            } else {
                                final_widget.min.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &min_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        // Date to DateTime (max).
                        let dt_max: chrono::DateTime<chrono::Utc> = {
                            let max_value: String = if widget_type == "inputDate" {
                                format!("{}T00:00", final_widget.max.clone())
                            } else {
                                final_widget.max.clone()
                            };
                            chrono::DateTime::<chrono::Utc>::from_utc(
                                chrono::NaiveDateTime::parse_from_str(
                                    &max_value,
                                    "%Y-%m-%dT%H:%M",
                                )?,
                                chrono::Utc,
                            )
                        };
                        // Check hit in range (min <> max).
                        if dt_value < dt_min || dt_value > dt_max {
                            is_err_symptom = true;
                            final_widget.error = Self::accumula_err(
                                &final_widget,
                                &"Date out of range between `min` and` max`.".to_owned(),
                            )
                            .unwrap();
                            continue;
                        }
                    }
                }
                // Validation of `select` type fields.
                // *********************************************************************************
                "selectText" | "selectI32" | "selectU32" | "selectI64" | "selectF64"
                | "selectTextDyn" | "selectI32Dyn" | "selectU32Dyn" | "selectI64Dyn"
                | "selectF64Dyn" => {
                    // Get selected items.
                    if !pre_json_value.is_null() {
                        match widget_type {
                            "selectText" | "selectTextDyn" => {
                                let val = pre_json_value.as_str().unwrap().to_string();
                                final_widget.value = val.clone();
                            }
                            "selectI32" | "selectI32Dyn" => {
                                let val = pre_json_value.as_i64().unwrap() as i32;
                                final_widget.value = val.to_string();
                            }
                            "selectU32" | "selectI64" | "selectU32Dyn" | "selectI64Dyn" => {
                                let val = pre_json_value.as_i64().unwrap();
                                final_widget.value = val.to_string();
                            }
                            "selectF64" | "selectF64Dyn" => {
                                let val = pre_json_value.as_f64().unwrap();
                                final_widget.value = val.to_string();
                            }
                            _ => Err(format!(
                                "Model: `{}` > Field: `{}` > Method: `check()` : \
                                        Unsupported widget type - `{}`.",
                                form_name, field_name, widget_type
                            ))?,
                        }
                    } else {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                        } else if !final_widget.widget.contains("Dyn")
                            && !final_widget.value.is_empty()
                        {
                            // Trying to apply the value default.
                            match widget_type {
                                "selectText" => {
                                    let val = final_widget.value.trim().to_string();
                                    // To deserialize an instance of a form with default values.
                                    *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                }
                                "selectI32" => {
                                    let val = final_widget.value.trim().parse::<i32>().unwrap();
                                    *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                }
                                "selectU32" | "selectI64" => {
                                    let val = final_widget.value.trim().parse::<i64>().unwrap();
                                    *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                }
                                "selectF64" => {
                                    let val = final_widget.value.trim().parse::<f64>().unwrap();
                                    *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                }
                                _ => Err(format!(
                                    "Model: `{}` > Field: `{}` > Method: `check()` : \
                                    Unsupported widget type - `{}`.",
                                    form_name, field_name, widget_type
                                ))?,
                            }
                        }
                        final_widget.value = String::new();
                    }
                }
                "selectTextMult" | "selectI32Mult" | "selectU32Mult" | "selectI64Mult"
                | "selectF64Mult" | "selectTextMultDyn" | "selectI32MultDyn"
                | "selectU32MultDyn" | "selectI64MultDyn" | "selectF64MultDyn" => {
                    // Get selected items.
                    if pre_json_value.is_null() && final_widget.required {
                        is_err_symptom = true;
                        final_widget.error =
                            Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                .unwrap();
                    }
                    final_widget.value = String::new();
                }
                // Validation of file type fields.
                // *********************************************************************************
                "inputFile" => {
                    // Get field value for validation.
                    let mut field_value: FileData = if !pre_json_value.is_null() {
                        let clean_data: FileData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        FileData::default()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if field_value.path.is_empty() && field_value.url.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default.
                            if !final_widget.value.is_empty() {
                                field_value = serde_json::from_str(final_widget.value.trim())?;
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() =
                                    serde_json::json!(field_value);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    final_widget.value = String::new();
                    // Flags to check.
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Incorrectly filled field. \
                            Example: {{\"path\":\"./media/hello_world.odt\",\"url\":\"/media/hello_world.odt\"}}",
                            form_name, field_name
                        ))?
                    }
                }
                "inputImage" => {
                    // Get field value for validation.
                    let mut field_value: ImageData = if !pre_json_value.is_null() {
                        let clean_data: ImageData =
                            serde_json::from_str(pre_json_value.as_str().unwrap())?;
                        clean_data
                    } else {
                        ImageData::default()
                    };
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    if field_value.path.is_empty() && field_value.url.is_empty() {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else {
                            // Trying to apply the value default.
                            if !final_widget.value.is_empty() {
                                field_value = serde_json::from_str(final_widget.value.trim())?;
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() =
                                    serde_json::json!(field_value);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    final_widget.value = String::new();
                    // Flags to check.
                    let is_emty_path = field_value.path.is_empty();
                    let is_emty_url = field_value.url.is_empty();
                    // Invalid if there is only one value.
                    if (!is_emty_path && is_emty_url) || (is_emty_path && !is_emty_url) {
                        Err(format!(
                            "Model: `{}` > Field: `{}` > Method: \
                            `check()` : Incorrectly filled field. \
                            Example: {{\"path\":\"./media/hello_world.odt\",\"url\":\"/media/hello_world.odt\"}}",
                            form_name, field_name
                        ))?
                    }
                }
                // Validation of number type fields.
                // *********************************************************************************
                "checkBoxI32" | "radioI32" | "numberI32" | "rangeI32" | "hiddenI32" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !final_widget.value.is_empty() {
                                let val = final_widget.value.trim().parse::<i64>().unwrap();
                                field_value = Some(val);
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data.
                    let field_value: i32 = field_value.unwrap() as i32;
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                }
                "checkBoxU32" | "radioU32" | "numberU32" | "rangeU32" | "checkBoxI64"
                | "radioI64" | "numberI64" | "rangeI64" | "hiddenU32" | "hiddenI64" => {
                    // Get field value for validation.
                    let mut field_value: Option<i64> = pre_json_value.as_i64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error.
                    // ( The default value is used whenever possible )
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !final_widget.value.is_empty() {
                                let val = final_widget.value.trim().parse::<i64>().unwrap();
                                field_value = Some(val);
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data.
                    let field_value: i64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value as f64;
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                }
                "checkBoxF64" | "radioF64" | "numberF64" | "rangeF64" | "hiddenF64" => {
                    // Get field value for validation.
                    let mut field_value: Option<f64> = pre_json_value.as_f64();
                    // Define field state flag.
                    let is_null_value: bool = pre_json_value.is_null();
                    // Validation, if the field is required and empty, accumulate the error
                    // ( The default value is used whenever possible ).
                    // -----------------------------------------------------------------------------
                    if is_null_value {
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error =
                                Self::accumula_err(&final_widget, &"Required field.".to_owned())
                                    .unwrap();
                            final_widget.value = String::new();
                            continue;
                        } else if is_null_value {
                            if !final_widget.value.is_empty() {
                                let val = final_widget.value.trim().parse::<f64>().unwrap();
                                field_value = Some(val);
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() = serde_json::json!(val);
                                final_widget.value = String::new();
                            } else {
                                final_widget.value = String::new();
                                continue;
                            }
                        }
                    }
                    // Get clean data.
                    let field_value: f64 = field_value.unwrap();
                    if !is_null_value {
                        // In case of an error, return the current
                        // state of the field to the user (client).
                        final_widget.value = field_value.to_string();
                    }
                    // Validation of range (`min` <> `max`).
                    // -----------------------------------------------------------------------------
                    let min: f64 = final_widget.min.parse().unwrap();
                    let max: f64 = final_widget.max.parse().unwrap();
                    let num: f64 = field_value.clone();
                    if (min > 0_f64 || max > 0_f64)
                        && !validator::validate_range(
                            validator::Validator::Range {
                                min: Some(min),
                                max: Some(max),
                            },
                            num,
                        )
                    {
                        is_err_symptom = true;
                        let msg = format!(
                            "Number {} is out of range (min={} <> max={}).",
                            num, min, max
                        );
                        final_widget.error = Self::accumula_err(&final_widget, &msg).unwrap();
                    }
                }
                // Validation of boolean type fields.
                // *********************************************************************************
                "checkBoxBool" => {
                    // Get field value for validation.
                    // -----------------------------------------------------------------------------
                    let field_value: bool = if pre_json_value.is_null() {
                        let mut result = false;
                        // Validation, if the field is required and empty, accumulate the error.
                        // ( The default value is used whenever possible )
                        // -------------------------------------------------------------------------
                        if final_widget.required {
                            is_err_symptom = true;
                            final_widget.error = Self::accumula_err(
                                &final_widget,
                                &"You must definitely choose.".to_owned(),
                            )
                            .unwrap();
                        } else {
                            // Trying to apply the value default.
                            if !final_widget.value.is_empty() {
                                result = final_widget.value.trim().parse::<bool>().unwrap();
                                // To deserialize an instance of a form with default values.
                                *pre_json.get_mut(field_name).unwrap() = serde_json::json!(result);
                            }
                        }
                        result
                    } else {
                        true
                    };
                    final_widget.value = String::new();
                    // In case of an error, return the current
                    // state of the field to the user (client).
                    final_widget.checked = field_value.clone();
                }
                _ => Err(format!(
                    "Form: `{}` > Field: `{}` > Method: `check()` : Unsupported widget type.",
                    form_name, field_name
                ))?,
            }
        }

        // Return result.
        // -----------------------------------------------------------------------------------------
        Ok(OutputDataForm::CheckForm((
            !is_err_symptom,
            Self::fields_name()?.clone(),
            final_map_widgets,
            pre_json,
        )))
    }
}