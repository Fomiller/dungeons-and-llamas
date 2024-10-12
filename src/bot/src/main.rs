mod commands;
mod verify;

use commands::*;
use lambda_http::{
    run, service_fn, tracing,
    tracing::{debug, info},
    Body, Error, Request, Response,
};
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
        Interaction::Modal(interaction) => {
            let content = format!(
                "custom_id: {:?}, kind: {:?}",
                interaction.data.custom_id, interaction.data.components
            );
            let message = CreateInteractionResponseMessage::new().content(content);
            CreateInteractionResponse::Message(message)
        }
        Interaction::Component(interaction) => {
            if interaction.data.custom_id == "class_menu" {
                Edit::command(interaction).await?;
                let message = CreateInteractionResponseMessage::new()
                    .embeds(vec![])
                    .content("EDITED")
                    .components(vec![]);
                // THIS LINE
                CreateInteractionResponse::UpdateMessage(message)
            } else {
                let content = format!(
                    "custom_id: {:?}, kind: {:?}",
                    interaction.data.custom_id, interaction.data.kind
                );
                let message = CreateInteractionResponseMessage::new().content(content);
                CreateInteractionResponse::UpdateMessage(message)
                // CreateInteractionResponse::Message(message)
            }
        }
        _ => commands::format_interaction_response(format!("Command not found.")),
    };
    debug!("RES MESSAGE: {:?}", response);
    debug!("SERIALIZE {}", serde_json::to_string(&response)?);
    let resp: Response<Body> = Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Body::Text(serde_json::to_string(&response)?))
        .unwrap();

    debug!("RESPONSE: {:?}", resp);

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
        SlashCommands::Buttons => Buttons::command(cmd),
        SlashCommands::Menu => Menu::command(cmd),
        SlashCommands::Text => Embed::command(cmd).await,
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // logging discussion for future reference
    // https://github.com/awslabs/aws-lambda-rust-runtime/discussions/672
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
