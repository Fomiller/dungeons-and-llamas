#[derive(Debug, Clone, Copy, strum::Display, strum::EnumIter)]
pub enum ConditionsSortKey {
    #[strum(to_string = "Buff")]
    Buff,
    #[strum(to_string = "Debuff")]
    Debuff,
}
