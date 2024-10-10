use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "ActiveGameId", skip_serializing_if = "Option::is_none")]
    pub active_game_id: Option<String>,
    #[serde(rename = "Games", skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T: Serialize> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<Vec<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentWeapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
