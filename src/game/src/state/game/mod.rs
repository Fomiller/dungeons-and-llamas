use self::player::PlayerSortKeyBuilder;

pub mod enemy;
pub mod level;
pub mod npc;
pub mod player;
pub mod round;

#[derive(strum::Display)]
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

// #[cfg(test)]
// mod tests {
//     use super::super::GameSortKey;
//     use super::super::RootSortKey;
//     use super::player::{stats::abilities::AbilitiesSortKey, stats::StatsSortKey};
//     use super::PlayerSortKey;
//
//     #[test]
//     fn test_game_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Enemy#Stats#Abilities#Strength",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Enemy(PlayerSortKey::Stats(StatsSortKey::Abilities(
//                         AbilitiesSortKey::Strength,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#NPC#Stats#Abilities#Strength",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::NPC(PlayerSortKey::Stats(StatsSortKey::Abilities(
//                         AbilitiesSortKey::Strength,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Level",
//                 RootSortKey::Game(game_id, GameSortKey::Level),
//             ),
//             (
//                 "12345#Game#Round",
//                 RootSortKey::Game(game_id, GameSortKey::Round),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
