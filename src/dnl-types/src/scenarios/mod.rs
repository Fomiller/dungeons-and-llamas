pub mod battle;

use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Debug, Deserialize, Serialize, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Scenario {
    Battle,
    Shop,
    Rest,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScenarioInput {
    pub user_id: String,
    pub game_id: String,
    pub model: String,
    pub scenario: String,
    pub theme: String,
    pub level: String,
    pub round: String,
}
