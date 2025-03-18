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
    pub health: String,
    pub enemy_type: String,
    pub attack: BattleToolEnemyAttack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemyAttack {
    pub attack_damage: String,
    pub attack_name: String,
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

impl DiscordMsg for BattleToolOutput {
    fn to_message(&self) -> String {
        let mut enemy_description = String::new();

        for enemy in &self.enemies {
            let description = format!(
                "- **{}**\n  - Attack: {}\n  - Damage: {}\n  - Health: {}\n",
                enemy.enemy_type,
                enemy.attack.attack_name,
                enemy.attack.attack_damage,
                enemy.health
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
        serde_json::json!({
            "type": "object",
            "required": ["name", "summary", "terrain", "enemies", "enemy_type", "health", "attack", "attack_name", "attack_damage"],
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
                            "health":{
                                "type": "integer",
                                "description": "Total health of the enemy",
                                "minimum": 1,
                                "maximum": 20
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
