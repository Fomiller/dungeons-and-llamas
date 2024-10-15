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

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::super::RootSortKey;
//     use super::super::super::super::super::{GameSortKey, PlayerSortKey};
//     use super::super::super::{InventorySortKey, ItemSortKey};
//     use super::super::WeaponEquippedStateSortKey;
//     use super::WeaponSortKey;
//
//     #[test]
//     fn test_weapon_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#UnEquipped#Melee",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::UnEquipped(
//                             WeaponSortKey::Melee,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#UnEquipped#Ranged",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::UnEquipped(
//                             WeaponSortKey::Ranged,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#UnEquipped#Thrown",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::UnEquipped(
//                             WeaponSortKey::Thrown,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#Equipped#Melee",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::Equipped(
//                             WeaponSortKey::Melee,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#Equipped#Ranged",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::Equipped(
//                             WeaponSortKey::Ranged,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Weapons#Equipped#Thrown",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Weapons(WeaponEquippedStateSortKey::Equipped(
//                             WeaponSortKey::Thrown,
//                         )),
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
