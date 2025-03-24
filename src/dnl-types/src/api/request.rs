use super::find_options_value;

use serde::{Deserialize, Serialize};
use serenity::model::application::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGameData {
    pub user_id: String,
    pub name: String,
    pub class: String,
    pub race: String,
    pub background: String,
}

impl From<CommandInteraction> for NewGameData {
    fn from(source: CommandInteraction) -> Self {
        Self {
            user_id: source.user.id.to_string(),
            name: find_options_value(&source.data.options, "name"),
            class: find_options_value(&source.data.options, "class"),
            race: find_options_value(&source.data.options, "race"),
            background: find_options_value(&source.data.options, "background"),
        }
    }
}
