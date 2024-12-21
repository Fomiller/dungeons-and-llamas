use aws_smithy_types::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

pub trait ToDocument {
    fn to_document(&self) -> Document;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolJsonSchema {
    r#type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Property {
    pub r#type: String,
    pub description: String,
}

impl ToDocument for Property {
    fn to_document(&self) -> Document {
        Document::Object(HashMap::from([
            ("type".to_owned(), Document::String(self.r#type.to_owned())),
            (
                "description".to_owned(),
                Document::String(self.description.to_owned()),
            ),
        ]))
    }
}

impl ToolJsonSchema {
    pub fn new(json: Value) -> anyhow::Result<Self> {
        let schema: ToolJsonSchema = serde_json::from_value(json)?;
        Ok(schema)
    }
}

impl ToDocument for ToolJsonSchema {
    fn to_document(&self) -> Document {
        let r#type = Document::String(self.r#type.to_owned());

        let properties: HashMap<String, Document> = self
            .properties
            .to_owned()
            .iter()
            .map(|(k, v)| (k.to_owned(), v.to_document()))
            .collect();

        let required: Vec<Document> = self
            .required
            .to_owned()
            .iter()
            .map(|r| Document::String(r.to_owned()))
            .collect();

        Document::Object(HashMap::<String, Document>::from([
            ("type".to_owned(), r#type),
            ("properties".to_owned(), Document::Object(properties)),
            ("required".to_owned(), Document::Array(required)),
        ]))
    }
}
