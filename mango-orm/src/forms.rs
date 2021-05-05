//! # Forms.
//! To create form of search, form of recover password, combine multiple models, etc.
//!
//! `ToForm` - Define form settings for models (widgets, html).
//! `Widget` - Form controls parameters.
//! `OutputData` - Output data for the `check()` and `save()` methods.
//! `TransMapWidgetType` - For transporting of Widget types map to implementation of methods.
//! `TransMapWidgets` - For transporting of Widgets map to implementation of methods.
//! `HtmlControls` - Rendering HTML-controls code for Form.
//! ( If necessary, customize the code generation yourself using html and css from Bootstrap, Material Design, etc. )
//!

pub mod caching;
pub mod html_controls;
pub mod output_data;
pub mod validation;

// FORMS
// #################################################################################################
// Data structures for `inputFile` and `inputImage` widgets.
// *************************************************************************************************
#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct FileData {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // in bytes
}

#[derive(Default, serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct ImageData {
    #[serde(default)]
    pub path: String, // max size = original
    #[serde(default)]
    pub path_xs: String, // max size = 200 px
    #[serde(default)]
    pub path_sm: String, // max size = 400 px
    #[serde(default)]
    pub path_md: String, // max size = 800 px
    #[serde(default)]
    pub path_lg: String, // max size = 1600 px
    #[serde(default)]
    pub url: String, // max size = original
    #[serde(default)]
    pub url_xs: String, // max size = 200 px
    #[serde(default)]
    pub url_sm: String, // max size = 400 px
    #[serde(default)]
    pub url_md: String, // max size = 800 px
    #[serde(default)]
    pub url_lg: String, // max size = 1600 px
    #[serde(default)]
    pub name: String,
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub size: u32, // in bytes
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub width: u32, // in pixels
    #[serde(default, with = "mongodb::bson::compat::u2f")]
    pub height: u32, // in pixels
}

// Widget.
// ( Form controls parameters )
// *************************************************************************************************
#[derive(serde::Serialize, serde::Deserialize, PartialEq, Clone, Debug)]
pub struct Widget {
    pub id: String, // Example: "model-name--field-name" ( The value is determined automatically )
    pub label: String,
    pub widget: String,
    pub input_type: String, // The value is determined automatically.
    pub name: String,       // The value is determined automatically.
    pub value: String,
    pub accept: String, // Hint: accept="image/jpeg,image/png,image/gif"
    pub placeholder: String,
    pub pattern: String, // Validating a field using a client-side regex.
    pub minlength: usize,
    pub maxlength: usize,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox">
    pub unique: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub step: String,
    pub min: String,
    pub max: String,
    pub other_attrs: String, // "autofocus tabindex=\"some number\" size=\"some number\" ..."
    pub css_classes: String, // Hint: "class-name class-name ..."
    pub options: Vec<(String, String)>, // Hint: <value, Title> - <option value="value1">Title 1</option>
    pub hint: String,
    pub warning: String,    // The value is determined automatically.
    pub error: String,      // The value is determined automatically.
    pub common_msg: String, // Messages common to the entire Form.
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            id: String::new(),
            label: String::new(),
            widget: String::from("inputText"),
            input_type: String::from("text"),
            name: String::new(),
            value: String::new(),
            accept: String::new(),
            placeholder: String::new(),
            pattern: String::new(),
            minlength: 0_usize,
            maxlength: 256_usize,
            required: false,
            checked: false,
            unique: false,
            disabled: false,
            readonly: false,
            step: String::from("1"),
            min: String::new(),
            max: String::new(),
            other_attrs: String::new(),
            css_classes: String::new(),
            options: Vec::new(),
            hint: String::new(),
            warning: String::new(),
            error: String::new(),
            common_msg: String::new(),
        }
    }
}

// For transporting of Widgets map to implementation of methods.
// Hint: <field name, Widget>
#[derive(serde::Deserialize)]
pub struct TransMapWidgets {
    pub map_widgets: std::collections::HashMap<String, Widget>,
}

// Form settings.
// *************************************************************************************************
pub trait ToForm {
    // Get form key.
    // (To access data in the cache)
    // ---------------------------------------------------------------------------------------------
    fn key() -> String;

    // Get form name
    // ---------------------------------------------------------------------------------------------
    fn form_name() -> String;

    // Get fields name list.
    // ---------------------------------------------------------------------------------------------
    fn fields_name() -> Result<Vec<String>, Box<dyn std::error::Error>>;

    // Get map of widgets for Form fields.
    // Hint: <field name, Widget>
    // ---------------------------------------------------------------------------------------------
    fn widgets() -> Result<std::collections::HashMap<String, Widget>, Box<dyn std::error::Error>>;

    // Serialize Form to json-line.
    // ---------------------------------------------------------------------------------------------
    fn self_to_json(&self) -> Result<serde_json::value::Value, Box<dyn std::error::Error>>;
}
