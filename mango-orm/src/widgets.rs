//! # widgets
//!
//! Widgets for Forms.

// WIDGETS =========================================================================================
/// Mediator for transporting widget attributes
#[derive(Default, Debug)]
pub struct Transport {
    pub id: Option<String>,
    pub label: Option<String>,
    pub field_type: Option<FieldType>,
    pub value: Option<DataType>,
    pub required: Option<bool>,
    pub readonly: Option<bool>, // For <input type="...">
    pub disabled: Option<bool>, // For <select></select>
    pub multiple: Option<bool>, // For <select></select>
    pub hint: Option<String>,
    pub unique: Option<bool>,
    pub hidden: Option<bool>,
    pub other_attrs: Option<String>,
    pub other_classes: Option<String>,
    pub select: Option<Vec<(String, DataType)>>, // For <select></select>
}
/// Field Types ------------------------------------------------------------------------------------
#[derive(Debug, Clone)]
pub enum FieldType {
    CheckBox,
    Color,
    Date,
    Email,
    File,
    Hidden,
    Image,
    Number,
    Password,
    Radio,
    Range,
    Tel,
    Text,
    Time,
    Url,
    TextArea,
    Select,
    ForeignKey,
    ManyToMany,
    OneToOne,
}
impl Default for FieldType {
    fn default() -> Self {
        FieldType::Text
    }
}
impl FieldType {
    pub fn get_type(&self) -> String {
        match self {
            Self::CheckBox => "checkbox".to_string(),
            Self::Color => "color".to_string(),
            Self::Date => "date".to_string(),
            Self::Email => "email".to_string(),
            Self::File => "file".to_string(),
            Self::Hidden => "hidden".to_string(),
            Self::Image => "image".to_string(),
            Self::Number => "number".to_string(),
            Self::Password => "password".to_string(),
            Self::Radio => "radio".to_string(),
            Self::Range => "range".to_string(),
            Self::Tel => "tel".to_string(),
            Self::Text => "text".to_string(),
            Self::Time => "time".to_string(),
            Self::Url => "url".to_string(),
            Self::TextArea => "textarea".to_string(),
            Self::Select => "select".to_string(),
            Self::ForeignKey => "m2o".to_string(),
            Self::ManyToMany => "m2m".to_string(),
            Self::OneToOne => "o2o".to_string(),
        }
    }
}

/// Data types for the `value` attribute -----------------------------------------------------------
#[derive(Debug, Clone)]
pub enum DataType {
    Text(String),
    I64(i64),
    U64(u64),
    F64(f64),
    Bool(bool),
}
impl Default for DataType {
    fn default() -> Self {
        DataType::Text(String::new())
    }
}
impl DataType {
    pub fn get_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_string(),
            Self::I64(data) => data.to_string(),
            Self::U64(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
            Self::Bool(data) => data.to_string(),
        }
    }
}

// Attributes for the widget -----------------------------------------------------------------------
/// For standard widgets
/// Use for:
/// <input type="checkbox">
/// <input type="color">
/// <input type="date">
/// <input type="email">
/// <input type="file">
/// <input type="hidden">
/// <input type="image">
/// <input type="number">
/// <input type="password">
/// <input type="radio">
/// <input type="range">
/// <input type="tel">
/// <input type="text">
/// <input type="time">
/// <input type="url">
/// <select></select>
/// <textarea></textarea>
#[derive(Default, Debug)]
pub struct Widget {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub value: DataType,
    pub field_type: FieldType,
    pub required: bool,
    pub readonly: bool, // For <input type="...">
    pub disabled: bool, // For <select></select>
    pub multiple: bool, // For <select></select>
    pub checked: bool,  // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,   // "autofocus ..."
    pub other_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, DataType)>,
}

impl Widget {
    // Get attributes of a widget
    pub fn get_attrs(&self) -> String {
        String::new()
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Standard type -------------------------------------------------------------------------------
    #[test]
    fn test_standard_type() {
        assert_eq!(FieldType::CheckBox.get_type(), "checkbox".to_string());
        assert_eq!(FieldType::Color.get_type(), "color".to_string());
        assert_eq!(FieldType::Date.get_type(), "date".to_string());
        assert_eq!(FieldType::Email.get_type(), "email".to_string());
        assert_eq!(FieldType::Hidden.get_type(), "hidden".to_string());
        assert_eq!(FieldType::Image.get_type(), "image".to_string());
        assert_eq!(FieldType::Number.get_type(), "number".to_string());
        assert_eq!(FieldType::Password.get_type(), "password".to_string());
        assert_eq!(FieldType::Radio.get_type(), "radio".to_string());
        assert_eq!(FieldType::Range.get_type(), "range".to_string());
        assert_eq!(FieldType::Tel.get_type(), "tel".to_string());
        assert_eq!(FieldType::Text.get_type(), "text".to_string());
        assert_eq!(FieldType::Time.get_type(), "time".to_string());
        assert_eq!(FieldType::Url.get_type(), "url".to_string());
        assert_eq!(FieldType::TextArea.get_type(), "textarea".to_string());
        assert_eq!(FieldType::Select.get_type(), "select".to_string());
        assert_eq!(FieldType::ForeignKey.get_type(), "m2o".to_string());
        assert_eq!(FieldType::ManyToMany.get_type(), "m2m".to_string());
        assert_eq!(FieldType::OneToOne.get_type(), "o2o".to_string());
    }

    // Default data type ---------------------------------------------------------------------------
    #[test]
    fn test_default_data_type() {
        assert_eq!(
            DataType::Text("Some text".to_string()).get_data(),
            "Some text".to_string()
        );
        assert_eq!(DataType::I64(10_i64).get_data(), 10_i64.to_string());
        assert_eq!(DataType::U64(10_u64).get_data(), 10_u64.to_string());
        assert_eq!(DataType::F64(10_f64).get_data(), 10_f64.to_string());
        assert_eq!(DataType::Bool(true).get_data(), true.to_string());
    }

    // Widget --------------------------------------------------------------------------------------
    #[test]
    fn test_standard_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), DataType::Text(String::new()))];
        // Fields
        assert_eq!(widget.id, "".to_string());
        assert_eq!(widget.label, "".to_string());
        assert_eq!(
            widget.value.get_data(),
            DataType::Text(String::new()).get_data()
        );
        assert_eq!(widget.field_type.get_type(), FieldType::Text.get_type());
        assert_eq!(widget.required, false);
        assert_eq!(widget.readonly, false);
        assert_eq!(widget.disabled, false);
        assert_eq!(widget.multiple, false);
        assert_eq!(widget.checked, false);
        assert_eq!(widget.hint, "".to_string());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, "".to_string());
        assert_eq!(widget.other_classes, "".to_string());
        assert_eq!(
            widget.select[0].1.get_data(),
            DataType::Text(String::new()).get_data()
        );
        // Methods
    }
}
