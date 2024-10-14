use player::PlayerSortKey;

pub mod enemy;
pub mod level;
pub mod npc;
pub mod player;
pub mod round;

#[derive(strum::Display)]
pub enum GameSortKey {
    #[strum(to_string = "Player#{0}")]
    Player(PlayerSortKey),
    #[strum(to_string = "Enemy#{0}")]
    Enemy(PlayerSortKey),
    #[strum(to_string = "NPC#{0}")]
    NPC(PlayerSortKey),
    #[strum(to_string = "Level#{0}")]
    Level(PlayerSortKey),
    #[strum(to_string = "Round#{0}")]
    Round(PlayerSortKey),
}
