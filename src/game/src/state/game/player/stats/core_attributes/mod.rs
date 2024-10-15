#[derive(strum::Display)]
pub enum CoreAttributesSortKey {
    #[strum(to_string = "ArmorClass")]
    ArmorClass,
    #[strum(to_string = "DifficultyClass")]
    DifficultyClass,
    #[strum(to_string = "HitPoints")]
    HitPoints,
    #[strum(to_string = "Speed")]
    Speed,
    #[strum(to_string = "ProficiencyBonus")]
    ProficiencyBonus,
    #[strum(to_string = "Initiative")]
    Initiative,
    #[strum(to_string = "Defenses")]
    Defenses,
    #[strum(to_string = "Level")]
    Level,
}
//
// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::RootSortKey;
//     use super::super::super::super::GameSortKey;
//     use super::super::super::PlayerSortKey;
//     use super::super::StatsSortKey;
//     use super::CoreAttributesSortKey;
//
//     #[test]
//     fn test_core_attributes_sort_key() {
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#Level",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::Level,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#Defenses",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::Defenses,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#ArmorClass",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::ArmorClass,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#DifficultyClass",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::DifficultyClass,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#HitPoints",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::HitPoints,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#Speed",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::Speed,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#ProficiencyBonus",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::ProficiencyBonus,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#CoreAttributes#Initiative",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::CoreAttributes(
//                         CoreAttributesSortKey::Initiative,
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
