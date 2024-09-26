mod commands;
use log::LevelFilter;
use poise::serenity_prelude as serenity;
use std::collections::HashMap;
use std::env;
use std::sync::atomic::AtomicU32;
use std::sync::Mutex;

// User data, which is stored and accessible in all command invocations
#[derive(Debug)]
struct Data {
    pub votes: Mutex<HashMap<String, u32>>,
    pub count: Mutex<u32>,
}

// struct Votes;
// impl TypeMapKey for Votes {
//     type Value = u32;
// }

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

    let commands = vec![
        commands::age(),
        commands::greet(),
        commands::vote(),
        commands::count(),
        commands::add(),
        commands::subtract(),
    ];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    votes: Mutex::new(HashMap::new()),
                    count: Mutex::new(0),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}
