use crate::schema::SchemaVersion;

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateComponent {
    #[serde(rename = "UserId")]
    pub user_id: String,
    #[serde(rename = "StateComponent")]
    pub state_component: String,
    #[serde(rename = "SchemaVersion")]
    pub schema_version: SchemaVersion,
}
