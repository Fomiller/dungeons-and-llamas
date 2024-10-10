mod commands;
mod verify;

use commands::*;
use lambda_http::{run, service_fn, tracing, tracing::info, Body, Error, Request, Response};
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::json;
use serenity::model::application::*;
use std::str::FromStr;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("EVENT: {:?}", event);

    let headers = event.headers();
    info!("HEADERS: {:?}", headers);

    let body = event.body();
    info!("BODY: {:?}", body);

    verify::validate_discord_signature(headers, body, &verify::PUB_KEY).unwrap();

    let discord_command: Interaction = json::from_slice(body)?;

    info!("COMMAND: {:?}", discord_command);
    info!("MSG TYPE: {:?}", discord_command.kind());

    let response = match discord_command {
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => handle_command(interaction).await?,
        _ => commands::format_interaction_response(format!("Command not found.")),
    };

    let resp: Response<Body> = Response::builder()
        .status(200)
        .body(Body::Text(serde_json::to_string(&response)?))
        .unwrap();

    Ok(resp)
}

async fn handle_command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
    info!("NAME: {:?}", &cmd.data.name);

    let command_name = SlashCommands::from_str(&cmd.data.name).unwrap();
    info!("COMMAND NAME: {:?}", command_name);

    match command_name {
        SlashCommands::Class => Class::command(cmd),
        SlashCommands::Roll => Roll::command(cmd),
        SlashCommands::NewGame => NewGame::command(cmd).await,
        SlashCommands::ResumeGame => ResumeGame::command(cmd).await,
        SlashCommands::ListGames => ListGames::command(cmd),
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
