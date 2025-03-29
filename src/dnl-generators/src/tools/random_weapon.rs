// use crate::*;
//
// use std::collections::HashMap;
//
// use dnl_types::scenarios::ScenarioInput;
// use dnl_types::tools::Tool;
//
// pub static RANDOM_WEAPON_SYSTEM_PROMPT: &str = "
// You are a DM for a single player dungeons and dragons style text adventure game.
// You need to create a selection of weapons for your player to choose from.
// It is important that you always create unique and fun weapons to keep the player engaged.
//
// Rules for creating random weapons:
// - Create weapons appropriate to the players level
// - Make sure that the weapon has appropriate stats
//
// As a DM you dont want to always overpower your player so it is important that you offer a variety of common, rare, and legendary weapons.
// Rare and legendary weapons should appear more often as the players level goes up.
// ";
//
// pub static RANDOM_WEAPON_JSON_PROMPT: &str = "
// Using the context provided use the 'random_weapon' tool to create {{weapon_count}} random weapons for a level {{level}} player.
// The response should be json with the following structure:
// {{example}}
// ";
//
// pub static RANDOM_WEAPON_TEXT_PROMPT: &str = "";
//
// pub type RandomWeaponJsonGenerator = JsonResponseGenerator<RandomWeaponJsonGeneratorConfig>;
//
// #[derive(Clone)]
// pub struct RandomWeaponJsonGeneratorConfig {
//     pub config_input: JsonGeneratorConfigInput,
//     pub tool: Tool,
//     pub system_prompt: Prompt,
//     pub json_prompt: Prompt,
//     pub text_prompt: Prompt,
// }
//
// impl JsonResponseGeneratorConfig for RandomWeaponJsonGeneratorConfig {
//     fn scenario_input(&self) -> JsonGeneratorConfigInput {
//         self.config_input.clone()
//     }
//     fn model(&self) -> String {
//         self.config_input.model().clone()
//     }
//     fn system_prompt(&self) -> String {
//         self.system_prompt.format()
//     }
//     fn schema(&self) -> serde_json::Value {
//         self.tool.schema()
//     }
//     fn json_prompt(&self) -> String {
//         self.json_prompt.format()
//     }
//     fn text_prompt(&self) -> String {
//         self.text_prompt.format()
//     }
//     fn tool(&self) -> Tool {
//         self.tool.clone()
//     }
// }
//
// impl RandomWeaponJsonGeneratorConfig {
//     pub fn new(
//         scenario_input: ScenarioInput,
//         system_vars: HashMap<String, String>,
//         text_vars: HashMap<String, String>,
//         json_vars: HashMap<String, String>,
//     ) -> Self {
//         let system_prompt = Prompt {
//             text: RANDOM_WEAPON_SYSTEM_PROMPT.to_string(),
//             variables: system_vars,
//         };
//
//         let json_prompt = Prompt {
//             text: RANDOM_WEAPON_JSON_PROMPT.to_string(),
//             variables: json_vars,
//         };
//
//         let text_prompt = Prompt {
//             text: RANDOM_WEAPON_TEXT_PROMPT.to_string(),
//             variables: text_vars,
//         };
//
//         let tool = Tool::RandomWeapon;
//
//         Self {
//             scenario_input,
//             system_prompt,
//             tool,
//             json_prompt,
//             text_prompt,
//         }
//     }
// }
