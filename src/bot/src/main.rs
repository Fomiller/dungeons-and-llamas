mod commands;
mod components;
mod modals;
mod verify;

use commands::*;
use components::*;
use lambda_http::{
    run, service_fn, tracing,
    tracing::{debug, info},
    Body, Error, Request, Response,
};
use modals::*;
use serenity::builder::*;
use serenity::json;
use serenity::model::application::*;

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    info!("EVENT: {:?}", event);
    let headers = event.headers();
    info!("HEADERS: {:?}", headers);
    let body = event.body();
    info!("BODY: {:?}", body);

    verify::try_validate_discord_signature(headers, body, &verify::PUB_KEY).unwrap();

    let interaction_type: Interaction = json::from_slice(body)?;
    info!("COMMAND: {:?}", interaction_type);
    info!("MSG TYPE: {:?}", interaction_type.kind());

    let response = match interaction_type {
        Interaction::Ping(_) => CreateInteractionResponse::Pong,
        Interaction::Command(interaction) => try_handle_command_interaction(interaction).await?,
        Interaction::Modal(interaction) => try_handle_modal_interaction(interaction),
        Interaction::Component(interaction) => {
            try_handle_component_interaction(interaction).await?
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

#[tokio::main]
async fn main() -> Result<(), Error> {
    // logging discussion for future reference
    // https://github.com/awslabs/aws-lambda-rust-runtime/discussions/672
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
