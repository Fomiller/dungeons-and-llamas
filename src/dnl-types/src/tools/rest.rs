use super::MockData;
use crate::traits::DiscordMsg;
use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestToolOutput {
    pub summary: String,
    pub flora: String,
    pub fauna: String,
    pub secret: Option<String>,
}

impl MockData for RestToolOutput {
    fn mock() -> Self {
        let summary = "This is a nice place to rest".to_string();
        let flora = "plants".to_string();
        let fauna = "animals".to_string();
        let secret = None;
        Self {
            summary,
            flora,
            fauna,
            secret,
        }
    }
}

impl DiscordMsg for RestToolOutput {
    fn to_message(&self) -> String {
        let mut message = format!("*{}*\n\n*{}*\n\n*{}*", self.summary, self.flora, self.fauna);

        if let Some(secret) = &self.secret {
            message.push_str(&format!("\n\n||{}||", secret));
            return message;
        };

        message
    }
}

lazy_static! {
    pub static ref REST_TOOL_SCHEMA: serde_json::Value = {
        serde_json::json!({
            "type": "object",
            "required": ["summary", "flora", "fauna"],
            "properties":{
                "summary":{
                    "type":"string",
                    "description":"A 30 to 50 word objective summary of the rest scenario."
                },
                "flora":{
                    "type":"string",
                    "description":"A description of the flora found int the area.",
                },
                "fauna":{
                    "type":"string",
                    "description":"A description of the fauna found int the area.",
                },
                "secret":{
                    "type":"string",
                    "description":"A secret that a player might find if they search the area.",
                }
            }
        })
    };
}
