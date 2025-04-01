use lazy_static::lazy_static;

use super::shop::ShopToolItem;

pub struct NewGameToolOutput {
    pub name: String,
    pub items: Vec<ShopToolItem>,
    pub summary: String,
}

lazy_static! {
    pub static ref NEW_GAME_TOOL_SCHEMA: serde_json::Value = {
        let required = vec![
            "name",
            "summary",
            "items",
            "item_name",
            "item_description",
            "item_stats",
            "item_price",
        ];

        serde_json::json!({
            "type": "object",
            "required": required,
            "properties":{
                "name": {
                    "type":"string",
                    "description":"A name for the new game setting"
                },
                "summary":{
                    "type":"string",
                    "description":"A DND Dungeon Masters opening narrative of a campaign. 3 paragraphs max."
                },
                "items": {
                    "type": "array",
                    "description": "A list of  3 items to that the adventure can choose from",
                    "items": {
                        "type": "object",
                        "description": "An Object that defines an item, items could be anything useful to a DnD player",
                        "properties": {
                            "item_name":{
                                "type": "string",
                                "description": "Name of the item"
                            },
                            "item_description":{
                                "type": "string",
                                "description": "1 sentence description of the item"
                            },
                            "item_stats":{
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
                            "item_price":{
                                "type": "string",
                                "description": "Cost of item for sale"
                            }
                        }
                    }
                }
            }
        })
    };
}
