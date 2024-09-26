mod commands;
use log::LevelFilter;
use poise::serenity_prelude as serenity;
use std::env;

#[derive(Debug)]
struct Data {} // User data, which is stored and accessible in all command invocations

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    env_logger::builder()
        .format_timestamp(None)
        .format_module_path(false)
        .filter_module("bot", LevelFilter::Info)
        .init();

    let intents = serenity::GatewayIntents::non_privileged();

    let commands = vec![commands::age(), commands::greet()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
