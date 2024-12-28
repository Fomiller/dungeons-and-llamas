use serde::*;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeaponStateComponent {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
