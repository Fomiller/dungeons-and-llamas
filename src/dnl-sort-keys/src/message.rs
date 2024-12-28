#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum MessageSortKey {
    #[strum(to_string = "LastMessageToken")]
    LastMessageToken,
}
