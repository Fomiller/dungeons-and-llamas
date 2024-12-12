use aws_sdk_s3::Client as S3Client;
use diesel::prelude::*;
use diesel::Connection;
use diesel::PgConnection;
use pgvector::Vector;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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
) -> anyhow::Result<String> {
    let mut conn = connect_to_database().await;
    let embedding_dimension = 256;

    let input = format!(
        "{}\n<Context>\n{}\n</Context>\n<Prompt>\n{}\n</Prompt>",
        instructions, context, prompt
    );

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
            println!("{}", output_string);

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
            println!("{}", output_string);

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

    write_to_database(&mut conn, embedding_data).await?;
    write_to_database(&mut conn, embedding_data_prompt).await?;

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

async fn download_db_from_s3(
    s3_client: &S3Client,
    bucket: &str,
    key: &str,
    local_path: &str,
) -> anyhow::Result<()> {
    let response = s3_client
        .get_object()
        .bucket(bucket)
        .key(key)
        .send()
        .await?;

    let body = response.body.collect().await?;
    let mut file = File::create(local_path).await?;
    file.write_all(&body.into_bytes()).await?;
    Ok(())
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

    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
}

async fn write_to_database(
    conn: &mut PgConnection,
    embedding: Vector,
) -> anyhow::Result<Embedding> {
    let new_embedding = NewEmbedding {
        embedding: Some(embedding),
    };

    match diesel::insert_into(embeddings::table)
        .values(&new_embedding)
        .get_result::<Embedding>(conn)
    {
        Ok(e) => Ok(e),
        Err(_) => Err(anyhow::anyhow!("Error inserting Embedding")),
    }
}

#[derive(Queryable)]
#[diesel(table_name = embeddings)]
pub struct Embedding {
    pub id: i32,
    pub embedding: Option<Vector>,
}

#[derive(Insertable)]
#[diesel(table_name = embeddings)]
pub struct NewEmbedding {
    pub embedding: Option<Vector>,
}

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (id) {
        id -> Int4,
        embedding -> Nullable<Vector>,
    }
}
