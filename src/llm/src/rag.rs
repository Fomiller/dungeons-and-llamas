use crate::embedding::*;
use crate::llm::LlmHandler;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use db::models::*;
use db::*;
use lambda_runtime::tracing;
use pgvector::Vector;
use serde_json::json;

pub struct RagWorkflow {
    pub database: VectorDatabase,
    pub llm: LlmHandler,
    pub embedding_config: EmbeddingConfig,
}

// RagHandler is a high level orchestrator that will offer common workflow
// patterns in the Rag Process
impl RagWorkflow {
    pub async fn new(llm: LlmHandler, embedding_config: EmbeddingConfig) -> Self {
        let database = VectorDatabase::new().await;

        Self {
            database,
            llm,
            embedding_config,
        }
    }

    pub async fn execute(
        &mut self,
        prompt: &str,
        user_id: &str,
        game_id: &str,
    ) -> anyhow::Result<String> {
        let vector_prompt = self.try_create_vector(&prompt).await?;

        let prompt_embed = NewEmbedding {
            vector: vector_prompt,
            user_id,
            game_id,
            text: &prompt,
            type_: "prompt".to_string(),
        };

        self.database
            .try_insert_new_embedding(&prompt_embed)
            .await?;

        let neighbors = self
            .database
            .try_similarity_search(
                5,
                prompt_embed.vector,
                prompt_embed.user_id,
                prompt_embed.game_id,
                false,
            )
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

    pub async fn try_create_vector(&mut self, input_text: &str) -> anyhow::Result<Vector> {
        //Todo create a struct for this
        let body = json!({
            "inputText": input_text,
            "dimensions": &self.embedding_config.dimension,
            "normalize": &self.embedding_config.normalize
        });

        let res = self
            .llm
            .client
            .invoke_model()
            .model_id(&self.embedding_config.model)
            .body(body.to_string().into_bytes().into())
            .accept("application/json")
            .content_type("application/json")
            .send()
            .await;

        match res {
            Ok(output) => {
                // Convert response bytes into a String
                let output_string = String::from_utf8(output.body.into_inner())
                    .expect("Response body is not valid UTF-8");

                // Parse JSON string into a serde_json::Value
                let json_response: serde_json::Value =
                    serde_json::from_str(&output_string).expect("Response body is not valid JSON");

                // Extract relevant fields from the JSON response
                let embedding = json_response.get("embedding").unwrap();
                Ok(embedding.to_vector()?)
            }
            Err(e) => {
                println!("{:?}", e.as_service_error());
                Err(anyhow::anyhow!("{:?}", e.as_service_error()))
            }
        }
    }
}
