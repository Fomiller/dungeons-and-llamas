use crate::traits::DiscordMsg;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestScenario {
    pub summary: String,
    pub flora: String,
    pub fauna: String,
    pub secret: Option<String>,
}

impl DiscordMsg for RestScenario {
    fn to_message(&self) -> String {
        let mut message = format!("*{}*\n\n*{}*\n\n*{}*", self.summary, self.flora, self.fauna);

        if let Some(secret) = &self.secret {
            message.push_str(&format!("\n\n||{}||", secret));
            return message;
        };

        message
    }
}
