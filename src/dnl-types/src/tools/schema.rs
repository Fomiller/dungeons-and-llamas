use lazy_static::lazy_static;
use serde_json::{json, Value};

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
    pub static ref REST_TOOL_SCHEMA: Value = json!({
        "type": "object",
        "required": ["name", "summary", "terrain", "enemies", "enemy_type", "health", "attack_name", "attack_damage"],
        "properties":{
            "name": {
                "type":"string",
                "description":"A name for the battle encounter"
            },
            "summary":{
                "type":"string",
                "description":"A 1-4 sentence description of the battle scenario"
            },
            "terrain":{
                "type":"string",
                "description":"A description of the terrain the battle is happening in",
                "enum": ["Cave", "Desert", "Forest"]
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
    });
    pub static ref SHOP_TOOL_SCHEMA: Value = json!({
        "type": "object",
        "required": ["name", "summary", "terrain", "enemies", "enemy_type", "health", "attack_name", "attack_damage"],
        "properties":{
            "name": {
                "type":"string",
                "description":"A name for the battle encounter"
            },
            "summary":{
                "type":"string",
                "description":"A 1-4 sentence description of the battle scenario"
            },
            "terrain":{
                "type":"string",
                "description":"A description of the terrain the battle is happening in",
                "enum": ["Cave", "Desert", "Forest"]
            },
            "enemies": {
                "type": "array",
                "description": "A list of enemies to fight",
                "items": {
                    "type": "object",
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
    });
}
