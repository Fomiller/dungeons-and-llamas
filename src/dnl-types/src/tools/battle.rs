use super::MockData;
use crate::traits::DiscordMsg;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolOutput {
    pub enemies: Vec<BattleToolEnemy>,
    pub summary: String,
    pub name: String,
    pub terrain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemy {
    pub health: Health,
    pub enemy_type: String,
    pub armor_class: u8,
    pub attack: BattleToolEnemyAttack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemyAttack {
    pub attack_damage: String,
    pub attack_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: u8,
    pub max: u8,
    pub expression: HealthExpression,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthExpression {
    pub die_count: u8,
    pub die_size: u8,
    pub modifier: u8,
}

impl std::fmt::Display for HealthExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut expression = format!("{}d{}", self.die_count, self.die_size);
        if self.modifier != 0 {
            let mod_expression = format!("+{}", self.modifier);
            expression.push_str(&mod_expression)
        }
        write!(f, "{}", expression)
    }
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
        let health_expression = HealthExpression {
            die_count: 1,
            die_size: 6,
            modifier: 2,
        };
        let health = Health {
            current: 8,
            max: 8,
            expression: health_expression.clone(),
        };
        let enemies = vec![
            BattleToolEnemy {
                health: health.clone(),
                enemy_type: "Goblin".to_string(),
                armor_class: 10,
                attack: sword,
            },
            BattleToolEnemy {
                health: health,
                enemy_type: "Goblin Archer".to_string(),
                armor_class: 10,
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

impl DiscordMsg for BattleToolOutput {
    fn to_message(&self) -> String {
        let mut enemy_description = String::new();

        for enemy in &self.enemies {
            let description = format!(
                "- **{}**\n  - Attack: {}\n  - Damage: {}\n  - Armor: {}\n  - Health: {}\n  - Current: {}\n  - Max: {}\n",
                enemy.enemy_type,
                enemy.attack.attack_name,
                enemy.attack.attack_damage,
                enemy.armor_class,
                enemy.health.expression,
                enemy.health.current,
                enemy.health.max
            );
            enemy_description.push_str(&description);
        }

        format!(
            "# *{}*\n## Description:\n{}\n\n## Terrain:\n{}\n\n## Enemies:\n{}\n\n## Summary:\n{}",
            self.name, self.summary, self.terrain, enemy_description, self.summary
        )
    }
}

lazy_static! {
    pub static ref BATTLE_TOOL_SCHEMA: serde_json::Value = {
        let required = vec![
            "name",
            "summary",
            "terrain",
            "enemies",
            "enemy_type",
            "armor_class",
            "health",
            "current",
            "max",
            "expression",
            "die_count",
            "die_size",
            "modifier",
            "attack",
            "attack_name",
            "attack_damage",
        ];

        serde_json::json!({
            "type": "object",
            "required": required,
            "properties":{
                "name": {
                    "type":"string",
                    "description":"A name for the battle encounter"
                },
                "summary":{
                    "type":"string",
                    "description":"A 30 to 50 word objective summary of the battle scenario. Make sure to include the number and types of enemies."
                },
                "terrain":{
                    "type":"string",
                    "description":"A description of the terrain the battle is happening in",
                },
                "enemies": {
                    "type": "array",
                    "description": "A list of enemies to fight",
                    "items": {
                        "type": "object",
                        "description": "An Object that defines an Enemy",
                        "properties": {
                            "enemy_type":{
                                "type": "string",
                                "description": "Type of enemy"
                            },
                            "armor_class":{
                                "type": "integer",
                                "description": "The armor value of the character."
                            },
                            "health":{
                                "type": "object",
                                "description": "An Object that defines an enemies health",
                                "properties": {
                                    "current": {
                                        "type": "integer",
                                        "description": "Current HP of enemy, this is always the same as max."
                                    },
                                    "max": {
                                        "type": "integer",
                                        "description": "Max HP of enemy, calculated through expression as (die_count)d(die_size)+(modifier)",
                                    },
                                    "expression": {
                                        "type": "object",
                                        "description": "An Object that defines an the amount of dice, the dice size, and modifiers that make up the enemies health expression, 1d6+2",
                                        "properties": {
                                            "die_count": {
                                                "type": "integer",
                                                "description": "number of dice from 1-12",
                                                "minimum": 1,
                                                "maximum": 12
                                            },
                                            "die_size": {
                                                "type": "integer",
                                                "description": "size of the dice from 4-12",
                                                "minimum": 4,
                                                "maximum": 12
                                            },
                                            "modifier": {
                                                "type": "integer",
                                                "description": "modifier to the health expression from 0-12",
                                                "minimum": 0,
                                                "maximum": 12
                                            }
                                        }
                                    }
                                }
                            },
                            "attack":{
                                "type": "object",
                                "description": "An Object that defines an enemies attack",
                                "properties": {
                                    "attack_name": {
                                        "type": "string",
                                        "description": "Name of the attack"
                                    },
                                    "attack_damage": {
                                        "type": "string",
                                        "description": "Damage value of attack as a integer value",
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    };
}
