#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum SpellSortKey {
    #[strum(to_string = "Cantrip")]
    Cantrip,
    #[strum(to_string = "Spell#Concentration")]
    Concentration,
    #[strum(to_string = "Spell#Instant")]
    Instant,
}
