#[derive(strum::Display)]
pub enum BookAndScrollSortKey {
    #[strum(to_string = "Reading")]
    Reading,
    #[strum(to_string = "Spellbook")]
    Spellbook,
    #[strum(to_string = "Scrolls")]
    Scroll,
}

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::super::RootSortKey;
//     use super::super::super::super::super::{GameSortKey, PlayerSortKey};
//     use super::super::super::{InventorySortKey, ItemSortKey};
//     use super::BookAndScrollSortKey;
//
//     #[test]
//     fn test_book_and_scrolls_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Inventory#Item#BooksAndScrolls#Reading",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::BooksAndScrolls(BookAndScrollSortKey::Reading),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#BooksAndScrolls#Spellbook",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::BooksAndScrolls(BookAndScrollSortKey::Spellbook),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Inventory#Item#BooksAndScrolls#Scrolls",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Inventory(InventorySortKey::Item(
//                         ItemSortKey::BooksAndScrolls(BookAndScrollSortKey::Scroll),
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
