#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum UserSortKey {
    #[strum(to_string = "ActiveGameId")]
    ActiveGameId,
    #[strum(to_string = "Metadata")]
    Metadata,
}
