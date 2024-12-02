use std::env;

use aws_sdk_bedrockruntime::{Client as BedrockClient, Error as BedrockError};
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    pub prompt: String,
}

#[derive(Serialize)]
struct Response {
    message: String,
    output: Option<String>,
}

async fn call_bedrock(
    client: &BedrockClient,
    model_id: &str,
    prompt: &str,
) -> Result<String, BedrockError> {
    let response = client
        .invoke_model()
        .model_id(model_id)
        .body(
            format!(r#"{{"inputText":"{}"}}"#, prompt)
                .into_bytes()
                .into(),
        )
        .content_type("application/json")
        .send()
        .await?;

    // Assume the response has a field "outputText"
    let body = response.body();
    if let Ok(output) = std::str::from_utf8(body.as_ref()) {
        return Ok(output.to_string());
    } else {
        Ok("No output from model.".to_string())
    }
    //
    // if let Some(payload) = {
    //     if let Ok(output) = std::str::from_utf8(payload.as_ref()) {
    //         return Ok(output.to_string());
    //     }
    // }
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
