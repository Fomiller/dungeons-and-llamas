#[derive(strum::Display)]
pub enum ClothingSortKey {
    #[strum(to_string = "Travelers")]
    Travelers,
    #[strum(to_string = "Cloak")]
    Cloak,
    #[strum(to_string = "Jewelry")]
    Jewelry,
}
// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::super::RootSortKey;
//     use super::super::super::super::super::{GameSortKey, PlayerSortKey};
//     use super::super::super::{InventorySortKey, ItemSortKey};
//     use super::ClothingSortKey;
//
//     #[test]
//     fn test_magic_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Inventory#Item#Clothing#Travelers",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Clothing(ClothingSortKey::Travelers),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Clothing#Cloak",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Clothing(ClothingSortKey::Cloak),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#Clothing#Jewelry",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::Clothing(ClothingSortKey::Jewelry),
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
