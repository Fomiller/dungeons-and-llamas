use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "ActiveGameId")]
    pub active_game_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Weapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
