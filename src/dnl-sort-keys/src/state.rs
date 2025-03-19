#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum StateSortKey {
    #[strum(to_string = "GameState")]
    GameState,
}
