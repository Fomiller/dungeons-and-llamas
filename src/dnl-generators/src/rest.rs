use crate::*;

use std::collections::HashMap;

use dnl_types::tools::battle::BattleToolOutput;
use dnl_types::tools::Tools;
use dnl_types::scenario::ScenarioInput;

use lazy_static::lazy_static;

pub static REST_SYSTEM_PROMPT: &str = "
You are a DM for a single player dungeons and dragons style text adventure game.
It is important that you always create unique and fun scenarios with a wide variety of
situations, enemies, items, and settings to keep the player engaged.

You are able to create 3 different scenario types. 
- Battle 
- Shop 
- Rest

Rules for creating rest scenarios:
- Keep the scenarios inline with the theme provided
- Make sure that the scenario is appropriate for the players level


You always keep your scenarios to single 2-4 sentence paragraphs, without bullet points.

";

pub static REST_JSON_PROMPT: &str = "
Using the context provided use the 'rest' tool to create a level {{level}} rest scenario.
The responses should be json with the following structure:
{{example}}
";

pub static REST_TEXT_PROMPT: &str = "
Create a random {{theme}} rest scenario for the player. Make sure that the scenario is truely unique
to the previous examples provided in the context if there are any. The player constantly wants to feel like they are 
being presented with brand new scenarios every time.
";

pub type BattleGenerator = JsonResponseGenerator<BattleJsonGeneratorConfig, BattleToolOutput>;

pub struct BattleJsonGeneratorConfig {
    pub scenario_input: ScenarioInput,
    pub tool: Tools,
    pub system_prompt: Prompt,
    pub json_prompt: Prompt,
    pub text_prompt: Prompt,
}

impl JsonResponseGeneratorConfig for BattleJsonGeneratorConfig {
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

impl BattleJsonGeneratorConfig {
    pub fn new(
        scenario_input: ScenarioInput,
        system_vars: HashMap<String, String>,
        text_vars: HashMap<String, String>,
        json_vars: HashMap<String, String>,
    ) -> Self {
        let system_prompt = Prompt {
            text: REST_SYSTEM_PROMPT.to_string(),
            variables: system_vars,
        };

        let json_prompt = Prompt {
            text: REST_JSON_PROMPT.to_string(),
            variables: json_vars,
        };

        let text_prompt = Prompt {
            text: REST_TEXT_PROMPT.to_string(),
            variables: text_vars,
        };

        let tool = Tools::Battle;

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
    pub static ref REST_TOOL_SCHEMA: serde_json::Value = {
        serde_json::json!({
            "type": "object",
            "required": ["name", "summary", "terrain", "enemies", "enemy_type", "health", "attack", "attack_name", "attack_damage"],
            "properties":{
                "name": {
                    "type":"string",
                    "description":"A name for the rest encounter"
                },
                "summary":{
                    "type":"string",
                    "description":"A 30 to 50 word objective summary of the rest scenario. Make sure to include the number and types of enemies."
                },
                "terrain":{
                    "type":"string",
                    "description":"A description of the terrain the rest is happening in",
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
