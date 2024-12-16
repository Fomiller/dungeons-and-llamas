use db::models::*;
use db::schema::*;
use diesel::prelude::*;
use diesel::query_dsl::RunQueryDsl;
use diesel::Connection;
use diesel::PgConnection;
use pgvector::{Vector, VectorExpressionMethods};
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
async fn call_bedrock(
    client: &BedrockClient,
    model_id: &str,
    prompt: &str,
    context: &str,
    instructions: &str,
    user: &str,
    game: &str,
) -> anyhow::Result<String> {
    let mut conn = connect_to_database().await;
    let embedding_dimension = 256;

    let embed_body = serde_json::json!({
        "inputText": context,
        "dimensions": embedding_dimension,
        "normalize": true
    });

    let embed_body_prompt = serde_json::json!({
        "inputText": prompt,
        "dimensions": embedding_dimension,
        "normalize": true
    });

    let embed_response = client
        .invoke_model()
        .model_id("amazon.titan-embed-text-v2:0")
        .body(embed_body.to_string().into_bytes().into())
        .accept("application/json")
        .content_type("application/json")
        .send()
        .await;

    let embed_response_prompt = client
        .invoke_model()
        .model_id("amazon.titan-embed-text-v2:0")
        .body(embed_body_prompt.to_string().into_bytes().into())
        .accept("application/json")
        .content_type("application/json")
        .send()
        .await;

    let embedding = match embed_response {
        Ok(output) => {
            // Convert response bytes into a String
            let output_string = String::from_utf8(output.body.into_inner())
                .expect("Response body is not valid UTF-8");
            // println!("{}", output_string);

            // Parse JSON string into a serde_json::Value
            let json_response: serde_json::Value =
                serde_json::from_str(&output_string).expect("Response body is not valid JSON");

            // Extract relevant fields from the JSON response
            let embedding = json_response.get("embedding").unwrap();
            Ok(embedding.clone())
        }
        Err(e) => {
            println!("{:?}", e.as_service_error());
            Err(anyhow::anyhow!("{:?}", e.as_service_error()))
        }
    }?;

    let embedding_prompt = match embed_response_prompt {
        Ok(output) => {
            // Convert response bytes into a String
            let output_string = String::from_utf8(output.body.into_inner())
                .expect("Response body is not valid UTF-8");
            // println!("{}", output_string);

            // Parse JSON string into a serde_json::Value
            let json_response: serde_json::Value =
                serde_json::from_str(&output_string).expect("Response body is not valid JSON");

            // Extract relevant fields from the JSON response
            let embedding = json_response.get("embedding").unwrap();
            Ok(embedding.clone())
        }
        Err(e) => {
            println!("{:?}", e.as_service_error());
            Err(anyhow::anyhow!("{:?}", e.as_service_error()))
        }
    }?;

    let embedding_data = Vector::from(value_to_f32_slice(&embedding)?);
    let embedding_data_prompt = Vector::from(value_to_f32_slice(&embedding_prompt)?);

    // write context to database
    write_to_database(&mut conn, embedding_data, user, game, context, "output").await?;
    // write prompt to database
    write_to_database(
        &mut conn,
        embedding_data_prompt.clone(),
        user,
        game,
        prompt,
        "prompt",
    )
    .await?;

    let neighbors =
        similarity_search(&mut conn, 5, embedding_data_prompt.clone(), user, game).await?;

    println!("Neighbors: {:?}", neighbors);

    let embedding_contexts = neighbors
        .into_iter()
        .map(|e| format!("\n{}\n", e.text))
        .collect::<Vec<String>>();

    println!("Embedding Contexts: {:?}", embedding_contexts);

    let mut input_context = String::new();
    for context in embedding_contexts {
        input_context.push_str(&context)
    }

    let input = format!(
        "{}\n<Context>{}</Context>\n<Prompt>\n{}\n</Prompt>",
        instructions, input_context, prompt
    );
    println!("INPUT: {}", input);

    let response = client
        .converse()
        .model_id(model_id)
        .system(SystemContentBlock::Text(
            "You are a Dungeons and Dragons Dungeon Master.".to_string(),
        ))
        .messages(
            Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(input.to_string()))
                .build()?,
        )
        .send()
        .await;

    match response {
        Ok(output) => {
            let text = get_converse_output_text(output)?;
            Ok(text)
        }
        Err(e) => Err(anyhow::anyhow!("{:?}", e.as_service_error())),
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
        &payload.prompt,
        &payload.context,
        &payload.instructions,
        &payload.user_id,
        &payload.game_id,
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
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

async fn connect_to_database() -> PgConnection {
    let port = 5432;
    let rds_user = env::var("RDS_USERNAME").expect("RDS_USERNAME must be set");
    let rds_pass = env::var("RDS_PASSWORD").expect("RDS_PASSWORD must be set");
    let database_endpoint = env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        rds_user, rds_pass, database_endpoint, port, database_name
    );

    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
}

async fn write_to_database(
    conn: &mut PgConnection,
    embedding: Vector,
    user_id: &str,
    game_id: &str,
    text: &str,
    type_: &str,
) -> anyhow::Result<Embedding> {
    let new_embedding = NewEmbedding {
        embedding,
        user_id: user_id.to_string(),
        game_id: game_id.to_string(),
        text: text.to_string(),
        type_: type_.to_string(),
    };

    match diesel::insert_into(embeddings::table)
        .values(&new_embedding)
        .returning(Embedding::as_returning())
        .get_result(conn)
    {
        Ok(e) => Ok(e),
        Err(_) => Err(anyhow::anyhow!("Error inserting Embedding")),
    }
}

async fn similarity_search(
    conn: &mut PgConnection,
    limit: i64,
    embedding: Vector,
    user_id: &str,
    game_id: &str,
) -> anyhow::Result<Vec<Embedding>> {
    let query = embeddings::table
        .filter(embeddings::user_id.eq(user_id))
        .filter(embeddings::game_id.eq(game_id))
        .filter(embeddings::type_.eq("output"))
        .order(embeddings::embedding.l2_distance(embedding))
        .limit(limit)
        .select(Embedding::as_select());

    // let debug = debug_query::<diesel::pg::Pg, _>(&query);
    // println!("QUERY: {:?}", debug);

    let neighbors = query.load(conn).expect("Error finding neighbors");
    Ok(neighbors)
}
