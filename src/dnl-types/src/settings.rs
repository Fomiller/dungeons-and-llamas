use crate::api::find_options_value;
use serde::{Deserialize, Serialize};
use serenity::model::application::CommandInteraction;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

impl From<CommandInteraction> for Settings {
    fn from(source: CommandInteraction) -> Self {
        Self {
            theme: find_options_value(&source.data.options, "theme"),
            model: find_options_value(&source.data.options, "model"),
        }
    }
}
