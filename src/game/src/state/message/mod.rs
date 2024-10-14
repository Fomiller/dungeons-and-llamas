use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum MessageSortKey {
    #[strum(to_string = "LastMessageToken")]
    LastMessageToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<'a> {
    #[serde(rename = "LastMessageToken")]
    pub last_message_token: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentMessage<'a> {
    #[serde(rename = "LastMessageToken", skip_serializing_if = "Option::is_none")]
    pub last_message_token: Option<&'a str>,
}
