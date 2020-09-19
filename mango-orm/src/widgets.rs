//! # Widgets
//!
//! Widgets for form elements.

use serde::Serialize;

// WIDGETS =========================================================================================
/// Field types for Widgets
#[derive(Debug, Clone, PartialEq)]
pub enum FieldType {
    InputCheckBox(bool),
    InputColor(String),
    InputDate(String),
    InputDateTime(String),
    InputEmail(String),
    InputFile,
    InputImage,
    InputNumberI32(i32),
    InputNumberU32(u32),
    InputNumberI64(i64),
    InputNumberF64(f64),
    InputPassword(String),
    InputRadio(bool),
    InputRangeI32(i32),
    InputRangeU32(u32),
    InputRangeI64(i64),
    InputRangeF64(f64),
    InputTel(String),
    InputText(String),
    InputUrl(String),
    TextArea(String),
    SelectText(String),
    SelectI32(i32),
    SelectU32(u32),
    SelectI64(i64),
    SelectF64(f64),
    ForeignKey,
    ManyToMany,
    OneToOne,
}

impl Default for FieldType {
    fn default() -> Self {
        FieldType::InputText(String::new())
    }
}

impl FieldType {
    pub fn input_type(&self) -> &'static str {
        match self {
            Self::InputCheckBox(_) => "checkbox",
            Self::InputColor(_) => "color",
            Self::InputDate(_) => "date",
            Self::InputDateTime(_) => "datetime",
            Self::InputEmail(_) => "email",
            Self::InputFile => "file",
            Self::InputImage => "image",
            Self::InputNumberI32(_) => "number",
            Self::InputNumberU32(_) => "number",
            Self::InputNumberI64(_) => "number",
            Self::InputNumberF64(_) => "number",
            Self::InputPassword(_) => "password",
            Self::InputRadio(_) => "radio",
            Self::InputRangeI32(_) => "range",
            Self::InputRangeU32(_) => "range",
            Self::InputRangeI64(_) => "range",
            Self::InputRangeF64(_) => "range",
            Self::InputTel(_) => "tel",
            Self::InputText(_) => "text",
            Self::InputUrl(_) => "url",
            Self::TextArea(_) => "textarea",
            Self::SelectText(_) => "select",
            Self::SelectI32(_) => "select",
            Self::SelectU32(_) => "select",
            Self::SelectI64(_) => "select",
            Self::SelectF64(_) => "select",
            Self::ForeignKey => "select",
            Self::ManyToMany => "select",
            Self::OneToOne => "hidden",
        }
    }

    pub fn raw_data(&self) -> String {
        match self {
            Self::InputCheckBox(data) => data.to_string(),
            Self::InputColor(data) => data.to_string(),
            Self::InputDate(data) => data.to_string(),
            Self::InputDateTime(data) => data.to_string(),
            Self::InputEmail(data) => data.to_string(),
            Self::InputFile => String::new(),
            Self::InputImage => String::new(),
            Self::InputNumberI32(data) => data.to_string(),
            Self::InputNumberU32(data) => data.to_string(),
            Self::InputNumberI64(data) => data.to_string(),
            Self::InputNumberF64(data) => data.to_string(),
            Self::InputPassword(data) => data.to_string(),
            Self::InputRadio(data) => data.to_string(),
            Self::InputRangeI32(data) => data.to_string(),
            Self::InputRangeU32(data) => data.to_string(),
            Self::InputRangeI64(data) => data.to_string(),
            Self::InputRangeF64(data) => data.to_string(),
            Self::InputTel(data) => data.to_string(),
            Self::InputText(data) => data.to_string(),
            Self::InputUrl(data) => data.to_string(),
            Self::TextArea(data) => data.to_string(),
            Self::SelectText(data) => data.to_string(),
            Self::SelectI32(data) => data.to_string(),
            Self::SelectU32(data) => data.to_string(),
            Self::SelectI64(data) => data.to_string(),
            Self::SelectF64(data) => data.to_string(),
            Self::ForeignKey => String::new(),
            Self::ManyToMany => String::new(),
            Self::OneToOne => String::new(),
        }
    }

    pub fn data_type(&self) -> &'static str {
        match self {
            Self::InputCheckBox(_) => "bool",
            Self::InputColor(_) => "string",
            Self::InputDate(_) => "string",
            Self::InputDateTime(_) => "string",
            Self::InputEmail(_) => "string",
            Self::InputFile => "none",
            Self::InputImage => "none",
            Self::InputNumberI32(_) => "i32",
            Self::InputNumberU32(_) => "u32",
            Self::InputNumberI64(_) => "i64",
            Self::InputNumberF64(_) => "f64",
            Self::InputPassword(_) => "string",
            Self::InputRadio(_) => "bool",
            Self::InputRangeI32(_) => "i32",
            Self::InputRangeU32(_) => "u32",
            Self::InputRangeI64(_) => "i64",
            Self::InputRangeF64(_) => "f64",
            Self::InputTel(_) => "string",
            Self::InputText(_) => "string",
            Self::InputUrl(_) => "string",
            Self::TextArea(_) => "string",
            Self::SelectText(_) => "string",
            Self::SelectI32(_) => "i32",
            Self::SelectU32(_) => "u32",
            Self::SelectI64(_) => "i64",
            Self::SelectF64(_) => "f64",
            Self::ForeignKey => "none",
            Self::ManyToMany => "none",
            Self::OneToOne => "none",
        }
    }
}

/// Data types for the `value` attribute -----------------------------------------------------------
#[derive(Debug, Clone)]
pub enum SelectDataType {
    Text(String),
    I32(i32),
    U32(u32),
    I64(i64),
    F64(f64),
}

impl Default for SelectDataType {
    fn default() -> Self {
        SelectDataType::Text(String::new())
    }
}

impl SelectDataType {
    pub fn raw_data(&self) -> String {
        match self {
            Self::Text(data) => data.to_owned(),
            Self::I64(data) => data.to_string(),
            Self::I32(data) => data.to_string(),
            Self::U32(data) => data.to_string(),
            Self::F64(data) => data.to_string(),
        }
    }

    pub fn data_type(&self) -> &'static str {
        match self {
            Self::Text(_) => "string",
            Self::I64(_) => "i64",
            Self::I32(_) => "i32",
            Self::U32(_) => "u32",
            Self::F64(_) => "f64",
        }
    }
}

/// Mediator for transporting widget attributes ----------------------------------------------------
#[derive(Serialize, Debug, Default)]
pub struct Transport {
    pub id: String, // "id-name" or auto
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub value: String,
    pub maxlength: u32,
    pub required: bool,
    pub checked: bool, // For <input type="checkbox|radio">
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,  // "autofocus step=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, String)>,
}

/// Attributes for the widget ----------------------------------------------------------------------
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
#[derive(Debug)]
pub struct Widget {
    pub label: String,
    pub relation_model: String,
    pub value: FieldType,
    pub maxlength: u32,
    pub required: bool,
    pub hint: String,
    pub unique: bool,
    pub hidden: bool,
    pub other_attrs: String,  // "autofocus step=\"число\" ..."
    pub some_classes: String, // "class-name class-name ..."
    pub select: Vec<(String, SelectDataType)>,
}

impl Default for Widget {
    fn default() -> Self {
        Widget {
            label: String::new(),
            relation_model: String::new(),
            value: FieldType::default(),
            maxlength: 0_u32,
            required: true,
            hint: String::new(),
            unique: false,
            hidden: false,
            other_attrs: String::new(),
            some_classes: String::new(),
            select: vec![],
        }
    }
}

impl Widget {
    // Get pure attributes from a widget
    pub fn clean_attrs(&self, name: &str) -> Transport {
        let field_type = match self.hidden {
            true => "hidden".to_string(),
            false => self.value.input_type().to_string(),
        };
        let checked = match self.value {
            FieldType::InputCheckBox(data) => data,
            FieldType::InputRadio(data) => data,
            _ => false,
        };
        let other_attrs = match self.value {
            FieldType::ManyToMany => match self.other_attrs.contains("multiple") {
                true => self.other_attrs.clone(),
                false => format!("multiple {}", self.other_attrs),
            },
            _ => self.other_attrs.clone(),
        };

        Transport {
            id: name.to_string(),
            label: self.label.clone(),
            field_type: field_type,
            name: name.to_string(),
            value: self.value.raw_data(),
            maxlength: self.maxlength.clone(),
            required: self.required.clone(),
            checked: checked,
            hint: self.hint.clone(),
            unique: self.unique.clone(),
            hidden: self.hidden.clone(),
            other_attrs: other_attrs,
            some_classes: self.some_classes.clone(),
            select: self
                .select
                .iter()
                .map(|item| (item.0.clone(), item.1.raw_data()))
                .collect::<Vec<(String, String)>>(),
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // Testing enum FieldTypes ---------------------------------------------------------------------
    #[test]
    fn test_field_types() {
        // Method input_type()
        assert_eq!(FieldType::InputCheckBox(false).input_type(), "checkbox");
        assert_eq!(FieldType::InputColor(String::new()).input_type(), "color");
        assert_eq!(FieldType::InputDate(String::new()).input_type(), "date");
        assert_eq!(
            FieldType::InputDateTime(String::new()).input_type(),
            "datetime"
        );
        assert_eq!(FieldType::InputEmail(String::new()).input_type(), "email");
        assert_eq!(FieldType::InputFile.input_type(), "file");
        assert_eq!(FieldType::InputImage.input_type(), "image");
        assert_eq!(FieldType::InputNumberI32(-1_i32).input_type(), "number");
        assert_eq!(FieldType::InputNumberU32(0_u32).input_type(), "number");
        assert_eq!(FieldType::InputNumberI64(-1_i64).input_type(), "number");
        assert_eq!(FieldType::InputNumberF64(-1.3_f64).input_type(), "number");
        assert_eq!(
            FieldType::InputPassword(String::new()).input_type(),
            "password"
        );
        assert_eq!(FieldType::InputRadio(false).input_type(), "radio");
        assert_eq!(FieldType::InputRangeI32(-1_i32).input_type(), "range");
        assert_eq!(FieldType::InputRangeU32(0_u32).input_type(), "range");
        assert_eq!(FieldType::InputRangeI64(-1_i64).input_type(), "range");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).input_type(), "range");
        assert_eq!(FieldType::InputTel(String::new()).input_type(), "tel");
        assert_eq!(FieldType::InputText(String::new()).input_type(), "text");
        assert_eq!(FieldType::InputUrl(String::new()).input_type(), "url");
        assert_eq!(FieldType::TextArea(String::new()).input_type(), "textarea");
        assert_eq!(FieldType::SelectText(String::new()).input_type(), "select");
        assert_eq!(FieldType::SelectI32(-1_i32).input_type(), "select");
        assert_eq!(FieldType::SelectU32(0_u32).input_type(), "select");
        assert_eq!(FieldType::SelectI64(-1_i64).input_type(), "select");
        assert_eq!(FieldType::SelectF64(-1.3_f64).input_type(), "select");
        assert_eq!(FieldType::ForeignKey.input_type(), "select");
        assert_eq!(FieldType::ManyToMany.input_type(), "select");
        assert_eq!(FieldType::OneToOne.input_type(), "hidden");

        // Method raw_data()
        assert_eq!(FieldType::InputCheckBox(false).raw_data(), "false");
        assert_eq!(
            FieldType::InputColor(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputDate(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputDateTime(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(
            FieldType::InputEmail(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(FieldType::InputFile.raw_data(), String::new());
        assert_eq!(FieldType::InputImage.raw_data(), String::new());
        assert_eq!(FieldType::InputNumberI32(-1_i32).raw_data(), "-1");
        assert_eq!(FieldType::InputNumberU32(0_u32).raw_data(), "0");
        assert_eq!(FieldType::InputNumberI64(-1_i64).raw_data(), "-1");
        assert_eq!(FieldType::InputNumberF64(-1.3_f64).raw_data(), "-1.3");
        assert_eq!(
            FieldType::InputPassword(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(FieldType::InputRadio(false).raw_data(), "false");
        assert_eq!(FieldType::InputRangeI32(-1_i32).raw_data(), "-1");
        assert_eq!(FieldType::InputRangeU32(0_u32).raw_data(), "0");
        assert_eq!(FieldType::InputRangeI64(-1_i64).raw_data(), "-1");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).raw_data(), "-1.3");
        assert_eq!(FieldType::InputTel(String::new()).raw_data(), String::new());
        assert_eq!(
            FieldType::InputText(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(FieldType::InputUrl(String::new()).raw_data(), String::new());
        assert_eq!(FieldType::TextArea(String::new()).raw_data(), String::new());
        assert_eq!(
            FieldType::SelectText(String::new()).raw_data(),
            String::new()
        );
        assert_eq!(FieldType::SelectI32(-1_i32).raw_data(), "-1");
        assert_eq!(FieldType::SelectU32(0_u32).raw_data(), "0");
        assert_eq!(FieldType::SelectI64(-1_i64).raw_data(), "-1");
        assert_eq!(FieldType::SelectF64(-1.3_f64).raw_data(), "-1.3");
        assert_eq!(FieldType::ForeignKey.raw_data(), String::new());
        assert_eq!(FieldType::ManyToMany.raw_data(), String::new());
        assert_eq!(FieldType::OneToOne.raw_data(), String::new());

        // Method data_type()
        assert_eq!(FieldType::InputCheckBox(false).data_type(), "bool");
        assert_eq!(FieldType::InputColor(String::new()).data_type(), "string");
        assert_eq!(FieldType::InputDate(String::new()).data_type(), "string");
        assert_eq!(
            FieldType::InputDateTime(String::new()).data_type(),
            "string"
        );
        assert_eq!(FieldType::InputEmail(String::new()).data_type(), "string");
        assert_eq!(FieldType::InputFile.input_type(), "none");
        assert_eq!(FieldType::InputImage.input_type(), "none");
        assert_eq!(FieldType::InputNumberI32(-1_i32).data_type(), "i32");
        assert_eq!(FieldType::InputNumberU32(0_u32).data_type(), "u32");
        assert_eq!(FieldType::InputNumberI64(-1_i64).data_type(), "i64");
        assert_eq!(FieldType::InputNumberF64(-1.3_f64).data_type(), "f64");
        assert_eq!(
            FieldType::InputPassword(String::new()).data_type(),
            "string"
        );
        assert_eq!(FieldType::InputRadio(false).data_type(), "bool");
        assert_eq!(FieldType::InputRangeI32(-1_i32).data_type(), "i32");
        assert_eq!(FieldType::InputRangeU32(0_u32).data_type(), "u32");
        assert_eq!(FieldType::InputRangeI64(-1_i64).data_type(), "i64");
        assert_eq!(FieldType::InputRangeF64(-1.3_f64).data_type(), "f64");
        assert_eq!(FieldType::InputTel(String::new()).data_type(), "string");
        assert_eq!(FieldType::InputText(String::new()).data_type(), "string");
        assert_eq!(FieldType::InputUrl(String::new()).data_type(), "string");
        assert_eq!(FieldType::TextArea(String::new()).data_type(), "string");
        assert_eq!(FieldType::SelectText(String::new()).data_type(), "string");
        assert_eq!(FieldType::SelectI32(-1_i32).data_type(), "i32");
        assert_eq!(FieldType::SelectU32(0_u32).data_type(), "u32");
        assert_eq!(FieldType::SelectI64(-1_i64).data_type(), "i64");
        assert_eq!(FieldType::SelectF64(-1.3_f64).data_type(), "f64");
        assert_eq!(FieldType::ForeignKey.data_type(), "none");
        assert_eq!(FieldType::ManyToMany.data_type(), "none");
        assert_eq!(FieldType::OneToOne.data_type(), "none");
    }

    // Testing Data types --------------------------------------------------------------------------
    #[test]
    fn test_data_types() {
        assert_eq!(
            SelectDataType::Text("Some text".to_string()).raw_data(),
            "Some text".to_string()
        );
        assert_eq!(
            SelectDataType::I64(-10_i64).raw_data(),
            (-10_i64).to_string()
        );
        assert_eq!(
            SelectDataType::I32(-10_i32).raw_data(),
            (-10_i32).to_string()
        );
        assert_eq!(SelectDataType::U32(10_u32).raw_data(), 10_u32.to_string());
        assert_eq!(
            SelectDataType::F64(-10_f64).raw_data(),
            (-10_f64).to_string()
        );
    }

    // Testing Transport structure -----------------------------------------------------------------
    #[test]
    fn test_transport() {
        let trans: Transport = Default::default();
        // Fields
        assert_eq!(trans.id, String::new());
        assert_eq!(trans.label, String::new());
        assert_eq!(trans.field_type, String::new());
        assert_eq!(trans.name, String::new());
        assert_eq!(trans.value, String::new());
        assert_eq!(trans.maxlength, 0);
        assert_eq!(trans.required, false);
        assert_eq!(trans.checked, false);
        assert_eq!(trans.hint, String::new());
        assert_eq!(trans.unique, false);
        assert_eq!(trans.hidden, false);
        assert_eq!(trans.other_attrs, String::new());
        assert_eq!(trans.some_classes, String::new());
        assert_eq!(trans.select, vec![]);
        // Methods
    }

    // Testing Widget structure --------------------------------------------------------------------
    #[test]
    fn test_widget() {
        let mut widget: Widget = Default::default();
        widget.select = vec![(String::new(), SelectDataType::Text(String::new()))];
        // Fields
        assert_eq!(widget.label, String::new());
        assert_eq!(
            widget.value.input_type(),
            FieldType::InputText(String::new()).input_type()
        );
        assert_eq!(widget.relation_model, String::new());
        assert_eq!(widget.maxlength, 0);
        assert_eq!(widget.required, true);
        assert_eq!(widget.hint, String::new());
        assert_eq!(widget.unique, false);
        assert_eq!(widget.hidden, false);
        assert_eq!(widget.other_attrs, String::new());
        assert_eq!(widget.some_classes, String::new());
        assert_eq!(widget.select[0].0, String::new());
        assert_eq!(widget.select[0].1.raw_data(), String::new());
        // Methods
        let mut attrs = widget.clean_attrs("");
        attrs.select = vec![(
            String::new(),
            SelectDataType::Text(String::new()).raw_data(),
        )];

        assert_eq!(attrs.id, String::new());
        assert_eq!(attrs.label, String::new());
        assert_eq!(attrs.field_type, "text".to_string());
        assert_eq!(attrs.name, String::new());
        assert_eq!(attrs.value, String::new());
        assert_eq!(attrs.maxlength, 0);
        assert_eq!(attrs.required, true);
        assert_eq!(attrs.checked, false);
        assert_eq!(attrs.hint, String::new());
        assert_eq!(attrs.unique, false);
        assert_eq!(attrs.hidden, false);
        assert_eq!(attrs.other_attrs, String::new());
        assert_eq!(attrs.some_classes, String::new());
        assert_eq!(attrs.select[0].0, String::new());
        assert_eq!(attrs.select[0].1, String::new());
    }
}
