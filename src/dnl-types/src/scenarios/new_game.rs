use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewGameScenario {
    pub summary: String,
    pub name: String,
}

impl DiscordMsg for NewGameScenario {
    fn to_message(&self) -> String {
        format!("# *{}*\n## Description:\n{}", self.name, self.summary,)
    }
}
