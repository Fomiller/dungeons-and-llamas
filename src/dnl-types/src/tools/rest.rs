use lazy_static::lazy_static;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestToolOutput {
    pub summary: String,
    pub flora: String,
    pub fauna: String,
    pub secret: Option<String>,
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
