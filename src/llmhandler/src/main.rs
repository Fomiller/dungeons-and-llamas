use std::env;

use aws_sdk_bedrockruntime::Client as BedrockClient;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
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
) -> anyhow::Result<String> {
    let response = client
        .converse()
        .model_id(model_id)
        .messages(
            Message::builder()
                .role(ConversationRole::User)
                .content(ContentBlock::Text(prompt.to_string()))
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
    let prompt = &payload.prompt;

    match call_bedrock(&bedrock_client, &model_id, prompt).await {
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
