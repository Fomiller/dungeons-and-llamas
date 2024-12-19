use crate::embedding::*;
use crate::llm::LlmHandler;
use db::*;
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
