use std::env;

// SERENITY
// use serenity::async_trait;
// use serenity::model::channel::Message;
// use serenity::prelude::*;
// struct Handler;

use poise::serenity_prelude as serenity;
struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

// #[async_trait]
// impl EventHandler for Handler {
//     async fn message(&self, ctx: serenity::Context, msg: Message) {
//         if msg.content == "!ping" {
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
//                 println!("Error sending message: {why:?}");
//             }
//         }
//         if msg.content == "!hello" {
//             if let Err(why) = msg.channel_id.say(&ctx.http, "Hi!").await {
//                 println!("Error sending message: {why:?}");
//             }
//         }
//     }
// }

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // SERENITY SETUP
    // Set gateway intents, which decides what events the bot will be notified about
    // let intents = GatewayIntents::GUILD_MESSAGES
    //     | GatewayIntents::DIRECT_MESSAGES
    //     | GatewayIntents::MESSAGE_CONTENT;
    //
    // Create a new instance of the Client, logging in as a bot.
    // let mut client = Client::builder(&token, intents)
    //     .event_handler(Handler)
    //     .await
    //     .expect("Err creating client");
    // Start listening for events by starting a single shard
    // if let Err(why) = client.start().await {
    //     println!("Client error: {why:?}");
    // }

    // POISE SETUP
    let intents = serenity::GatewayIntents::non_privileged();
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age()],
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
