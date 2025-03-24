use super::find_options_value;

use serde::{Deserialize, Serialize};
use serenity::model::application::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct NewGameRequest {
    pub id: String,
    pub name: String,
    pub race: String,
    pub background: String,
}

impl From<CommandInteraction> for NewGameRequest {
    fn from(source: CommandInteraction) -> Self {
        Self {
            id: source.user.id.to_string(),
            name: find_options_value(&source.data.options, "name"),
            race: find_options_value(&source.data.options, "race"),
            background: find_options_value(&source.data.options, "background"),
        }
    }
}
