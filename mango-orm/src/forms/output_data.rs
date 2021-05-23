//! # Output data types for Forms
//!
//! `OutputDataForm` - To return results after processing Forms.
//!

use crate::forms::{html_controls::HtmlControls, Widget};

/// Output data type
#[derive(Debug)]
pub enum OutputDataForm {
    CheckForm(
        (
            bool,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            serde_json::value::Value,
        ),
    ),
    CheckModel(
        (
            bool,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            mongodb::bson::document::Document,
        ),
    ),
    Save(
        (
            bool,
            Vec<String>,
            std::collections::HashMap<String, Widget>,
            String,
        ),
    ),
    Delete((bool, String)),
}

impl HtmlControls for OutputDataForm {
    /// Get Html-line
    fn html(&self) -> String {
        match self {
            Self::CheckForm(data) => Self::to_html(&data.1, data.2.clone()),
            Self::CheckModel(data) => Self::to_html(&data.1, data.2.clone()),
            Self::Save(data) => Self::to_html(&data.1, data.2.clone()),
            _ => panic!("Invalid output type."),
        }
    }
}

impl OutputDataForm {
    /// Get Hash-line
    // ---------------------------------------------------------------------------------------------
    fn to_hash(
        map_widgets: &std::collections::HashMap<String, Widget>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut errors = String::new();
        for (field_name, widget) in map_widgets {
            let tmp = errors.clone();
            if !widget.error.is_empty() {
                errors = format!("{}Field: `{}` : {}", tmp, field_name, widget.error);
            }
        }
        if !errors.is_empty() {
            errors = errors.replace("<br>", " | ");
            //Err(errors)?
            println!("\n{}\n", errors);
        }
        Ok(map_widgets.get("hash").unwrap().value.clone())
    }

    /// Get Hash-line
    // ---------------------------------------------------------------------------------------------
    pub fn hash(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::CheckModel(data) => Ok(Self::to_hash(&data.2)?),
            Self::Save(data) => Ok(Self::to_hash(&data.2)?),
            _ => panic!("Invalid output type."),
        }
    }

    /// Get MongoDB ID from hash-line
    // ---------------------------------------------------------------------------------------------
    pub fn id(&self) -> Result<mongodb::bson::oid::ObjectId, Box<dyn std::error::Error>> {
        match self {
            Self::CheckModel(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.2)?.as_str(),
            )?),
            Self::Save(data) => Ok(mongodb::bson::oid::ObjectId::with_string(
                Self::to_hash(&data.2)?.as_str(),
            )?),
            _ => panic!("Invalid output type."),
        }
    }

    /// Get Map of Widgets
    // ---------------------------------------------------------------------------------------------
    /// ( Wig - Widgets )
    pub fn wig(&self) -> std::collections::HashMap<String, Widget> {
        match self {
            Self::CheckForm(data) => data.2.clone(),
            Self::CheckModel(data) => data.2.clone(),
            Self::Save(data) => data.2.clone(),
            _ => panic!("Invalid output type."),
        }
    }

    /// Get Json-line
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::CheckForm(data) => Ok(serde_json::to_string(&data.2)?),
            Self::CheckModel(data) => Ok(serde_json::to_string(&data.2)?),
            Self::Save(data) => Ok(serde_json::to_string(&data.2)?),
            _ => panic!("Invalid output type."),
        }
    }

    /// Json-line for admin panel.
    // ---------------------------------------------------------------------------------------------
    /// ( converts a widget map to a list, in the order of the Model fields )
    pub fn json_for_admin(&self) -> Result<String, Box<dyn std::error::Error>> {
        let data = match self {
            Self::Save(data) => data,
            _ => panic!("Invalid output type."),
        };
        let map_widgets = data.2.clone();
        let mut widget_list: Vec<Widget> = Vec::new();
        let hash = map_widgets.get("hash").unwrap().clone().value;
        // Get a list of widgets in the order of the model fields.
        for field_name in data.1.iter() {
            let mut widget = map_widgets.get(field_name).unwrap().clone();
            if field_name.contains("password") && !hash.is_empty() {
                widget.widget = "hiddenText".to_string();
                widget.input_type = "hidden".to_string();
            }
            widget_list.push(widget);
        }
        //
        Ok(serde_json::to_string(&widget_list)?)
    }

    /// Get validation status (boolean)
    // ---------------------------------------------------------------------------------------------
    ///
    /// # Example:
    ///
    /// ```
    /// let restore_password  = RestorePasswordForm {...}
    /// let result = restore_password.check()?;
    /// assert!(result.is_valid());
    /// ```
    ///
    pub fn is_valid(&self) -> bool {
        match self {
            Self::CheckForm(data) => data.0,
            Self::CheckModel(data) => data.0,
            Self::Save(data) => data.0,
            Self::Delete(data) => data.0,
        }
    }

    /// Get Document
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::CheckModel(data) => data.3.clone(),
            _ => panic!("Invalid output type."),
        }
    }

    /// Get Form instance.)
    // ---------------------------------------------------------------------------------------------
    /// (It is convenient if the form passes (after validation) the value of the fields to Models.
    pub fn form<T>(&self) -> Result<T, serde_json::error::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Self::CheckForm(data) => serde_json::from_value::<T>(data.3.clone()),
            _ => panic!("Invalid output type."),
        }
    }

    /// A description of the error if the document was not deleted.
    // ---------------------------------------------------------------------------------------------
    /// (Main use for admin panel.)
    ///
    /// # Example:
    ///
    /// ```
    /// let output_data = RestorePasswordForm.delete()?;
    /// println!("{}", output_data.err_msg());
    /// ```
    ///
    pub fn err_msg(&self) -> String {
        match self {
            Self::Delete(data) => data.1.clone(),
            _ => panic!("Invalid output type."),
        }
    }
}
