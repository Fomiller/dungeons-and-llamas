use crate::*;

use std::collections::HashMap;

use dnl_types::tools::shop::ShopToolOutput;
use dnl_types::tools::Tools;
use dnl_types::scenario::ScenarioInput;

use lazy_static::lazy_static;

pub static SHOP_SYSTEM_PROMPT: &str = "
You are a DM for a single player dungeons and dragons style text adventure game.
It is important that you always create unique and fun scenarios with a wide variety of
situations, enemies, items, and settings to keep the player engaged.

You are able to create 3 different scenario types. 
- Battle 
- Shop 
- Rest

Rules for creating scenarios:
- Keep the scenarios inline with the theme provided
- Make sure that the scenario is appropriate for the players level


You always keep your scenarios to single 2-4 sentence paragraphs, without bullet points.

";

pub static SHOP_JSON_PROMPT: &str = "
Using the context provided use the 'shop' tool to create a level {{level}} shop scenario.
The responses should be json with the following structure:
{{example}}
";

pub static SHOP_TEXT_PROMPT: &str = "
Create a random {{theme}} shop scenario for the player. Make sure that the scenario is truely unique
to the previous examples provided in the context if there are any. The player constantly wants to feel like they are 
being presented with brand new scenarios every time.
";

pub type ShopJsonGenerator = JsonResponseGenerator<ShopJsonGeneratorConfig>;

#[derive(Clone)]
pub struct ShopJsonGeneratorConfig {
    pub scenario_input: ScenarioInput,
    pub tool: Tools,
    pub system_prompt: Prompt,
    pub json_prompt: Prompt,
    pub text_prompt: Prompt,
}

impl JsonResponseGeneratorConfig for ShopJsonGeneratorConfig {
    fn scenario_input(&self) -> ScenarioInput {
        self.scenario_input.clone()
    }
    fn model(&self) -> String {
        self.scenario_input.model.clone()
    }
    fn system_prompt(&self) -> String {
        self.system_prompt.format()
    }
    fn schema(&self) -> serde_json::Value {
        self.tool.schema()
    }
    fn json_prompt(&self) -> String {
        self.json_prompt.format()
    }
    fn text_prompt(&self) -> String {
        self.text_prompt.format()
    }
    fn tool(&self) -> Tools {
        self.tool
    }
}

impl ShopJsonGeneratorConfig {
    pub fn new(
        scenario_input: ScenarioInput,
        system_vars: HashMap<String, String>,
        text_vars: HashMap<String, String>,
        json_vars: HashMap<String, String>,
    ) -> Self {
        let system_prompt = Prompt {
            text: SHOP_SYSTEM_PROMPT.to_string(),
            variables: system_vars,
        };

        let json_prompt = Prompt {
            text: SHOP_JSON_PROMPT.to_string(),
            variables: json_vars,
        };

        let text_prompt = Prompt {
            text: SHOP_TEXT_PROMPT.to_string(),
            variables: text_vars,
        };

        let tool = Tools::Shop;

        Self {
            scenario_input,
            system_prompt,
            tool,
            json_prompt,
            text_prompt,
        }
    }
}

lazy_static! {
    pub static ref SHOP_TOOL_SCHEMA: serde_json::Value = {
        serde_json::json!({
            "type": "object",
            "required": ["name", "merchant", "name", "description", "items", "name", "desciption", "stats", "price"],
            "properties":{
                "name": {
                    "type":"string",
                    "description":"A name for the shop encounter"
                },
                "merchant":{
                    "type": "object",
                    "description": "An Object that defines merchant",
                    "properties": {
                        "name": {
                            "type": "string",
                            "description": "Name of the Merchant"
                        },
                        "description": {
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
                            "name":{
                                "type": "string",
                                "description": "Name of the item"
                            },
                            "description":{
                                "type": "string",
                                "description": "1 sentence description of the item"
                            },
                            "stats":{
                                "type": "string",
                                "description": "The stats of item."
                            },
                            "price":{
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
