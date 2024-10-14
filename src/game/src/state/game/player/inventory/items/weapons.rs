use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum WeaponEquippedStateSortKey {
    #[strum(to_string = "#Equipped{0}")]
    Equipped(WeaponSortKey),
    #[strum(to_string = "#UnEquipped{0}")]
    UnEquipped(WeaponSortKey),
}

#[derive(strum::Display)]
pub enum WeaponSortKey {
    #[strum(to_string = "Melee")]
    Melee,
    #[strum(to_string = "Ranged")]
    Ranged,
    #[strum(to_string = "Thrown")]
    Thrown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentWeapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
