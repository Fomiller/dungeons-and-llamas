mod commands;

use anyhow::anyhow;
use commands::*;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use lambda_http::{http::HeaderMap, run, service_fn, tracing, Body, Error, Request, Response};
use serenity::all::CommandInteraction;
use serenity::builder::*;
use serenity::json;
use serenity::model::application::*;
use std::env;
use std::str::FromStr;
use strum::EnumString;

lazy_static::lazy_static! {
    static ref PUB_KEY: VerifyingKey = VerifyingKey::from_bytes(
        hex::decode(
            env::var("DISCORD_PUBLIC_KEY")
              .expect("Expected DISCORD_PUBLIC_KEY to be set in the environment")
        )
        .expect("Couldn't hex::decode the DISCORD_PUBLIC_KEY").as_slice().try_into().unwrap()
    )
    .expect("Couldn't create a PublicKey from DISCORD_PUBLIC_KEY bytes");
}

#[derive(Debug, PartialEq, EnumString)]
enum SlashCommands {
    #[strum(ascii_case_insensitive)]
    Class,
    #[strum(ascii_case_insensitive)]
    Roll,
    #[strum(ascii_case_insensitive)]
    NewGame,
    #[strum(ascii_case_insensitive)]
    ResumeGame,
    #[strum(ascii_case_insensitive)]
    ListGames,
}

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    println!("EVENT: {:?}", event);

    let headers = event.headers();
    println!("HEADERS: {:?}", headers);

    validate_discord_signature(headers, event.body(), &PUB_KEY).unwrap();

    let discord_command = json::from_slice::<Interaction>(event.body())?;

    println!("COMMAND: {:?}", discord_command);
    println!("MSG TYPE: {:?}", discord_command.kind());

    let response = match discord_command {
        // Discord rejects the interaction endpoints URL if pings are not acknowledged
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => handle_command(interaction).await?,
        _ => CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("Command not handled")),
        ),
    };
    let resp: Response<Body> = Response::builder()
        .status(200)
        .body(Body::Text(serde_json::to_string(&response)?))
        .unwrap();
    Ok(resp)
}

async fn handle_command(cmd: CommandInteraction) -> anyhow::Result<CreateInteractionResponse> {
    println!("NAME: {:?}", &cmd.data.name);
    let command_name = SlashCommands::from_str(&cmd.data.name).unwrap();
    println!("COMMAND NAME: {:?}", command_name);
    match command_name {
        SlashCommands::Class => {
            let class = &cmd
                .data
                .options
                .first()
                .expect("No options available")
                .value;
            Ok(CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("You chose the {} class", class.as_str().unwrap())),
            ))
        }
        SlashCommands::Roll => Roll::command(cmd),
        SlashCommands::NewGame => NewGame::command(cmd).await,
        SlashCommands::ResumeGame => ResumeGame::command(cmd).await,
        SlashCommands::ListGames => ListGames::command(cmd),
    }
}

pub fn validate_discord_signature(
    headers: &HeaderMap,
    body: &Body,
    pub_key: &VerifyingKey,
) -> anyhow::Result<()> {
    let sig_ed25519 = {
        let header_signature = headers
            .get("X-Signature-Ed25519")
            .ok_or(anyhow!("missing X-Signature-Ed25519 header"))?;
        let decoded_header = hex::decode(header_signature)?;

        let mut sig_arr: [u8; 64] = [0; 64];
        for (i, byte) in decoded_header.into_iter().enumerate() {
            sig_arr[i] = byte;
        }
        Signature::from_bytes(&sig_arr)
    };
    println!("ED245519: {}", sig_ed25519);

    let sig_timestamp = headers
        .get("X-Signature-Timestamp")
        .ok_or(anyhow!("missing X-Signature-Timestamp header"))?;
    println!("TIMESTAMP: {:?}", sig_timestamp);

    if let Body::Text(body) = body {
        let content = sig_timestamp
            .as_bytes()
            .iter()
            .chain(body.as_bytes().iter())
            .cloned()
            .collect::<Vec<u8>>();

        pub_key
            .verify(&content.as_slice(), &sig_ed25519)
            .map_err(anyhow::Error::msg)
    } else {
        Err(anyhow!("Invalid body type"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
