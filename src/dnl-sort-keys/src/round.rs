#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum TempState {
    #[strum(to_string = "CharacterCreation")]
    CharacterCreation,
}
