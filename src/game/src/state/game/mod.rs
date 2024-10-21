pub mod enemy;
pub mod level;
pub mod map;
pub mod npc;
pub mod player;
pub mod round;

use player::PlayerSortKeyBuilder;

#[derive(strum::Display, strum::EnumIter)]
pub enum GameSortKey {
    #[strum(to_string = "Player#")]
    Player,
    #[strum(to_string = "Enemy#")]
    Enemy,
    #[strum(to_string = "NPC#")]
    NPC,
    #[strum(to_string = "Level")]
    Level,
    #[strum(to_string = "Round")]
    Round,
}

#[derive(Default)]
pub struct GameSortKeyBuilder {
    player: Option<PlayerSortKeyBuilder>,
    enemy: Option<bool>,
    npc: Option<bool>,
    level: Option<bool>,
    round: Option<bool>,
}

impl GameSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn player(mut self, player: PlayerSortKeyBuilder) -> Self {
        self.player = Some(player);
        self
    }
    pub fn enemy(mut self, enemy: bool) -> Self {
        self.enemy = Some(enemy);
        self
    }
    pub fn npc(mut self, npc: bool) -> Self {
        self.npc = Some(npc);
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
        if let Some(player) = self.player {
            result.push_str(&format!("{}", player.build().to_string()));
        } else if let Some(enemy) = self.enemy {
            result.push_str(&format!("{}", enemy.to_string()));
        } else if let Some(npc) = self.npc {
            result.push_str(&format!("{}", npc.to_string()));
        } else if let Some(level) = self.level {
            result.push_str(&format!("{}", level.to_string()));
        } else if let Some(round) = self.round {
            result.push_str(&format!("{}", round.to_string()));
        }

        result
    }
}
