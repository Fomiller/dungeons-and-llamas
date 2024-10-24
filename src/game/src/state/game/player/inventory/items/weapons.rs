use serde::{Deserialize, Serialize};
use std::any::Any;

use super::equipped::EquippedStateSortKey;
use crate::state::SortKeyBuildable;

#[derive(strum::Display, Debug, Clone, Copy, strum::EnumIter)]
pub enum WeaponSortKey {
    #[strum(to_string = "Melee")]
    Melee,
    #[strum(to_string = "Ranged")]
    Ranged,
    #[strum(to_string = "Thrown")]
    Thrown,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct WeaponSortKeyBuilder {
    weapon: Option<WeaponSortKey>,
    equipped: EquippedStateSortKey,
}

impl SortKeyBuildable for WeaponSortKeyBuilder {
    fn build(&self) -> String {
        let mut result = String::from(format!("Weapons#{}", self.equipped.to_string()));
        if let Some(weapon) = self.weapon {
            result.push_str(&format!("{}", weapon.to_string()));
        }
        result
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl From<Box<dyn SortKeyBuildable>> for WeaponSortKeyBuilder {
    fn from(skb: Box<dyn SortKeyBuildable>) -> Self {
        if let Some(weapon) = skb.as_any().downcast_ref::<WeaponSortKeyBuilder>() {
            return *weapon;
        }
        Self::default()
    }
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentWeapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}
