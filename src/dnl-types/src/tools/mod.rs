pub mod battle;
pub mod random_weapon;
pub mod rest;
pub mod shop;

use crate::dice::DiceExpression;
use crate::scenarios::battle::BattleScenarioEnemy;
use crate::scenarios::shop::*;
use crate::scenarios::*;
use crate::tools::battle::*;
use crate::tools::shop::*;
use rest::REST_TOOL_SCHEMA;
use shop::SHOP_TOOL_SCHEMA;

use std::collections::HashMap;

use aws_sdk_bedrockruntime::types::Tool as BedRockTool;
use aws_sdk_bedrockruntime::types::*;
use aws_smithy_types::Document;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub trait MockData {
    fn mock(self) -> Self;
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, strum::EnumString, strum::Display)]
pub enum Tool {
    #[strum(to_string = "battle")]
    Battle,
    #[strum(to_string = "shop")]
    Shop,
    #[strum(to_string = "rest")]
    Rest,
    // #[strum(to_string = "weapon")]
    // Weapon,
    // #[strum(to_string = "spell")]
    // Spell,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolInput {
    pub tool: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, strum::EnumString, strum::Display)]
#[serde(untagged)]
pub enum ToolOutput {
    #[strum(to_string = "battle")]
    Battle {
        enemies: Vec<BattleToolEnemy>,
        summary: String,
        name: String,
        terrain: String,
    },
    #[strum(to_string = "shop")]
    Shop {
        items: Vec<ShopToolItem>,
        merchant: ShopToolMerchant,
        summary: String,
    },
    #[strum(to_string = "rest")]
    Rest {
        summary: String,
        flora: String,
        fauna: String,
        secret: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[serde(untagged)]
pub enum ToolOutputModel {
    #[strum(to_string = "battle")]
    Battle {
        enemies: Vec<BattleScenarioEnemy>,
        summary: String,
        name: String,
        terrain: String,
    },
    #[strum(to_string = "shop")]
    Shop {
        items: Vec<ShopScenarioItem>,
        merchant: ShopScenarioMerchant,
        summary: String,
    },
    #[strum(to_string = "rest")]
    Rest {
        summary: String,
        flora: String,
        fauna: String,
        secret: Option<String>,
    },
}

impl From<ToolOutput> for ToolOutputModel {
    fn from(source: ToolOutput) -> Self {
        match source {
            ToolOutput::Battle {
                enemies,
                summary,
                name,
                terrain,
            } => ToolOutputModel::Battle {
                enemies: enemies.into_iter().map(Into::into).collect(),
                summary,
                name,
                terrain,
            },
            ToolOutput::Shop {
                items,
                merchant,
                summary,
            } => ToolOutputModel::Shop {
                items: items.into_iter().map(Into::into).collect(),
                merchant: ShopScenarioMerchant::from(merchant),
                summary,
            },
            ToolOutput::Rest {
                summary,
                flora,
                fauna,
                secret,
            } => ToolOutputModel::Rest {
                summary,
                flora,
                fauna,
                secret,
            },
        }
    }
}

impl Tool {
    pub fn description(&self) -> &str {
        match self {
            Tool::Battle=> "Creates the Battle scenario for a Dungeons and Dragons-style text adventure, in a JSON format.",
            Tool::Shop=> "Defines a shopping scenario where players can purchase items, in a JSON format.",
            Tool::Rest=> "Allows players to rest and regain health, in a JSON format.",
        }
    }

    pub fn schema(&self) -> Value {
        match self {
            Tool::Battle => BATTLE_TOOL_SCHEMA.clone(),
            Tool::Shop => SHOP_TOOL_SCHEMA.clone(),
            Tool::Rest => REST_TOOL_SCHEMA.clone(),
        }
    }

    pub fn schema_as_document(&self) -> anyhow::Result<Document> {
        Ok(ToolJsonSchema::new(self.schema())?.to_document())
    }

    pub fn config(&self) -> anyhow::Result<ToolConfiguration> {
        let name = &self.to_string();
        let description = self.description();
        let document = self.schema_as_document()?;

        let input_schema = ToolInputSchema::Json(document);

        let tool_spec = ToolSpecification::builder()
            .name(name)
            .description(description)
            .input_schema(input_schema)
            .build()?;

        let tool = BedRockTool::ToolSpec(tool_spec);

        let tool_choice = ToolChoice::Tool(SpecificToolChoice::builder().name(name).build()?);

        let config = ToolConfiguration::builder()
            .tools(tool)
            .tool_choice(tool_choice)
            .build()?;

        Ok(config)
    }
}

impl ToolOutput {
    pub fn mock_battle() -> ToolOutput {
        let sword = BattleToolEnemyAttack {
            attack_expression: DiceExpression {
                die_count: 1,
                die_size: 6,
                modifier: 2,
            },
            attack_name: "Rusted Sword".to_string(),
        };
        let bow = BattleToolEnemyAttack {
            attack_expression: DiceExpression {
                die_count: 1,
                die_size: 6,
                modifier: 2,
            },
            attack_name: "ShortBow".to_string(),
        };
        let health_expression = DiceExpression {
            die_count: 1,
            die_size: 6,
            modifier: 2,
        };
        let health_expression_2 = DiceExpression {
            die_count: 2,
            die_size: 8,
            modifier: 0,
        };

        let enemies = vec![
            BattleToolEnemy {
                enemy_health: health_expression,
                enemy_type: "Goblin".to_string(),
                enemy_armor_class: 10,
                enemy_attack: sword,
            },
            BattleToolEnemy {
                enemy_health: health_expression_2,
                enemy_type: "Goblin Archer".to_string(),
                enemy_armor_class: 12,
                enemy_attack: bow,
            },
        ];

        let summary = "summary of the scenario".to_string();
        let name = "a dangerous encounter".to_string();
        let terrain = "description of the terrain".to_string();

        ToolOutput::Battle {
            enemies,
            summary,
            name,
            terrain,
        }
    }
    fn mock_rest() -> ToolOutput {
        let summary = "This is a nice place to rest".to_string();
        let flora = "plants".to_string();
        let fauna = "animals".to_string();
        let secret = None;

        let rest = ToolOutput::Rest {
            summary,
            flora,
            fauna,
            secret,
        };

        rest
    }
    fn mock_shop() -> ToolOutput {
        let stats = DiceExpression {
            die_count: 1,
            die_size: 6,
            modifier: 2,
        };

        let sword = ShopToolItem {
            item_name: "Magic Sword".to_string(),
            item_description: "A glowing sword.".to_string(),
            item_stats: stats.clone(),
            item_price: "3 gold".to_string(),
        };

        let bow = ShopToolItem {
            item_name: "Short Bow".to_string(),
            item_description: "A short bow, it seems to have some intricate carvings".to_string(),
            item_stats: stats.clone(),
            item_price: "2 gold".to_string(),
        };

        let potion = ShopToolItem {
            item_name: "Health Potion".to_string(),
            item_description: "Restores Health".to_string(),
            item_stats: stats.clone(),
            item_price: "5 gold".to_string(),
        };

        let items = vec![sword, bow, potion];

        let merchant = ShopToolMerchant { merchant_name: "Dirty Dan".to_string(), merchant_description: "A dirty old man who is missing teeth, but you can see a cart full of treasure behind him".to_string()};

        let summary = "This is summary Text".to_string();

        let shop = ToolOutput::Shop {
            merchant,
            items,
            summary,
        };

        shop
    }
}

impl From<ToolOutput> for ScenarioModel {
    fn from(source: ToolOutput) -> Self {
        match source {
            ToolOutput::Battle {
                enemies,
                summary,
                name,
                terrain,
            } => ScenarioModel::Battle {
                enemies: enemies.into_iter().map(Into::into).collect(),
                summary,
                name,
                terrain,
            },
            ToolOutput::Shop {
                items,
                merchant,
                summary,
            } => ScenarioModel::Shop {
                items: items.into_iter().map(Into::into).collect(),
                merchant: ShopScenarioMerchant::from(merchant),
                summary,
            },
            ToolOutput::Rest {
                summary,
                flora,
                fauna,
                secret,
            } => ScenarioModel::Rest {
                summary,
                flora,
                fauna,
                secret,
            },
        }
    }
}

impl MockData for ToolOutput {
    fn mock(self) -> Self {
        match self {
            ToolOutput::Battle {
                enemies: _,
                summary: _,
                name: _,
                terrain: _,
            } => ToolOutput::mock_battle(),
            ToolOutput::Shop {
                items: _,
                merchant: _,
                summary: _,
            } => ToolOutput::mock_shop(),
            ToolOutput::Rest {
                summary: _,
                flora: _,
                fauna: _,
                secret: _,
            } => ToolOutput::mock_rest(),
        }
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
