pub mod entity;
pub mod level;
pub mod map;
pub mod round;

use entity::EntitySortKeyBuilder;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

impl GameState {
    pub fn new(user_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
        }
    }
}

#[derive(strum::Display, strum::EnumIter)]
pub enum GameSortKey {
    #[strum(to_string = "Entity#")]
    Entity,
    #[strum(to_string = "Level")]
    Level,
    #[strum(to_string = "Round")]
    Round,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct GameSortKeyBuilder {
    entity: Option<EntitySortKeyBuilder>,
    level: Option<bool>,
    round: Option<bool>,
}

impl GameSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn entity(mut self, entity: EntitySortKeyBuilder) -> Self {
        self.entity = Some(entity);
        self
    }

    pub fn level(mut self, level: bool) -> Self {
        self.level = Some(level);
        self
    }
    pub fn round(mut self, round: bool) -> Self {
        self.round = Some(round);
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from("Game#");
        if let Some(entity) = self.entity {
            result.push_str(&format!("{}", entity.build().to_string()));
        } else if let Some(level) = self.level {
            result.push_str(&format!("{}", level.to_string()));
        } else if let Some(round) = self.round {
            result.push_str(&format!("{}", round.to_string()));
        }

        result
    }
}
