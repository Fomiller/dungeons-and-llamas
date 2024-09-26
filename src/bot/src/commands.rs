use std::ops::Deref;
use std::sync::atomic::Ordering;

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
    info!("DATA: {:?}", ctx.data());

    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let response = format!("Hello {}!", u.name);

    ctx.say(response).await?;

    Ok(())
}

/// Displays your or another user's account creation date
#[poise::command(slash_command, prefix_command)]
pub async fn vote(
    ctx: Context<'_>,
    #[description = "What to vote for"] choice: String,
) -> Result<(), anyhow::Error> {
    let num_votes = {
        let mut hash_map = ctx.data().votes.lock().unwrap();
        let num_votes = hash_map.entry(choice.clone()).or_default();
        *num_votes += 1;
        *num_votes
    };

    let response = format!("Successfully voted for {choice}. {choice} now has {num_votes} votes!");
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn count(ctx: Context<'_>) -> Result<(), anyhow::Error> {
    let count = {
        let mut count = ctx.data().count.lock().unwrap();
        *count += 1;
        *count
    };

    let response =
        format!("Successfully incremented counter by 1. counter is not at {count} votes!");
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Number to add to counter"] num: String,
) -> Result<(), anyhow::Error> {
    let count = {
        let mut count = ctx.data().count.lock().unwrap();
        *count += num.parse::<u32>().unwrap();
        *count
    };

    let response = format!("Counter is now {count}!");
    ctx.say(response).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn subtract(
    ctx: Context<'_>,
    #[description = "Number to subtract from count"] num: String,
) -> Result<(), anyhow::Error> {
    let count = {
        let mut count = ctx.data().count.lock().unwrap();
        *count -= num.parse::<u32>().unwrap();
        *count
    };

    let response = format!("Counter is now {count}!");
    ctx.say(response).await?;

    Ok(())
}
