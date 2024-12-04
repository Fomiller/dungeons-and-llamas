use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client as S3Client;
use rusqlite::{params, Connection};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use std::env;

use aws_sdk_bedrockruntime::{types::SystemContentBlock, Client as BedrockClient};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
// use rust_bert::pipelines::sentence_embeddings::{
//     SentenceEmbeddingsBuilder, SentenceEmbeddingsModelType,
// };
// use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    let input = format!(
        "{}\n<Context>\n{}\n</Context>\n<Prompt>\n{}\n</Prompt>",
        instructions, context, prompt
    );

    let embed_body = serde_json::json!({
        "inputText": context
    });

    let embed_response = client
        .invoke_model()
        .model_id("amazon.titan-embed-text-v2:0")
        .body(embed_body.to_string().into_bytes().into())
        .accept("application/json")
        .content_type("application/json")
        .send()
        .await;

    let s3_client = S3Client::new(&aws_config::load_from_env().await);
    let bucket = "fomiller-dev-dungeons-and-llamas-llm";
    let db_key = "embeddings/new.db";
    let local_path = "/tmp/embeddings.sqlite";
    //
    // Download database
    download_db_from_s3(&s3_client, &bucket, db_key, local_path).await?;

    // Write to database
    let embedding_id = "example-id";
    let embedding_data = vec![0.1, 0.2, 0.3]; // Example embedding

    write_to_sqlite(local_path, embedding_id, &embedding_data)?;

    // Upload updated database
    upload_db_to_s3(&s3_client, &bucket, db_key, local_path).await?;

    match embed_response {
        Ok(output) => {
            // Convert response bytes into a String
            let output_string = String::from_utf8(output.body.into_inner())
                .expect("Response body is not valid UTF-8");
            println!("{}", output_string);

            // Parse JSON string into a serde_json::Value
            let json_response: serde_json::Value =
                serde_json::from_str(&output_string).expect("Response body is not valid JSON");

            // Extract relevant fields from the JSON response
            if let Some(embedding) = json_response.get("embedding") {
                println!("Embedding: {}", embedding);
            } else {
                println!("No embedding found in the response.");
            }
        }
        Err(e) => println!("{:?}", e.as_service_error()),
    }

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

    // // Assume the response has a field "outputText"
    // let body = response.output.;
    // if let Ok(output) = std::str::from_utf8(body.as_ref()) {
    //     return Ok(output.to_string());
    // } else {
    //     Ok("No output from model.".to_string())
    // }
    //
    // if let Some(payload) = {
    //     if let Ok(output) = std::str::from_utf8(payload.as_ref()) {
    //         return Ok(output.to_string());
    //     }
    // }
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

// async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
//     // Extract some useful information from the request
//     println!("{:?}", event);
//
//     Ok(())
// }

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
fn write_to_sqlite(
    db_path: &str,
    embedding_id: &str,
    embedding_data: &[f32],
) -> anyhow::Result<()> {
    let conn = Connection::open(db_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS embeddings (id TEXT PRIMARY KEY, data BLOB)",
        [],
    )?;

    let data_blob = bincode::serialize(embedding_data)?; // Serialize f32 slice into a binary blob
    conn.execute(
        "INSERT OR REPLACE INTO embeddings (id, data) VALUES (?1, ?2)",
        params![embedding_id, data_blob],
    )?;

    Ok(())
}
async fn upload_db_to_s3(
    s3_client: &S3Client,
    bucket: &str,
    key: &str,
    local_path: &str,
) -> anyhow::Result<()> {
    let db_content = tokio::fs::read(local_path).await?;
    s3_client
        .put_object()
        .bucket(bucket)
        .key(key)
        .body(ByteStream::from(db_content))
        .send()
        .await?;
    Ok(())
}
