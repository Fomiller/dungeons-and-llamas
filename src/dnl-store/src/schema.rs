#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize, strum::Display)]
pub enum SchemaVersion {
    #[strum(to_string = "v1")]
    #[default]
    V1,
}
