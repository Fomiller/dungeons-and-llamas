#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum StateSortKey {
    #[strum(to_string = "GameState")]
    GameState,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub round: Option<u8>,
    pub level: Option<u8>,
    pub prev_encounter: Option<String>,
    pub curr_encounter: Option<String>,
}
