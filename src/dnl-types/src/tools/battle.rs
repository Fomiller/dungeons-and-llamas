use crate::dice::DiceExpression;
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
    pub enemy_health: DiceExpression,
    pub enemy_type: String,
    pub enemy_armor_class: u8,
    pub enemy_attack: BattleToolEnemyAttack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BattleToolEnemyAttack {
    pub attack_expression: DiceExpression,
    pub attack_name: String,
}

lazy_static! {
    pub static ref BATTLE_TOOL_SCHEMA: serde_json::Value = {
        let required = vec![
            "attack_expression",
            "attack_name",
            "current",
            "die_count",
            "die_size",
            "enemies",
            "enemy_armor_class",
            "enemy_attack",
            "enemy_health",
            "enemy_type",
            "health_expression",
            "max",
            "modifier",
            "name",
            "summary",
            "terrain",
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
                            "enemy_armor_class":{
                                "type": "number",
                                "description": "The armor value of the character."
                            },
                            "enemy_health":{
                                "type": "object",
                                "description": "An Object that defines an the amount of dice, the dice size, and modifiers that make up the expression, 1d6+2",
                                "properties": {
                                    "die_count": {
                                        "type": "number",
                                        "description": "number of dice from 1-12",
                                        "minimum": 1,
                                        "maximum": 12
                                    },
                                    "die_size": {
                                        "type": "number",
                                        "description": "size of the dice from 4-12",
                                        "minimum": 4,
                                        "maximum": 12
                                    },
                                    "modifier": {
                                        "type": "number",
                                        "description": "modifier to the expression from 0-12",
                                        "minimum": 0,
                                        "maximum": 12
                                    }
                                }
                            },
                            "enemy_attack":{
                                "type": "object",
                                "description": "An Object that defines an enemies attack",
                                "properties": {
                                    "attack_name": {
                                        "type": "string",
                                        "description": "Name of the attack"
                                    },
                                    "attack_expression": {
                                        "type": "object",
                                        "description": "An Object that defines an the amount of dice, the dice size, and modifiers that make up the expression, 1d6+2",
                                        "properties": {
                                            "die_count": {
                                                "type": "number",
                                                "description": "number of dice from 1-12",
                                                "minimum": 1,
                                                "maximum": 12
                                            },
                                            "die_size": {
                                                "type": "number",
                                                "description": "size of the dice from 4-12",
                                                "minimum": 4,
                                                "maximum": 12
                                            },
                                            "modifier": {
                                                "type": "number",
                                                "description": "modifier to the expression from 0-12",
                                                "minimum": 0,
                                                "maximum": 12
                                            }
                                        }
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
