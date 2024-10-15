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

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::super::RootSortKey;
//     use super::super::super::super::super::{GameSortKey, PlayerSortKey};
//     use super::super::super::{InventorySortKey, ItemSortKey};
//     use super::super::ArmorEquippedStateSortKey;
//     use super::ArmorSortKey;
//
//     #[test]
//     fn test_armor_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#UnEquipped#Light",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::UnEquipped(
//                             ArmorSortKey::Light,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#UnEquipped#Medium",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::UnEquipped(
//                             ArmorSortKey::Medium,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#UnEquipped#Heavy",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::UnEquipped(
//                             ArmorSortKey::Heavy,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#UnEquipped#Shield",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::UnEquipped(
//                             ArmorSortKey::Shield,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#Equipped#Light",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::Equipped(
//                             ArmorSortKey::Light,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#Equipped#Medium",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::Equipped(
//                             ArmorSortKey::Medium,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#Equipped#Heavy",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::Equipped(
//                             ArmorSortKey::Heavy,
//                         )),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Armor#Equipped#Shield",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Armor(ArmorEquippedStateSortKey::Equipped(
//                             ArmorSortKey::Shield,
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
