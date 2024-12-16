use db::models::*;
use db::*;
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

#[derive(Debug)]
struct BedrockConverseError(String);
impl std::fmt::Display for BedrockConverseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Can't invoke. Reason: {}", self.0)
    }
}

impl std::error::Error for BedrockConverseError {}

impl From<&str> for BedrockConverseError {
    fn from(value: &str) -> Self {
        BedrockConverseError(value.to_string())
    }
}

impl From<&ConverseError> for BedrockConverseError {
    fn from(value: &ConverseError) -> Self {
        BedrockConverseError::from(match value {
            ConverseError::ModelTimeoutException(_) => "Model took too long",
            ConverseError::ModelNotReadyException(_) => "Model is not ready",
            _ => "Unknown",
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}

async fn call_bedrock(
    client: &BedrockClient,
    model_id: &str,
    prompt: String,
    context: String,
    instructions: String,
    user_id: String,
    game_id: String,
) -> anyhow::Result<String> {
    let mut conn = db::connect_to_database().await;
    let embedding_dimension = 256;

    let vector_context =
        Vector::from(try_create_embed(client, &context, embedding_dimension, true).await?);

    let vector_prompt =
        Vector::from(try_create_embed(client, &prompt, embedding_dimension, true).await?);

    // write context to database
    let ctx_embed = NewEmbedding {
        vector: vector_context,
        user_id: &user_id,
        game_id: &game_id,
        text: context.to_string(),
        type_: "output".to_string(),
    };
    try_insert_new_embedding(&mut conn, &ctx_embed).await?;

    // write prompt to database
    let prompt_embed = NewEmbedding {
        vector: vector_prompt,
        user_id: &user_id,
        game_id: &game_id,
        text: prompt.to_string(),
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

    let contexts = get_context_from_neighbors(neighbors);

    let input = create_input(contexts, instructions, prompt);

    //TODO reafactor this
    let message = Message::builder()
        .role(ConversationRole::User)
        .content(ContentBlock::Text(input.to_string()))
        .build()?;
    let system = "You are a Dungeons and Dragons Dungeon Master.".to_string();
    let response = converse(client, model_id, system, message).await;

    match response {
        Ok(output) => {
            let text = get_converse_output_text(output)?;
            Ok(text)
        }
        Err(e) => Err(anyhow::anyhow!("{}", e)),
    }
}

fn get_converse_output_text(output: ConverseOutput) -> anyhow::Result<String> {
    let text = output
        .output()
        .ok_or_else(|| anyhow::anyhow!("no output"))?
        .as_message()
        .map_err(|_| anyhow::anyhow!("output not a message"))?
        .content()
        .first()
        .ok_or_else(|| anyhow::anyhow!("no content in message"))?
        .as_text()
        .map_err(|_| anyhow::anyhow!("content is not text"))?
        .to_string();
    Ok(text)
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let (payload, _context) = event.into_parts();
    let config = aws_config::load_from_env().await;
    let bedrock_client = BedrockClient::new(&config);

    let model_id = env::var("MODEL_ID")?;

    match call_bedrock(
        &bedrock_client,
        &model_id,
        payload.prompt,
        payload.context,
        payload.instructions,
        payload.user_id,
        payload.game_id,
    )
    .await
    {
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

fn value_to_f32_slice(value: &Value) -> anyhow::Result<Vec<f32>> {
    if let Value::Array(array) = value {
        // Try to parse each element as f32
        let result: Result<Vec<f32>, _> = array
            .iter()
            .map(|v| {
                v.as_f64()
                    .ok_or_else(|| anyhow::anyhow!("Value is not a valid number"))
            })
            .map(|num| num.and_then(|n| Ok(n as f32)))
            .collect();

        result.map_err(|e| anyhow::anyhow!("Error parsing array: {}", e))
    } else {
        Err(anyhow::anyhow!("Value is not an array"))
    }
}

async fn try_create_embed(
    client: &BedrockClient,
    input_text: &str,
    dimensions: i64,
    normalize: bool,
) -> anyhow::Result<Vec<f32>> {
    let body = serde_json::json!({
        "inputText": input_text,
        "dimensions": dimensions,
        "normalize": normalize
    });

    let res = client
        .invoke_model()
        .model_id("amazon.titan-embed-text-v2:0")
        .body(body.to_string().into_bytes().into())
        .accept("application/json")
        .content_type("application/json")
        .send()
        .await;

    let embedding_prompt = match res {
        Ok(output) => {
            // Convert response bytes into a String
            let output_string = String::from_utf8(output.body.into_inner())
                .expect("Response body is not valid UTF-8");

            // Parse JSON string into a serde_json::Value
            let json_response: serde_json::Value =
                serde_json::from_str(&output_string).expect("Response body is not valid JSON");

            // Extract relevant fields from the JSON response
            let embedding = json_response.get("embedding").unwrap();
            let x = value_to_f32_slice(embedding);
            Ok(x)
        }
        Err(e) => {
            println!("{:?}", e.as_service_error());
            Err(anyhow::anyhow!("{:?}", e.as_service_error()))
        }
    };

    embedding_prompt.unwrap()
}

fn get_context_from_neighbors(neighbors: Vec<Embedding>) -> Vec<String> {
    let contexts = neighbors
        .into_iter()
        .map(|e| format!("\n{}\n", e.text))
        .collect::<Vec<String>>();

    tracing::debug!("Embedding Contexts: {:?}", contexts);
    contexts
}

fn create_input(contexts: Vec<String>, instructions: String, prompt: String) -> String {
    let mut input_context = String::new();
    for context in contexts {
        input_context.push_str(&context)
    }
    let input = format!(
        "{}\n<Context>{}</Context>\n<Prompt>\n{}\n</Prompt>",
        instructions, input_context, prompt
    );

    tracing::debug!("INPUT: {}", input);

    input
}

async fn converse(
    client: &BedrockClient,
    model_id: &str,
    system: String,
    message: Message,
) -> anyhow::Result<ConverseOutput> {
    let response = client
        .converse()
        .model_id(model_id)
        .system(SystemContentBlock::Text(system))
        .messages(message)
        .send()
        .await?;

    Ok(response)
}
