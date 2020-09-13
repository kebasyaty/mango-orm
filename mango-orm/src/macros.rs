//! # Macros
//!
//!  Custom macros for mango models.

// MACROS ==========================================================================================
/// Macro for converting Structure to Model
#[macro_export]
macro_rules! create_model {
    ($service:expr, $database:expr, struct $sname:ident { $($fname:ident : $ftype:ty),* }) => {

        #[derive(Serialize, Deserialize, Debug, Default)]
        pub struct $sname {
            $(pub $fname : $ftype),*
        }

        impl $sname {
            // Get structure name
            pub fn struct_name() -> &'static str {
                stringify!($sname)
            }

            // Get array of field names
            pub fn field_names() -> &'static [&'static str] {
                &[$(stringify!($fname)),*]
            }

            // Metadata (database name, collection name, etc)
            pub fn meta() -> Meta {
                Meta {
                    database: $database.to_lowercase(),
                    collection: format!("{}_{}",
                        $service.to_lowercase(),
                        stringify!($sname).to_lowercase()
                    )
                }
            }

            // Checking Models and creating migrations to the Database.
            pub async fn migrat(_client: &Client) {
                let _meta: Meta = Self::meta();
                let attrs: HashMap<&'static str, Widget> = Self::raw_attrs();
                static STRUCT_NAME: &'static str = stringify!($sname);
                static FIELD_NAMES: &'static [&'static str] = &[$(stringify!($fname)),*];
                // Checking Widgets
                for (field, widget) in attrs {
                    // Checking for the correct field name
                    if !FIELD_NAMES.contains(&field) {
                        panic!(
                            "Service: `{}` -> Model: `{}` -> Field: `{}` - ???",
                            $service, STRUCT_NAME, field
                        )
                    }
                    // Checking the relationship of attribute states
                    match widget.field_type {
                        // InputCheckBox -----------------------------------------------------------
                        FieldType::InputCheckBox => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputCheckBox` -> `relation_model` = only blank string",
                                    $service, STRUCT_NAME
                                )
                            } else if widget.value != DataType::Bool(false) || widget.value != DataType::Bool(true) {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputCheckBox` -> `value` = only false or true",
                                    $service, STRUCT_NAME
                                )
                            } else if widget.maxlength != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputCheckBox` -> `maxlength` = only 0 (zero)",
                                    $service, STRUCT_NAME
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputCheckBox` -> `select` = only vec![]",
                                    $service, STRUCT_NAME
                                )
                            }
                        }
                        // InputColor --------------------------------------------------------------
                        FieldType::InputColor => {
                            if widget.relation_model != String::new() {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputColor` -> `relation_model` = only blank string",
                                    $service, STRUCT_NAME
                                )
                            } else if widget.select.len() != 0 {
                                panic!(
                                    "Service: `{}` -> Model: `{}` -> FieldType `InputColor` -> `select` = only vec![]",
                                    $service, STRUCT_NAME
                                )
                            }
                        }
                        // InputDate ---------------------------------------------------------------
                        FieldType::InputDate => {}
                        // InputEmail --------------------------------------------------------------
                        FieldType::InputEmail => {}
                        // InputFile ---------------------------------------------------------------
                        FieldType::InputFile => {}
                        // InputImage --------------------------------------------------------------
                        FieldType::InputImage => {}
                        // InputNumber -------------------------------------------------------------
                        FieldType::InputNumber => {}
                        // InputPassword -----------------------------------------------------------
                        FieldType::InputPassword => {}
                        // InputRadio --------------------------------------------------------------
                        FieldType::InputRadio => {}
                        // InputRange --------------------------------------------------------------
                        FieldType::InputRange => {}
                        // InputTel ----------------------------------------------------------------
                        FieldType::InputTel => {}
                        // InputText ---------------------------------------------------------------
                        FieldType::InputText => {}
                        // InputTime ---------------------------------------------------------------
                        FieldType::InputTime => {}
                        // InputUrl ----------------------------------------------------------------
                        FieldType::InputUrl => {}
                        // TextArea ----------------------------------------------------------------
                        FieldType::TextArea => {}
                        // Select ------------------------------------------------------------------
                        FieldType::Select => {}
                        // ForeignKey --------------------------------------------------------------
                        FieldType::ForeignKey => {}
                        // ManyToMany --------------------------------------------------------------
                        FieldType::ManyToMany => {}
                        // OneToOne ----------------------------------------------------------------
                        FieldType::OneToOne => {}
                        _ => panic!("Service: `{}` -> Model: `{}` -> Field: `{}` -> `field_type` - Non-existent field type.",
                        $service, STRUCT_NAME, field),
                    }
                }
            }
        }
    }
}

// TESTS ===========================================================================================
#[cfg(test)]
mod tests {
    //
}
