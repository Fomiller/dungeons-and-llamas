use serde::{Deserialize, Serialize};

#[derive(strum::Display)]
pub enum MessageSortKey {
    #[strum(to_string = "LastMessageToken")]
    LastMessageToken,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<'a> {
    #[serde(rename = "LastMessageToken")]
    pub last_message_token: &'a str,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentMessage<'a> {
    #[serde(rename = "LastMessageToken", skip_serializing_if = "Option::is_none")]
    pub last_message_token: Option<&'a str>,
}

// #[cfg(test)]
// mod tests {
//     use super::super::RootSortKey;
//     use super::MessageSortKey;
//
//     #[test]
//     fn test_message_sort_key() {
//         // maybe use EnumIter here, initial exploration did not work b/c of
//         // having to use a default
//         let user_id = "12345";
//         let variants = vec![(
//             "12345#Messages#LastMessageToken",
//             RootSortKey::Message(user_id, MessageSortKey::LastMessageToken),
//         )];
//         for variant in variants.iter() {
//             assert_eq!(variant.0, variant.1.to_string())
//         }
//     }
// }
