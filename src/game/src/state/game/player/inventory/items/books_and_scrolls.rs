#[derive(strum::Display)]
pub enum BookAndScrollSortKey {
    #[strum(to_string = "Reading")]
    Reading,
    #[strum(to_string = "Spellbook")]
    Spellbook,
    #[strum(to_string = "Scroll")]
    Scroll,
}
