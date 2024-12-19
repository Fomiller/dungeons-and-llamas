use crate::embedding::*;
use crate::llm::LlmHandler;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use db::*;
use lambda_runtime::tracing;

pub struct RagWorkflow {
    pub database: VectorDatabase,
    pub llm: LlmHandler,
    pub embedding_engine: EmbeddingEngine,
}

// RagHandler is a high level orchestrator that will offer common workflow
// patterns in the Rag Process
impl RagWorkflow {
    pub async fn new(llm: LlmHandler, embedding_engine: EmbeddingEngine) -> Self {
        let database = VectorDatabase::new().await;

        Self {
            database,
            llm,
            embedding_engine,
        }
    }

    pub async fn execute(
        &mut self,
        prompt: &str,
        user_id: &str,
        game_id: &str,
    ) -> anyhow::Result<String> {
        let vector = self.embedding_engine.try_create_vector(&prompt).await?;

        let neighbors = self
            .database
            .try_similarity_search(5, vector, user_id, game_id, false)
            .await?;

        tracing::info!("Neighbors: {:?}", neighbors);

        let contexts = VectorDatabase::get_context_from_neighbors(neighbors);
        tracing::debug!("Embedding Contexts: {:?}", contexts);

        let input = self.llm.create_input(contexts, prompt);
        tracing::debug!("INPUT: {}", input);

        self.llm.messages = vec![Message::builder()
            .role(ConversationRole::User)
            .content(ContentBlock::Text(input.to_string()))
            .build()?];

        let response = self.llm.converse().await;

        match response {
            Ok(_) => Ok(self.llm.get_converse_output_text()?),
            Err(e) => Err(anyhow::anyhow!("{}", e)),
        }
    }
}
