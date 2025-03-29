use crate::dice::DiceExpression;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolOutput {
    pub items: Vec<ShopToolItem>,
    pub merchant: ShopToolMerchant,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ShopToolMerchant {
    pub merchant_name: String,
    pub merchant_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopToolItem {
    pub item_name: String,
    pub item_description: String,
    pub item_stats: DiceExpression,
    pub item_price: String,
}

lazy_static! {
    pub static ref SHOP_TOOL_SCHEMA: serde_json::Value = {
        serde_json::json!({
            "type": "object",
            "required": ["summary", "merchant", "merchant_name", "merchant_description", "items", "item_name", "item_desciption", "item_stats", "item_price"],
            "properties":{
                "summary":{
                    "type":"string",
                    "description":"A 30 to 50 word objective summary of the shop scenario."
                },
                "merchant":{
                    "type": "object",
                    "description": "An Object that defines merchant",
                    "properties": {
                        "merchant_name": {
                            "type": "string",
                            "description": "Name of the Merchant"
                        },
                        "merchant_description": {
                            "type": "string",
                            "description": "A 30-50 word description of the merchant and his surroundings",
                        }
                    }
                },
                "items": {
                    "type": "array",
                    "description": "A list of items to purchase",
                    "items": {
                        "type": "object",
                        "description": "An Object that defines an item to purchase, items could be anything useful to a DnD player",
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
