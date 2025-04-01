pub mod prompt;

use crate::tools::Tool;
use prompt::Prompt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorObjectConfig {
    Weapon,
    Spell,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorScenarioConfig {
    Battle,
    Shop,
    Rest,
    NewGame,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneratorType {
    Scenario(GeneratorScenarioConfig),
    Object(GeneratorObjectConfig),
}

impl GeneratorType {
    pub fn is_scenario(&self) -> bool {
        match self {
            GeneratorType::Scenario(_) => true,
            _ => false,
        }
    }
    pub fn is_object(&self) -> bool {
        match self {
            GeneratorType::Object(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GeneratorToolConfig {
    pub tool: Tool,
}

#[derive(Debug, Clone)]
pub struct GeneratorPromptConfig {
    pub system: Prompt,
    pub json: Prompt,
    pub text: Option<Prompt>,
}
