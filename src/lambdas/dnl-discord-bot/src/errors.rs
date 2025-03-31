use serenity::builder::*;
use serenity::http::Http;
use serenity::model::application::*;

pub async fn handle_error<T>(
    http: &Http,
    cmd: &CommandInteraction,
    err: anyhow::Error,
) -> anyhow::Result<T> {
    let message = CreateInteractionResponseFollowup::new().content(format!("Error: {:?}", err));
    let _ = cmd.create_followup(http, message).await;
    Err(anyhow::anyhow!(err))
}
