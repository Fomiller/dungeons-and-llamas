use crate::schema::SchemaVersion;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateComponent<T> {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "State")]
    pub state: Option<T>,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: SchemaVersion,
}
