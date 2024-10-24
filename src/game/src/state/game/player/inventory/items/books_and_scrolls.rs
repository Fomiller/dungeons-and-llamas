#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum BookAndScrollSortKey {
    #[strum(to_string = "Reading")]
    Reading,
    #[strum(to_string = "Spellbook")]
    Spellbook,
    #[strum(to_string = "Scrolls")]
    Scroll,
}
