#[derive(strum::Display, Debug, Default, Clone, Copy, strum::EnumIter)]
pub enum EquippedStateSortKey {
    #[strum(to_string = "Equipped#")]
    Equipped,
    #[default]
    #[strum(to_string = "UnEquipped#")]
    UnEquipped,
}
