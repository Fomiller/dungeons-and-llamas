use anyhow::anyhow;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use lambda_http::{http::HeaderMap, run, service_fn, tracing, Body, Error, Request, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
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

// const PUBLIC_KEY: &str = "d2fe3cc5121565c24dbd2eaa1eb1f3890f0be4cb62cdee654aea9a3dabe7d6ea";

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    println!("EVENT: {:?}", event);

    let headers = event.headers();
    println!("HEADERS: {:?}", headers);

    // let body = std::str::from_utf8(event.body()).expect("non utf-8 body");
    // println!("BODY: {:?}", body);

    validate_discord_signature(headers, event.body(), &PUB_KEY).unwrap();
    // let timestamp = headers
    //     .get("x-signature-timestamp")
    //     .expect("missing x-signature-timestamp header")
    //     .to_str()
    //     .unwrap();
    //
    // let signature = headers
    //     .get("x-signature-ed25519")
    //     .expect("missing x-signature-ed25519 header")
    //     .to_str()
    //     .unwrap();
    //
    // let message = timestamp.to_owned() + body;
    //
    // println!("MESSAGE: {:?}", message);
    //
    // let _ = match verify(
    //     message.as_bytes(),
    //     signature.as_bytes(),
    //     PUBLIC_KEY.as_bytes(),
    // ) {
    //     Ok(res) => res,
    //     Err(_) => {
    //         let resp: Response<Body> = Response::builder()
    //             .status(401)
    //             .header("content-type", "text/html")
    //             .body("invalid request signature".into())
    //             .map_err(Box::new)?;
    //         return Ok(resp);
    //     }
    // };

    let discord_command: DiscordCommand =
        serde_json::from_str(std::str::from_utf8(event.body()).expect("non utf-8 body")).unwrap();
    println!("BODY_JSON: {:?}", discord_command);

    // let msg_type = body_json.get("type").expect("type not found");
    println!("MSG TYPE: {:?}", discord_command.command_type);
    match discord_command.command_type {
        1 => {
            let json = json!({"type": 1});
            let resp: Response<Body> = Response::builder()
                .status(200)
                .body(Body::Text(json.to_string()))
                .unwrap();
            println!("RES: {:?}", resp);
            return Ok(resp);
        }
        2 => {
            let name = discord_command.data.name;
            let cmd = Command::from_str(&name)?;
            // let opts: Option<Vec<CommandOptions>> = match discord_command.data.options {
            //     Some(opts) => Some(serde_json::from_value(opts.clone()).unwrap()),
            //     None => None,
            // };
            let json = handle_command(cmd, discord_command.data.options);
            let resp: Response<Body> = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text(json.to_string()))
                .unwrap();
            println!("RES: {:?}", resp);
            return Ok(resp);
        }
        _ => {
            let resp: Response<Body> = Response::builder()
                .status(400)
                .body("unhandled command".into())
                .unwrap();
            return Ok(resp);
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
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

#[derive(Debug, PartialEq, EnumString)]
enum Command {
    Hello,
    Class,
    Goodbye,
}

#[derive(Serialize, Deserialize)]
struct CommandOptions {
    name: String,
    value: String,
    _type: CommandType,
}

#[derive(Serialize, Deserialize)]
enum CommandType {
    SubCommand,      // 1
    SubCommandGroup, // 2
    String,          // 3
    Integer,         // 4 (Any integer between -2^53 and 2^53)
    Boolean,         // 5
    User,            // 6
    Channel,         // 7 (Includes all channel types + categories)
    Role,            // 8
    Mentionable,     // 9 (Includes users and roles)
    Number,          // 10 (Any double between -2^53 and 2^53)
    Attachment,      // 11 (Attachment object)
}

fn handle_command(cmd: Command, opts: Option<Vec<CommandOption>>) -> serde_json::Value {
    match cmd {
        Command::Hello => json!({"type": 4,"data": {"content": "Hello, World."}}),
        Command::Goodbye => json!({"type": 4,"data": {"content": "Goodbye, World."}}),
        Command::Class => {
            let class = opts
                .expect("Could not find options for command.")
                .first()
                .expect("No options available")
                .value
                .clone();
            let content = format!("You chose the {} class!", class);
            json!({"type": 4,"data": {"content": content}})
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DiscordCommand {
    #[serde(rename = "type")]
    command_type: u8, // Type of the command
    token: String,           // Token for the command
    member: Member,          // Member object
    id: String,              // Command ID
    guild_id: String,        // Guild ID
    app_permissions: String, // Application-specific permissions
    guild_locale: String,    // Locale of the guild
    locale: String,          // Locale of the user
    data: CommandData,       // Command data
    channel_id: String,      // Channel ID
}

#[derive(Debug, Serialize, Deserialize)]
struct Member {
    user: User,                    // User object
    roles: Vec<String>,            // List of role IDs
    premium_since: Option<String>, // Premium since (nullable)
    permissions: String,           // Permissions bitfield
    pending: bool,                 // Whether the user is pending
    nick: Option<String>,          // User's nickname (nullable)
    mute: bool,                    // Whether the user is muted
    joined_at: String,             // Date of user joining the guild
    is_pending: bool,              // Whether the user's membership is pending
    deaf: bool,                    // Whether the user is deafened
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,            // User ID
    username: String,      // Username
    avatar: String,        // Avatar hash
    discriminator: String, // User discriminator
    public_flags: u32,     // Public flags of the user
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandData {
    options: Option<Vec<CommandOption>>, // List of command options
    #[serde(rename = "type")]
    data_type: u8, // Type of command data
    name: String,                        // Command name
    id: String,                          // Command ID
}

#[derive(Debug, Serialize, Deserialize)]
struct CommandOption {
    #[serde(rename = "type")]
    option_type: u8, // Type of option
    name: String,  // Name of the option
    value: String, // Value of the option
}
