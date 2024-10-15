pub mod game;
pub mod message;
pub mod user;

use game::GameSortKey;
use message::MessageSortKey;
use serde::{Deserialize, Serialize};
use user::UserSortKey;

use self::game::player::PlayerSortKeyBuilder;

#[derive(strum::Display)]
pub enum RootSortKey {
    #[strum(to_string = "Game#")]
    Game,
    #[strum(to_string = "User#")]
    User,
    #[strum(to_string = "Messages#")]
    Message,
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

#[derive(Default)]
pub struct SortKeyBuilder {
    id: String,
    game: Option<GameSortKey>,
    user: Option<UserSortKey>,
    message: Option<MessageSortKey>,
}

impl SortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn game(mut self, game: GameSortKey) -> Self {
        self.game = Some(game);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from(format!("{}#", self.id));

        if let Some(game) = self.game {
            result.push_str(&format!(
                "#{}",
                match game {
                    GameSortKey::Player => {
                        PlayerSortKeyBuilder
                    }
                    GameSortKey::NPC => GameSortKey::NPC.to_string(),
                    GameSortKey::Level => GameSortKey::Level.to_string(),
                    GameSortKey::Round => GameSortKey::Round.to_string(),
                    GameSortKey::Enemy => GameSortKey::Enemy.to_string(),
                }
            ));
        }
        if let Some(user) = self.user {
            result.push_str(&format!("#{}", user.to_string()));
        }
        if let Some(item_type) = self.game {
            result.push_str(&format!(
                "#{}",
                match item_type {
                    GameSortKey::Player => GameSortKey::Player.to_string(),
                    GameSortKey::NPC => GameSortKey::NPC.to_string(),
                    GameSortKey::Level => GameSortKey::Level.to_string(),
                    GameSortKey::Round => GameSortKey::Round.to_string(),
                    GameSortKey::Enemy => GameSortKey::Enemy.to_string(),
                }
            ));
        }

        result
    }
}
