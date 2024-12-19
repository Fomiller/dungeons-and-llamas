#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmConverseInput {
    pub model: String,
    pub prompt: String,
    pub system: String,
    pub instructions: String,
}
