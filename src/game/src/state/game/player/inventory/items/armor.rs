#[derive(strum::Display)]
pub enum ArmorSortKey {
    #[strum(to_string = "Light")]
    Light,
    #[strum(to_string = "Medium")]
    Medium,
    #[strum(to_string = "Heavy")]
    Heavy,
    #[strum(to_string = "Shield")]
    Shield,
}

#[derive(strum::Display)]
pub enum ArmorEquippedStateSortKey {
    #[strum(to_string = "#Equipped{0}")]
    Equipped(ArmorSortKey),
    #[strum(to_string = "#UnEquipped{0}")]
    UnEquipped(ArmorSortKey),
}
