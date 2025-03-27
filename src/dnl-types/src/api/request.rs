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
    pub theme: String,
}

impl From<CommandInteraction> for NewGameData {
    fn from(source: CommandInteraction) -> Self {
        Self {
            user_id: source.user.id.to_string(),
            name: find_options_value(&source.data.options, "name").unwrap(),
            class: find_options_value(&source.data.options, "class").unwrap(),
            race: find_options_value(&source.data.options, "race").unwrap(),
            background: find_options_value(&source.data.options, "background").unwrap(),
            theme: find_options_value(&source.data.options, "theme").unwrap(),
        }
    }
}
