pub mod game;
pub mod message;
pub mod user;

use game::GameSortKey;
use message::MessageSortKey;
use serde::{Deserialize, Serialize};
use user::UserSortKey;

#[derive(strum::Display)]
pub enum RootSortKey<'a> {
    #[strum(to_string = "{0}#Game#{1}")]
    Game(&'a str, GameSortKey),
    #[strum(to_string = "{0}#User#{1}")]
    User(&'a str, UserSortKey),
    #[strum(to_string = "{0}#Messages#{1}")]
    Message(&'a str, MessageSortKey),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
}
