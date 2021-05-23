//! # Output data types for database queries
//!
//! `OutputDataOne` - To return results after processing queries for one document.
//! `OutputDataMany` - To return results after processing queries for many documents.
//!

// To return results after processing queries for one document.
// *************************************************************************************************
#[derive(Debug, Clone)]
pub enum OutputDataOne {
    Doc(
        (
            Option<mongodb::bson::document::Document>,
            Vec<String>,
            std::collections::HashMap<String, String>,
            String,
            String,
        ),
    ),
}

impl OutputDataOne {
    // Get raw document.
    // Hint: For non-standard operations.
    // ---------------------------------------------------------------------------------------------
    pub fn raw_doc(&self) -> mongodb::bson::document::Document {
        match self {
            Self::Doc(data) => {
                if data.0.is_some() {
                    data.0.clone().unwrap()
                } else {
                    mongodb::bson::document::Document::new()
                }
            }
        }
    }

    // Get prepared document.
    // Hint: For page template.
    // ---------------------------------------------------------------------------------------------
    pub fn doc(&self) -> Result<mongodb::bson::document::Document, Box<dyn std::error::Error>> {
        match self {
            Self::Doc(data) => {
                if data.0.is_some() {
                    Self::to_prepared_doc(
                        data.0.clone().unwrap(),
                        data.1.clone(),
                        data.2.clone(),
                        data.3.clone(),
                    )
                } else {
                    Ok(mongodb::bson::document::Document::new())
                }
            }
        }
    }

    // Get json-line.
    // Hint: For Ajax.
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Doc(data) => {
                if data.0.is_some() {
                    Ok(mongodb::bson::Bson::Document(Self::to_prepared_doc(
                        data.0.clone().unwrap(),
                        data.1.clone(),
                        data.2.clone(),
                        data.3.clone(),
                    )?)
                    .into_relaxed_extjson()
                    .to_string())
                } else {
                    Ok(String::from("{}"))
                }
            }
        }
    }

    // Get model instance.
    // Hint: For the `save`, `update`, `delete` operations.
    // ---------------------------------------------------------------------------------------------
    pub fn model<T>(&self) -> Result<T, mongodb::bson::de::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        match self {
            Self::Doc(data) => {
                if data.0.is_some() {
                    let doc = Self::to_prepared_doc(
                        data.0.clone().unwrap(),
                        data.1.clone(),
                        data.2.clone(),
                        data.3.clone(),
                    )
                    .unwrap();
                    let ignore_fields = data.1.clone();
                    let map_widget_type = data.2.clone();
                    let mut prepared_doc = mongodb::bson::document::Document::new();
                    let bson_null = &mongodb::bson::Bson::Null;
                    for (field_name, widget_type) in map_widget_type {
                        if ignore_fields.contains(&field_name) {
                            continue;
                        }
                        let bson_val = doc.get(field_name.as_str()).unwrap();
                        if widget_type == "inputFile" || widget_type == "inputImage" {
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    let result = serde_json::to_string(
                                        &bson_val.clone().into_relaxed_extjson(),
                                    )
                                    .unwrap();
                                    mongodb::bson::Bson::String(result)
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else {
                            prepared_doc.insert(field_name, bson_val);
                        }
                    }
                    mongodb::bson::de::from_document::<T>(prepared_doc)
                } else {
                    let prepared_doc = mongodb::bson::document::Document::new();
                    mongodb::bson::de::from_document::<T>(prepared_doc)
                }
            }
        }
    }

    // Get validation status (boolean)
    // Hint: For check document availability.
    // ---------------------------------------------------------------------------------------------
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Doc(data) => data.0.is_some(),
        }
    }

    // A description of the error if the document was not deleted.
    // (Main use for admin panel.)
    // ---------------------------------------------------------------------------------------------
    pub fn err_msg(&self) -> String {
        match self {
            Self::Doc(data) => data.4.clone(),
        }
    }

    // Get prepared doc.
    // Hint: Converting data types to model-friendly formats.
    // ---------------------------------------------------------------------------------------------
    pub fn to_prepared_doc(
        doc: mongodb::bson::document::Document,
        ignore_fields: Vec<String>,
        map_widget_type: std::collections::HashMap<String, String>,
        model_name: String,
    ) -> Result<mongodb::bson::document::Document, Box<dyn std::error::Error>> {
        let bson_null = &mongodb::bson::Bson::Null;
        let mut prepared_doc = mongodb::bson::document::Document::new();
        for (field_name, widget_type) in map_widget_type {
            if ignore_fields.contains(&field_name) {
                continue;
            }
            if field_name == "hash" {
                let bson_val = doc.get("_id").unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(bson_val.as_object_id().unwrap().to_hex())
                    } else {
                        Err(format!(
                            "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                Missing document identifier `_id`.",
                            model_name.clone()
                        ))?
                    },
                );
            } else if widget_type == "inputPassword" {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(String::new())
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDate" {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..10].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else if widget_type == "inputDateTime" {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(
                    field_name,
                    if bson_val != bson_null {
                        mongodb::bson::Bson::String(
                            bson_val.as_datetime().unwrap().to_rfc3339()[..16].into(),
                        )
                    } else {
                        mongodb::bson::Bson::Null
                    },
                );
            } else {
                let bson_val = doc.get(field_name.as_str()).unwrap();
                prepared_doc.insert(field_name, bson_val);
            }
        }

        Ok(prepared_doc)
    }
}

// To return results after processing queries for many documents.
// *************************************************************************************************
#[derive(Debug, Clone)]
pub enum OutputDataMany {
    Data(
        (
            Option<mongodb::bson::document::Document>,
            Option<mongodb::options::FindOptions>,
            mongodb::sync::Collection,
            Vec<String>,
            std::collections::HashMap<String, String>,
            String,
        ),
    ),
}

impl OutputDataMany {
    // Get raw documents.
    // Hint: For non-standard operations.
    // ---------------------------------------------------------------------------------------------
    pub fn raw_docs(
        &self,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        match self {
            Self::Data(data) => {
                let cursor = data.2.find(data.0.clone(), data.1.clone())?;
                Ok(cursor
                    .map(|item| item.unwrap())
                    .collect::<Vec<mongodb::bson::document::Document>>())
            }
        }
    }

    // Get prepared documents.
    // Hint: For page template.
    // ---------------------------------------------------------------------------------------------
    pub fn docs(
        &self,
    ) -> Result<Vec<mongodb::bson::document::Document>, Box<dyn std::error::Error>> {
        match self {
            Self::Data(data) => {
                let mut cursor = data.2.find(data.0.clone(), data.1.clone())?;
                let ignore_fields = data.3.clone();
                let bson_null = &mongodb::bson::Bson::Null;
                let mut docs: Vec<mongodb::bson::document::Document> = Vec::new();
                while let Some(doc) = cursor.next() {
                    let doc = doc?;
                    let map_widget_type = data.4.clone();
                    let mut prepared_doc = mongodb::bson::document::Document::new();
                    for (field_name, widget_type) in map_widget_type {
                        if ignore_fields.contains(&field_name) {
                            continue;
                        }
                        if field_name == "hash" {
                            let bson_val = doc.get("_id").unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_object_id().unwrap().to_hex(),
                                    )
                                } else {
                                    Err(format!(
                                        "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                        Missing document identifier `_id`.",
                                        data.5.clone()
                                    ))?
                                },
                            );
                        } else if widget_type == "inputPassword" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(String::new())
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else if widget_type == "inputDate" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_datetime().unwrap().to_rfc3339()[..10].into(),
                                    )
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else if widget_type == "inputDateTime" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_datetime().unwrap().to_rfc3339()[..16].into(),
                                    )
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(field_name, bson_val);
                        }
                    }
                    docs.push(prepared_doc);
                }

                Ok(docs)
            }
        }
    }

    // Get json-line.
    // Hint: For Ajax.
    // ---------------------------------------------------------------------------------------------
    pub fn json(&self) -> Result<String, Box<dyn std::error::Error>> {
        match self {
            Self::Data(data) => {
                let mut cursor = data.2.find(data.0.clone(), data.1.clone())?;
                let ignore_fields = data.3.clone();
                let bson_null = &mongodb::bson::Bson::Null;
                let mut json_line = String::new();
                while let Some(doc) = cursor.next() {
                    let doc = doc?;
                    let map_widget_type = data.4.clone();
                    let mut prepared_doc = mongodb::bson::document::Document::new();
                    for (field_name, widget_type) in map_widget_type {
                        if ignore_fields.contains(&field_name) {
                            continue;
                        }
                        if field_name == "hash" {
                            let bson_val = doc.get("_id").unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_object_id().unwrap().to_hex(),
                                    )
                                } else {
                                    Err(format!(
                                        "Model: `{}` > Field: `hash` > Method: `find_one()` : \
                                        Missing document identifier `_id`.",
                                        data.5.clone()
                                    ))?
                                },
                            );
                        } else if widget_type == "inputPassword" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(String::new())
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else if widget_type == "inputDate" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_datetime().unwrap().to_rfc3339()[..10].into(),
                                    )
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else if widget_type == "inputDateTime" {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(
                                field_name,
                                if bson_val != bson_null {
                                    mongodb::bson::Bson::String(
                                        bson_val.as_datetime().unwrap().to_rfc3339()[..16].into(),
                                    )
                                } else {
                                    mongodb::bson::Bson::Null
                                },
                            );
                        } else {
                            let bson_val = doc.get(field_name.as_str()).unwrap();
                            prepared_doc.insert(field_name, bson_val);
                        }
                    }

                    json_line = format!(
                        "{},{}",
                        json_line,
                        mongodb::bson::Bson::Document(prepared_doc)
                            .into_relaxed_extjson()
                            .to_string(),
                    );
                }

                Ok(format!(
                    "[{}]",
                    if !json_line.is_empty() {
                        &json_line[1..]
                    } else {
                        ""
                    }
                ))
            }
        }
    }

    // Get validation status (boolean)
    // Hint: For check documents availability.
    // ---------------------------------------------------------------------------------------------
    pub fn is_valid(&self) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(self.count()? > 0)
    }

    // Get the number of documents.
    // ---------------------------------------------------------------------------------------------
    pub fn count(&self) -> mongodb::error::Result<i64> {
        match self {
            Self::Data(data) => {
                let find_options = data.1.clone().unwrap();
                let mut options = mongodb::options::CountOptions::default();
                options.hint = find_options.hint;
                options.limit = find_options.limit;
                options.max_time = find_options.max_time;
                options.skip = find_options.skip;
                options.collation = find_options.collation;
                data.2.count_documents(data.0.clone(), Some(options))
            }
        }
    }
}
