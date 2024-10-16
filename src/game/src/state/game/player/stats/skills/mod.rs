#[derive(strum::Display, strum::EnumIter)]
pub enum SkillsSortKey {
    #[strum(to_string = "Acrobatics")]
    Acrobatics,
    #[strum(to_string = "AnimalHandling")]
    AnimalHandling,
    #[strum(to_string = "Arcana")]
    Arcana,
    #[strum(to_string = "Atheletics")]
    Atheletics,
    #[strum(to_string = "Deception")]
    Deception,
    #[strum(to_string = "History")]
    History,
    #[strum(to_string = "Insight")]
    Insight,
    #[strum(to_string = "Intimidatiaon")]
    Intimidatiaon,
    #[strum(to_string = "Investigation")]
    Investigation,
    #[strum(to_string = "Medicine")]
    Medicine,
    #[strum(to_string = "Nature")]
    Nature,
    #[strum(to_string = "Perception")]
    Perception,
    #[strum(to_string = "Persuasion")]
    Persuasion,
    #[strum(to_string = "Religion")]
    Religion,
    #[strum(to_string = "SleightOfHand")]
    SleightOfHand,
    #[strum(to_string = "Stealth")]
    Stealth,
    #[strum(to_string = "Survival")]
    Survival,
}
