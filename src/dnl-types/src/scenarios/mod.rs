pub mod battle;
pub mod new_game;
pub mod rest;
pub mod shop;

use battle::*;
use shop::*;

use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Scenario {
    #[strum(to_string = "battle")]
    Battle,
    #[strum(to_string = "shop")]
    Shop,
    #[strum(to_string = "rest")]
    Rest,
    #[strum(to_string = "new-game")]
    NewGame,
}

#[derive(Debug, Clone, Deserialize, Serialize, strum::Display, strum::EnumString)]
#[strum(ascii_case_insensitive)]
#[serde(untagged)]
pub enum ScenarioModel {
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
    #[strum(to_string = "new-game")]
    NewGame {
        summary: String,
        name: String,
        items: Vec<ShopScenarioItem>,
    },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScenarioInput {
    pub scenario: String,
}

impl DiscordMsg for ScenarioModel {
    fn to_message(&self) -> String {
        match self {
            ScenarioModel::Battle {
                enemies,
                summary,
                name,
                terrain,
            } => {
                let mut enemy_description = String::new();

                for enemy in enemies {
                    let description = format!(
                        "- **{}**\n  - Id: {}\n  - Attack: {}\n  - Damage: {}\n  - Armor: {}\n  - Health: {}\n  - Current: {}\n  - Max: {}\n",
                        enemy.r#type,
                        enemy.id,
                        enemy.attacks.name,
                        enemy.attacks.expression,
                        enemy.armor_class,
                        enemy.health.expression,
                        enemy.health.current,
                        enemy.health.max
                    );
                    enemy_description.push_str(&description);
                }

                format!(
                    "# *{}*\n## Description:\n{}\n\n## Terrain:\n{}\n\n## Enemies:\n{}\n\n## Summary:\n{}",
                    name, summary, terrain, enemy_description, summary
                )
            }
            ScenarioModel::Shop {
                items,
                merchant,
                summary,
            } => {
                let mut item_descriptions = String::new();

                for item in items {
                    let description = format!(
                        "**{}**\n{}\n - Stats: {}\n - Price: {}\n\n",
                        item.name, item.description, item.stats, item.price
                    );
                    item_descriptions.push_str(&description);
                }

                format!(
                    "*{}*\n# {}\n*{}*\n## Items:\n{}",
                    summary, merchant.name, merchant.description, item_descriptions
                )
            }
            ScenarioModel::Rest {
                summary,
                flora,
                fauna,
                secret,
            } => {
                let mut message = format!("*{}*\n\n*{}*\n\n*{}*", summary, flora, fauna);

                if let Some(secret) = secret {
                    message.push_str(&format!("\n\n||{}||", secret));
                    return message;
                };

                message
            }
            ScenarioModel::NewGame {
                summary,
                name,
                items,
            } => {
                let mut item_descriptions = String::new();

                for item in items {
                    let description = format!(
                        "**{}**\n{}\n - Stats: {}\n - Price: {}\n\n",
                        item.name, item.description, item.stats, item.price
                    );
                    item_descriptions.push_str(&description);
                }
                format!(
                    "# *{}*\n## Description:\n{}\n## Choose a Weapon:\n{}",
                    name, summary, item_descriptions
                )
            }
        }
    }
}
