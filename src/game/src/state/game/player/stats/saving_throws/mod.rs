#[derive(strum::Display)]
pub enum SavingThrowsSortKey {
    #[strum(to_string = "Strength")]
    Strength,
    #[strum(to_string = "Charisma")]
    Charisma,
    #[strum(to_string = "Constitution")]
    Constitution,
    #[strum(to_string = "Dexterity")]
    Dexterity,
    #[strum(to_string = "Intelligence")]
    Intelligence,
    #[strum(to_string = "Wisdom")]
    Wisdom,
}
//
// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::RootSortKey;
//     use super::super::super::super::GameSortKey;
//     use super::super::super::PlayerSortKey;
//     use super::super::StatsSortKey;
//     use super::SavingThrowsSortKey;
//
//     #[test]
//     fn test_saving_throws_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Strength",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Strength,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Dexterity",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Dexterity,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Constitution",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Constitution,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Charisma",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Charisma,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Intelligence",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Intelligence,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#SavingThrows#Wisdom",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::SavingThrows(
//                         SavingThrowsSortKey::Wisdom,
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
