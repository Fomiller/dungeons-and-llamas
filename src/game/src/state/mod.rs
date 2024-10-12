use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameState {
    #[serde(rename = "UserId")]
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message<'a> {
    #[serde(rename = "LastMessageToken")]
    pub last_message_token: &'a str,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
}

#[derive(strum::Display)]
pub enum UserSortKeys<'a> {
    #[strum(to_string = "{0}#ActiveGameId")]
    ActiveGameId(&'a str),
}

#[derive(strum::Display)]
pub enum MessageSortKeys<'a> {
    #[strum(to_string = "{0}#LastMessageToken")]
    LastMessageToken(&'a str),
}

#[derive(strum::Display)]
pub enum ItemSortKeys<'a> {
    #[strum(to_string = "{0}#Item#Weapons")]
    Weapons(&'a str),
    #[strum(to_string = "{0}#Item#Spells")]
    Spells(&'a str),
    #[strum(to_string = "{0}#Item#Armor")]
    Armor(&'a str),
}

#[derive(strum::Display)]
pub enum GameSortKeys<'a> {
    #[strum(to_string = "{0}#Enemies")]
    Enemies(&'a str),
    #[strum(to_string = "{0}#Level")]
    Level(&'a str),
    #[strum(to_string = "{0}#Round")]
    Round(&'a str),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentWeapon {
    pub name: String,
    pub price: u8,
    pub damage: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateComponentMessage<'a> {
    #[serde(rename = "LastMessageToken", skip_serializing_if = "Option::is_none")]
    pub last_message_token: Option<&'a str>,
}
