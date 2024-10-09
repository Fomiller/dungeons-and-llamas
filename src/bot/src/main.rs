use anyhow::anyhow;
use dice::Dice;
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
    Hello,
    #[strum(ascii_case_insensitive)]
    Class,
    #[strum(ascii_case_insensitive)]
    Goodbye,
    #[strum(ascii_case_insensitive)]
    Roll,
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
        Interaction::Command(interaction) => handle_command(interaction),
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

fn handle_command(cmd: CommandInteraction) -> CreateInteractionResponse {
    println!("NAME: {:?}", &cmd.data.name);
    let command_name = SlashCommands::from_str(&cmd.data.name).unwrap();
    println!("COMMAND NAME: {:?}", command_name);
    match command_name {
        SlashCommands::Hello => CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("Hello, World!")),
        ),
        SlashCommands::Goodbye => CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new().content(format!("Goodbye, World!")),
        ),
        SlashCommands::Class => {
            let class = &cmd
                .data
                .options
                .first()
                .expect("No options available")
                .value;
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("You chose the {} class", class.as_str().unwrap())),
            )
        }
        SlashCommands::Roll => {
            let count = &cmd
                .data
                .options
                .iter()
                .find(|o| o.name == "count")
                .expect("could not find 'count' option for /roll")
                .value
                .as_i64()
                .expect("could not convert count.value to integer");

            let sides: &usize = &cmd
                .data
                .options
                .iter()
                .find(|o| o.name == "sides")
                .expect("Could not find 'sides' option for /roll")
                .value
                .as_i64()
                .expect("could not convert sides.value to integer")
                .try_into()
                .expect("could not convert to usize");

            let _modifier: &Option<&CommandDataOption> =
                &cmd.data.options.iter().find(|o| o.name == "modifier");

            let modifier: usize = match _modifier {
                Some(m) => m
                    .value
                    .as_i64()
                    .expect("could not convert modifier.value to integer")
                    .try_into()
                    .expect("could not convert to usize"),
                None => 0,
            };

            let dice: Vec<Dice> = (0..*count).map(|_| Dice::new(*sides)).collect();

            let dice_values: Vec<usize> =
                dice.into_iter().map(|d| d.roll()).collect::<Vec<usize>>();

            let roll: usize = dice_values.iter().sum();

            //TODO: make into function
            let roll_text = if modifier > 0 {
                format!(
                    "[{}{}] + {}",
                    dice_values[0],
                    dice_values[1..]
                        .iter()
                        .map(|v| format!(" + {}", v))
                        .collect::<Vec<String>>()
                        .join(""),
                    modifier
                )
            } else {
                format!("{:?}", dice_values)
            };

            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content(format!("Roll: {}\nYou rolled {}!", roll_text, roll)),
            )
        }
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
