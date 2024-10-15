use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum UserSortKey {
    #[strum(to_string = "ActiveGameId")]
    ActiveGameId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "ActiveGameId", skip_serializing_if = "Option::is_none")]
    pub active_game_id: Option<String>,
    #[serde(rename = "Games", skip_serializing_if = "Option::is_none")]
    pub games: Option<Vec<String>>,
}
//
// #[cfg(test)]
// mod tests {
//     use super::super::RootSortKey;
//     use super::UserSortKey;
//
//     #[test]
//     fn test_user_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let user_id = "12345";
//         let variants = vec![(
//             "12345#User#ActiveGameId",
//             RootSortKey::User(user_id, UserSortKey::ActiveGameId),
//         )];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
