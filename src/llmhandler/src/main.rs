use db::models::*;
use db::*;
use llm::*;
use pgvector::Vector;
use serde_json::Value;

use std::env;

use aws_sdk_bedrockruntime::{types::SystemContentBlock, Client as BedrockClient};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

use anyhow;
use aws_sdk_bedrockruntime::{
    operation::converse::{ConverseError, ConverseOutput},
    types::{ContentBlock, ConversationRole, Message},
};

#[derive(Deserialize)]
struct Request {
    pub prompt: String,
    pub instructions: String,
    pub context: String,
    pub user_id: String,
    pub game_id: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    output: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (payload, _context) = event.into_parts();
    let config = aws_config::load_from_env().await;
    let bedrock_client = BedrockClient::new(&config);

    let model_id = env::var("MODEL_ID")?;

    let mut conn = db::connect_to_database().await;
    let model = Llm::new(bedrock_client);
    let embedding_dimension = 256;

    let vector_context = Vector::from(
        model
            .try_create_embed(&payload.context, embedding_dimension, true)
            .await?,
    );

    let vector_prompt = Vector::from(
        model
            .try_create_embed(&payload.prompt, embedding_dimension, true)
            .await?,
    );

    // write context to database
    let ctx_embed = NewEmbedding {
        vector: vector_context,
        user_id: &payload.user_id,
        game_id: &payload.game_id,
        text: payload.context.to_string(),
        type_: "output".to_string(),
    };
    try_insert_new_embedding(&mut conn, &ctx_embed).await?;

    // write prompt to database
    let prompt_embed = NewEmbedding {
        vector: vector_prompt,
        user_id: &payload.user_id,
        game_id: &payload.game_id,
        text: payload.prompt.to_string(),
        type_: "prompt".to_string(),
    };
    try_insert_new_embedding(&mut conn, &prompt_embed).await?;

    let neighbors = db::try_similarity_search(
        &mut conn,
        5,
        prompt_embed.vector,
        prompt_embed.user_id,
        prompt_embed.game_id,
        false,
    )
    .await?;

    tracing::info!("Neighbors: {:?}", neighbors);

    let contexts = db::get_context_from_neighbors(neighbors);

    let input = Llm::create_input(contexts, payload.instructions, payload.prompt);
    tracing::debug!("INPUT: {}", input);

    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(input.to_string()))
        .build()?;
    let system = "You are a Dungeons and Dragons Dungeon Master.".to_string();
    let response = model.converse(&model_id, system, message).await;

    let res = match response {
        Ok(output) => {
            let text = Llm::get_converse_output_text(output)?;
            Ok(text)
        }
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    };

    match res {
        Ok(output) => Ok(Response {
            message: "Success".to_string(),
            output: Some(output),
        }),
        Err(e) => Ok(Response {
            message: format!("Failed to call model: {}", e),
            output: None,
        }),
    }
}
