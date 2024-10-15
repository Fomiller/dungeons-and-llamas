#[derive(strum::Display, Default, strum::EnumIter)]
pub enum EquippedStateSortKey {
    #[strum(to_string = "Equipped#")]
    Equipped,
    #[default]
    #[strum(to_string = "UnEquipped#")]
    UnEquipped,
}
