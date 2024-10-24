#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum SavingThrowsSortKey {
    #[strum(to_string = "Strength")]
    Strength,
    #[strum(to_string = "Charisma")]
    Charisma,
    #[strum(to_string = "Constitution")]
    Constitution,
    #[strum(to_string = "Dexterity")]
    Dexterity,
    #[strum(to_string = "Intelligence")]
    Intelligence,
    #[strum(to_string = "Wisdom")]
    Wisdom,
}
