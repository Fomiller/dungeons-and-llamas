#[derive(strum::Display)]
pub enum SkillsSortKey {
    #[strum(to_string = "Acrobatics")]
    Acrobatics,
    #[strum(to_string = "AnimalHandling")]
    AnimalHandling,
    #[strum(to_string = "Arcana")]
    Arcana,
    #[strum(to_string = "Atheletics")]
    Atheletics,
    #[strum(to_string = "Deception")]
    Deception,
    #[strum(to_string = "History")]
    History,
    #[strum(to_string = "Insight")]
    Insight,
    #[strum(to_string = "Intimidatiaon")]
    Intimidatiaon,
    #[strum(to_string = "Investigation")]
    Investigation,
    #[strum(to_string = "Medicine")]
    Medicine,
    #[strum(to_string = "Nature")]
    Nature,
    #[strum(to_string = "Perception")]
    Perception,
    #[strum(to_string = "Persuasion")]
    Persuasion,
    #[strum(to_string = "Religion")]
    Religion,
    #[strum(to_string = "SleightOfHand")]
    SleightOfHand,
    #[strum(to_string = "Stealth")]
    Stealth,
    #[strum(to_string = "Survival")]
    Survival,
}

// #[cfg(test)]
// mod tests {
//     use super::super::super::super::super::RootSortKey;
//     use super::super::super::super::GameSortKey;
//     use super::super::super::PlayerSortKey;
//     use super::super::StatsSortKey;
//     use super::SkillsSortKey;
//
//     #[test]
//     fn test_skills_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let game_id = "12345";
//         let variants = vec![
//             (
//                 "12345#Game#Player#Stats#Skills#Acrobatics",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Acrobatics,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#AnimalHandling",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::AnimalHandling,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Arcana",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Arcana,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Atheletics",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Atheletics,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Deception",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Deception,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#History",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::History,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Insight",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Insight,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Intimidatiaon",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Intimidatiaon,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Investigation",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Investigation,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Medicine",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Medicine,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Nature",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Nature,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Perception",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Perception,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Persuasion",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Persuasion,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Religion",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Religion,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#SleightOfHand",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::SleightOfHand,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Stealth",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Stealth,
//                     ))),
//                 ),
//             ),
//             (
//                 "12345#Game#Player#Stats#Skills#Survival",
//                 RootSortKey::Game(
//                     game_id,
//                     GameSortKey::Player(PlayerSortKey::Stats(StatsSortKey::Skills(
//                         SkillsSortKey::Survival,
//                     ))),
//                 ),
//             ),
//         ];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
