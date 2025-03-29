#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum StateSortKey {
    #[strum(to_string = "GameState")]
    GameState,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct State {
    pub round: Option<String>,
    pub level: Option<String>,
    pub prev_encounter: Option<String>,
    pub curr_encounter: Option<String>,
}
