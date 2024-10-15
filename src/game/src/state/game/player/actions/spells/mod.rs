#[derive(strum::Display)]
pub enum SpellSortKey {
    #[strum(to_string = "Cantrip")]
    Cantrip,
    #[strum(to_string = "Spell#Concentration")]
    Concentration,
    #[strum(to_string = "Spell#Instant")]
    Instant,
}

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::RootSortKey;
//     use super::super::super::super::GameSortKey;
//     use super::super::super::PlayerSortKey;
//     use super::super::ActionsSortKey;
//     use super::{SpellSortKey, SpellTypeSortKey};
//
//     #[test]
//     fn test_spells_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Actions#Spells#Cantrip",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::Spells(
//                         SpellSortKey::Cantrip,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Actions#Spells#Spell#Concentration",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::Spells(
//                         SpellSortKey::Spell(SpellTypeSortKey::Concentration),
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Actions#Spells#Spell#Instant",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Actions(ActionsSortKey::Spells(
//                         SpellSortKey::Spell(SpellTypeSortKey::Instant),
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
