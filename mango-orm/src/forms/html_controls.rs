//! # HtmlControls
//!
//! Rendering HTML-controls code for Form.
//! ( If necessary, customize the code generation yourself using html and css from Bootstrap, Material Design, etc. )
//!

use crate::forms::Widget;

pub trait HtmlControls {
    /// Rendering HTML-controls code for Form.
    /// ( If necessary, customize the code generation yourself using html and css from
    /// Bootstrap, Material Design, etc. )
    ///
    fn to_html(
        fields_name: &Vec<String>,
        map_widgets: std::collections::HashMap<String, Widget>,
    ) -> String {
        // Controls of Form.
        // -----------------------------------------------------------------------------------------
        let mut controls = String::new();
        for field_name in fields_name {
            let attrs = map_widgets.get(field_name).unwrap();
            // Messages common to the entire Form - Is required.
            // Hint: alternatively use in popup.
            if !attrs.common_msg.is_empty() {
                controls = format!("<p class=\"warning\">{}</p>{}", attrs.common_msg, controls);
            }
            match attrs.input_type.as_str() {
                "text" | "url" | "tel" | "password" | "email" | "color" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.pattern.is_empty() {
                            format!(" pattern=\"{}\"", attrs.pattern)
                        } else {
                            String::new()
                        },
                        if !attrs.minlength > 0 {
                            format!(" minlength=\"{}\"", attrs.minlength)
                        } else {
                            String::new()
                        },
                        if !attrs.maxlength > 0 {
                            format!(" maxlength=\"{}\"", attrs.maxlength)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "checkbox" => {
                    controls = format!(
                        "{}<p><input{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                        controls,
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.checked { " checked" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label>", attrs.id, attrs.label)
                        } else {
                            format!("<label for=\"{}\">{}:</label>", attrs.id, "Untitled")
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "radio" => {
                    let mut inputs = String::new();
                    for (idx, item) in attrs.options.iter().enumerate() {
                        inputs = format!(
                            "{}<p><input{}{}{}{}{}{}{}{}{}>{}{}{}{}</p>",
                            inputs,
                            format!(" id=\"{}\"-{}", attrs.id, idx),
                            format!(" type=\"{}\"", attrs.input_type),
                            format!(" name=\"{}\"", attrs.name),
                            format!(" value=\"{}\"", item.0),
                            if item.0 == attrs.value {
                                " checked"
                            } else {
                                ""
                            },
                            if attrs.disabled { " disabled" } else { "" },
                            if attrs.readonly { " readonly" } else { "" },
                            if !attrs.css_classes.is_empty() {
                                format!(" class=\"{}\"", attrs.css_classes)
                            } else {
                                String::new()
                            },
                            if !attrs.other_attrs.is_empty() {
                                format!(" {}", attrs.other_attrs)
                            } else {
                                String::new()
                            },
                            if !attrs.label.is_empty() {
                                format!("<label for=\"{}\">{}:</label>", attrs.id, attrs.label)
                            } else {
                                format!("<label for=\"{}\">{}:</label>", attrs.id, "Untitled")
                            },
                            if !attrs.hint.is_empty() {
                                format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                            } else {
                                String::new()
                            },
                            if !attrs.warning.is_empty() {
                                format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                            } else {
                                String::new()
                            },
                            if !attrs.error.is_empty() {
                                format!("<br><small class=\"error\">{}</small>", attrs.error)
                            } else {
                                String::new()
                            }
                        );
                    }
                    controls = format!("{}{}", controls, inputs);
                }
                "date" | "datetime" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.pattern.is_empty() {
                            format!(" pattern=\"{}\"", attrs.pattern)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\" ", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "file" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "number" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if attrs.step != "0" {
                            format!(" step=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.min != "0" {
                            format!(" min=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.max != "0" {
                            format!(" max=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "range" => {
                    controls = format!(
                        "{}<p>{}<input{}{}{}{}{}{}{}{}{}{}{}{}>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if attrs.step != "0" {
                            format!(" step=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.min != "0" {
                            format!(" min=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if attrs.max != "0" {
                            format!(" max=\"{}\"", attrs.step)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "textarea" => {
                    controls = format!(
                        "{}<p>{}<textarea{}{}{}{}{}{}{}{}{}{}>{}</textarea>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        format!(" name=\"{}\"", attrs.name),
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.minlength > 0 {
                            format!(" minlength=\"{}\"", attrs.minlength)
                        } else {
                            String::new()
                        },
                        if !attrs.maxlength > 0 {
                            format!(" maxlengt\"{}\"", attrs.maxlength)
                        } else {
                            String::new()
                        },
                        if !attrs.placeholder.is_empty() {
                            format!(" placeholder=\"{}\"", attrs.placeholder)
                        } else {
                            String::new()
                        },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        attrs.value,
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "select" => {
                    let mut options = String::new();
                    for item in attrs.options.iter() {
                        options = format!(
                            "{}<option{}value=\"{}\">{}</option>",
                            options,
                            if attrs.value == item.0 {
                                " selected "
                            } else {
                                " "
                            },
                            item.0,
                            item.1
                        );
                    }
                    controls = format!(
                        "{}<p>{}<select{}{}{}{}{}{}{}>{}</select>{}{}{}</p>",
                        controls,
                        if !attrs.label.is_empty() {
                            format!("<label for=\"{}\">{}:</label><br>", attrs.id, attrs.label)
                        } else {
                            String::new()
                        },
                        format!(" id=\"{}\"", attrs.id),
                        match attrs.widget.contains("Mult") {
                            true => format!(" name=\"{}[]\" multiple", attrs.name),
                            false => format!(" name=\"{}\"", attrs.name),
                        },
                        if attrs.required { " required" } else { "" },
                        if attrs.disabled { " disabled" } else { "" },
                        if attrs.readonly { " readonly" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        },
                        options,
                        if !attrs.hint.is_empty() {
                            format!("<br><small class=\"hint\">{}</small>", attrs.hint)
                        } else {
                            String::new()
                        },
                        if !attrs.warning.is_empty() {
                            format!("<br><small class=\"warning\">{}</small>", attrs.warning)
                        } else {
                            String::new()
                        },
                        if !attrs.error.is_empty() {
                            format!("<br><small class=\"error\">{}</small>", attrs.error)
                        } else {
                            String::new()
                        }
                    );
                }
                "hidden" => {
                    controls = format!(
                        "{}<input{}{}{}{}{}{}{}>",
                        controls,
                        format!(" id=\"{}\"", attrs.id),
                        format!(" type=\"{}\"", attrs.input_type),
                        format!(" name=\"{}\"", attrs.name),
                        format!(" value=\"{}\"", attrs.value),
                        if attrs.required { " required" } else { "" },
                        if !attrs.css_classes.is_empty() {
                            format!(" class=\"{}\"", attrs.css_classes)
                        } else {
                            String::new()
                        },
                        if !attrs.other_attrs.is_empty() {
                            format!(" {}", attrs.other_attrs)
                        } else {
                            String::new()
                        }
                    );
                }
                _ => panic!("Invalid input type."),
            }
        }
        // Add buttons and Return.
        // -----------------------------------------------------------------------------------------
        format!("{}<p><input type=\"submit\" value=\"Save\"></p>", controls)
    }

    // Get Html-line for `OutputDataForm`.
    // *********************************************************************************************
    fn html(&self) -> String {
        // Stub.
        String::new()
    }
}
