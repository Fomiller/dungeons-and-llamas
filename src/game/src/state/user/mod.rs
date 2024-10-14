use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum UserSortKey {
    #[strum(to_string = "ActiveGameId")]
    ActiveGameId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "ActiveGameId", skip_serializing_if = "Option::is_none")]
    pub active_game_id: Option<String>,
    #[serde(rename = "Games", skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<String>>,
}
