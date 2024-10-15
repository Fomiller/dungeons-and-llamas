#[derive(strum::Display, Default)]
pub enum EquippedStateSortKey {
    #[strum(to_string = "Equipped#")]
    Equipped,
    #[default]
    #[strum(to_string = "UnEquipped#")]
    UnEquipped,
}
