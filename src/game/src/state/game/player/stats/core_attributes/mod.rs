#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum CoreAttributesSortKey {
    #[strum(to_string = "ArmorClass")]
    ArmorClass,
    #[strum(to_string = "DifficultyClass")]
    DifficultyClass,
    #[strum(to_string = "HitPoints")]
    HitPoints,
    #[strum(to_string = "Speed")]
    Speed,
    #[strum(to_string = "ProficiencyBonus")]
    ProficiencyBonus,
    #[strum(to_string = "Initiative")]
    Initiative,
    #[strum(to_string = "Defenses")]
    Defenses,
    #[strum(to_string = "Level")]
    Level,
}
