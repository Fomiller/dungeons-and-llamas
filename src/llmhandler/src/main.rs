use db::models::*;
use db::*;
use llm::embedding::EmbeddingConfig;
use llm::llm::*;

use anyhow;
use aws_sdk_bedrockruntime::types::{ContentBlock, ConversationRole, Message};
use aws_sdk_bedrockruntime::Client as BedrockClient;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use llm::rag::RagWorkflow;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {
    pub prompt: String,
    pub instructions: String,
    pub context: String,
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

    let dimension = 256;

    let system = "You are a Dungeons and Dragons Dungeon Master.".to_string();

    let embedding_model = "amazon.titan-embed-text-v2:0".to_string();

    let embedding_config = EmbeddingConfig {
        model: embedding_model,
        dimension,
        normalize: true,
    };

    let llm = LlmHandler::new(bedrock_client, model_id, system);

    let mut rag = RagWorkflow::new(llm, embedding_config).await;

    let vector_context = rag.try_create_vector(&payload.context).await?;

    let vector_prompt = rag.try_create_vector(&payload.prompt).await?;

    // create embedding for context
    let ctx_embed = NewEmbedding {
        vector: vector_context,
        user_id: &payload.user_id,
        game_id: &payload.game_id,
        text: payload.context.to_string(),
        type_: "output".to_string(),
    };

    rag.database.try_insert_new_embedding(&ctx_embed).await?;

    // create embedding for prompt
    let prompt_embed = NewEmbedding {
        vector: vector_prompt,
        user_id: &payload.user_id,
        game_id: &payload.game_id,
        text: payload.prompt.to_string(),
        type_: "prompt".to_string(),
    };

    rag.database.try_insert_new_embedding(&prompt_embed).await?;

    let neighbors = rag
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

    let input = LlmHandler::create_input(contexts, payload.instructions, payload.prompt);
    tracing::debug!("INPUT: {}", input);

    rag.llm.messages = vec![Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(input.to_string()))
        .build()?];

    let response = rag.llm.converse().await;

    let res = match response {
        Ok(output) => Ok(LlmHandler::get_converse_output_text(output)?),
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    };

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
