use llm::embedding::EmbeddingConfig;
use llm::llm::*;

use aws_sdk_bedrockruntime::Client as BedrockClient;
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

    let config = aws_config::load_from_env().await;
    let bedrock_client = BedrockClient::new(&config);

    let model_id = env::var("MODEL_ID")?;

    let system = "You are a Dungeons and Dragons Dungeon Master.".to_string();

    let embedding_config = EmbeddingConfig::default();

    let llm = LlmHandler::new(bedrock_client, model_id, system, payload.instructions);

    let mut rag = RagWorkflow::new(llm, embedding_config).await;

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
