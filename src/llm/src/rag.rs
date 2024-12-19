use crate::llm::LlmHandler;
use db::*;
use diesel::PgConnection;

pub struct RagHandler {
    pub db: PgConnection,
    pub llm: LlmHandler,
}

// RagHandler is a high level orchestrator that will offer common workflow
// patterns in the Rag Process
impl RagHandler {
    pub async fn new() -> Self {
        let config = aws_config::load_from_env().await;
        let bedrock_client = aws_sdk_bedrockruntime::Client::new(&config);

        let db = connect_to_database().await;
        let llm = LlmHandler::new(bedrock_client);

        Self { db, llm }
    }
}
