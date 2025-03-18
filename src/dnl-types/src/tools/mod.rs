pub mod battle;
pub mod rest;
pub mod shop;

use crate::tools::battle::{BattleToolOutput, BATTLE_TOOL_SCHEMA};
use rest::{RestToolOutput, REST_TOOL_SCHEMA};
use shop::{ShopToolOutput, SHOP_TOOL_SCHEMA};
use std::collections::HashMap;

use aws_sdk_bedrockruntime::types::*;
use aws_smithy_types::Document;
use serde::{Deserialize, Serialize};
use serde::{Deserializer, Serializer};
use serde_json::Value;

pub trait MockData {
    fn mock() -> Self;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Tools {
    Battle,
    Shop,
    Rest,
}

impl Tools {
    pub fn name(&self) -> &str {
        match self {
            Tools::Battle => "battle",
            Tools::Shop => "shop",
            Tools::Rest => "rest",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Tools::Battle => "Creates the Battle scenario for a Dungeons and Dragons-style text adventure, in a JSON format.",
            Tools::Shop => "Defines a shopping scenario where players can purchase items, in a JSON format.",
            Tools::Rest => "Allows players to rest and regain health, in a JSON format.",
        }
    }

    pub fn schema(&self) -> Value {
        match self {
            Tools::Battle => BATTLE_TOOL_SCHEMA.clone(),
            Tools::Shop => SHOP_TOOL_SCHEMA.clone(),
            Tools::Rest => REST_TOOL_SCHEMA.clone(),
        }
    }

    pub fn schema_as_document(&self) -> anyhow::Result<Document> {
        Ok(ToolJsonSchema::new(self.schema())?.to_document())
    }

    pub fn config(&self) -> anyhow::Result<ToolConfiguration> {
        let name = self.name();
        let description = self.description();
        let document = self.schema_as_document()?;

        let input_schema = ToolInputSchema::Json(document);

        let tool_spec = ToolSpecification::builder()
            .name(name)
            .description(description)
            .input_schema(input_schema)
            .build()?;

        let tool = Tool::ToolSpec(tool_spec);

        let tool_choice = ToolChoice::Tool(SpecificToolChoice::builder().name(name).build()?);

        let config = ToolConfiguration::builder()
            .tools(tool)
            .tool_choice(tool_choice)
            .build()?;

        Ok(config)
    }
}

pub trait ToDocument {
    fn to_document(&self) -> Document;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolJsonSchema {
    r#type: String,
    pub properties: HashMap<String, Property>,
    pub required: Vec<String>,
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

#[derive(Clone, Debug)]
pub enum ToolOutputEnum {
    Battle(battle::BattleToolOutput),
    Shop(shop::ShopToolOutput),
    Rest(rest::RestToolOutput),
}

impl Serialize for ToolOutputEnum {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ToolOutputEnum::Battle(inner) => inner.serialize(serializer),
            ToolOutputEnum::Shop(inner) => inner.serialize(serializer),
            ToolOutputEnum::Rest(inner) => inner.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for ToolOutputEnum {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = serde_json::Value::deserialize(deserializer)?;

        if let Ok(inner) = serde_json::from_value::<BattleToolOutput>(value.clone()) {
            return Ok(ToolOutputEnum::Battle(inner));
        }
        if let Ok(inner) = serde_json::from_value::<ShopToolOutput>(value.clone()) {
            return Ok(ToolOutputEnum::Shop(inner));
        }
        if let Ok(inner) = serde_json::from_value::<RestToolOutput>(value.clone()) {
            return Ok(ToolOutputEnum::Rest(inner));
        }

        Err(serde::de::Error::custom("Unknown tool output type"))
    }
}
