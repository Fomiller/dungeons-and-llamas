use serde::{Deserialize, Serialize};

use super::equipped::EquippedStateSortKey;

#[derive(strum::Display, Debug, strum::EnumIter)]
pub enum WeaponSortKey {
    #[strum(to_string = "Melee")]
    Melee,
    #[strum(to_string = "Ranged")]
    Ranged,
    #[strum(to_string = "Thrown")]
    Thrown,
}

#[derive(Default)]
pub struct WeaponSortKeyBuilder {
    weapon: Option<WeaponSortKey>,
    equipped: EquippedStateSortKey,
}

impl WeaponSortKeyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn weapon(mut self, weapon: WeaponSortKey) -> Self {
        self.weapon = Some(weapon);
        self
    }
    pub fn equipped(mut self, equipped: EquippedStateSortKey) -> Self {
        self.equipped = equipped;
        self
    }

    pub fn build(self) -> String {
        let mut result = String::from(format!("Weapons#{}", self.equipped.to_string()));
        if let Some(weapon) = self.weapon {
            result.push_str(&format!("{}", weapon.to_string()));
        }
        result
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentWeapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
