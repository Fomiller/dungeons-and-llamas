use llm::embedding::{EmbeddingConfig, EmbeddingEngine};
use llm::llm::*;

use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use llm::rag::RagWorkflow;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {
    pub prompt: String,
    pub instructions: String,
    pub user_id: String,
    pub game_id: String,
}

#[derive(Serialize)]
struct LambdaResponse {
    message: String,
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

//possibly parameterize the request object to include dimension,normalize, etc
async fn function_handler(event: LambdaEvent<Request>) -> Result<LambdaResponse, Error> {
    let (payload, _context) = event.into_parts();

    let model_id = env::var("MODEL_ID")?;

    let system = "You are a Dungeons and Dragons Dungeon Master.".to_string();

    let embed = EmbeddingEngine::new(EmbeddingConfig::default()).await;

    let llm = LlmHandler::new(model_id, system, payload.instructions).await;

    let mut rag = RagWorkflow::new(llm, embed).await;

    let res = rag
        .execute(&payload.prompt, &payload.user_id, &payload.game_id)
        .await;

    match res {
        Ok(output) => Ok(LambdaResponse {
            message: "Success".to_string(),
            output: Some(output),
        }),
        Err(e) => Ok(LambdaResponse {
            message: format!("Failed to call model: {}", e),
            output: None,
        }),
    }
}
