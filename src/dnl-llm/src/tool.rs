use super::schemas::*;
use aws_sdk_bedrockruntime::types::*;
use aws_smithy_types::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(thiserror::Error, Debug)]
pub enum ToolError {
    #[error("Unable to parse tool output to struct: {0}")]
    ParseOutput(String),
    #[error("No tool output available")]
    NoOutput,
}

pub trait ToDocument {
    fn to_document(&self) -> Document;
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

pub trait MockData {
    fn mock() -> Self;
}

impl MockData for BattleToolOutput {
    fn mock() -> Self {
        let sword = BattleToolEnemyAttack {
            attack_damage: "1d6+2".to_string(),
            attack_name: "Rusted Sword".to_string(),
        };
        let bow = BattleToolEnemyAttack {
            attack_damage: "1d6+2".to_string(),
            attack_name: "ShortBow".to_string(),
        };
        let enemies = vec![
            BattleToolEnemy {
                health: "1d6+8".to_string(),
                enemy_type: "Goblin".to_string(),
                attack: sword,
            },
            BattleToolEnemy {
                health: "1d6+8".to_string(),
                enemy_type: "Goblin Archer".to_string(),
                attack: bow,
            },
        ];

        let summary = "summary of the scenario".to_string();
        let name = "a dangerous encounter".to_string();
        let terrain = "description of the terrain".to_string();

        Self {
            enemies,
            summary,
            name,
            terrain,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleToolOutput {
    pub enemies: Vec<BattleToolEnemy>,
    pub summary: String,
    pub name: String,
    pub terrain: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShopToolOutput { }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RestToolOutput { }

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleToolEnemy {
    pub health: String,
    pub enemy_type: String,
    pub attack: BattleToolEnemyAttack,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BattleToolEnemyAttack {
    pub attack_damage: String,
    pub attack_name: String,
}

impl MockData for RestToolOutput {
    fn mock() -> Self { Self{} }
}

impl MockData for ShopToolOutput {
    fn mock() -> Self { Self{} }
}
