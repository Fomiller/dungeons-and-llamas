use aws_config::BehaviorVersion;
use aws_lambda_events::event::s3::S3Event;
use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde_json::Value;
use std::env;

lazy_static::lazy_static! {
    static ref DISCORD_COMMAND_URL: String = format!("https://discord.com/api/v10/applications/{}/commands", env::var("DISCORD_APP_ID").expect("Could not find DISCORD_APP_ID env var."));
}

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Value, Error> {
    println!("EVENT: {:?}", event);

    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let s3_client = aws_sdk_s3::client::Client::new(&config);

    let object = s3_client
        .get_object()
        .bucket("fomiller-dev-dungeons-and-llamas")
        .key("data/commands.json")
        .send()
        .await?;

    let bytes = object.body.collect().await?.into_bytes();
    let response = std::str::from_utf8(&bytes)?;
    let temp: Value = serde_json::from_str(&response).unwrap();

    println!("COMMANDS: {:?}", temp);

    let client = reqwest::Client::new();
    let res = client
        .put(DISCORD_COMMAND_URL.as_str())
        .header(
            "Authorization",
            format!("Bot {}", env::var("DISCORD_BOT_TOKEN")?),
        )
        .header("Content-Type", "application/json")
        .body(bytes)
        .send()
        .await?;

    Ok(res.json().await?)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
