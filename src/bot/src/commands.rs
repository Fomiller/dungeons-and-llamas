use crate::Data;
use log::info;
use poise::serenity_prelude as serenity;

type Context<'a> = poise::Context<'a, Data, anyhow::Error>;

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), anyhow::Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let response = format!("{}'s account was created at {}", u.name, u.created_at());

    ctx.say(response).await?;

    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn greet(
    ctx: Context<'_>,
    #[description = "Greet user"] user: Option<serenity::User>,
) -> Result<(), anyhow::Error> {
    println!("User: {:?}", user);
    info!("User: {:?}", user);
    info!("Context: {:?}", ctx);

    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let response = format!("Hello {}!", u.name);

    ctx.say(response).await?;

    Ok(())
}
