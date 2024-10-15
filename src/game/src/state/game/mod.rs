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

#[cfg(test)]
mod tests {
    use super::super::GameSortKey;
    use super::super::RootSortKey;
    use super::player::{stats::abilities::AbilitiesSortKey, stats::StatsSortKey};
    use super::PlayerSortKey;

    #[test]
    fn test_game_sort_key() {
        // maybe use EnumIter here, initial exploration did not work b/c of
        // having to use a default
        let game_id = "12345";
        let variants = vec![
            (
                "12345#Game#Enemy#Stats#Abilities#Strength",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::Enemy(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Strength,
                    ))),
                ),
            ),
            (
                "12345#Game#NPC#Stats#Abilities#Strength",
                RootSortKey::Game(
                    game_id,
                    GameSortKey::NPC(PlayerSortKey::Stats(StatsSortKey::Abilities(
                        AbilitiesSortKey::Strength,
                    ))),
                ),
            ),
            (
                "12345#Game#Level",
                RootSortKey::Game(game_id, GameSortKey::Level),
            ),
            (
                "12345#Game#Round",
                RootSortKey::Game(game_id, GameSortKey::Round),
            ),
        ];
        for variant in variants.iter() {
            assert_eq!(variant.0, variant.1.to_string())
        }
    }
}
