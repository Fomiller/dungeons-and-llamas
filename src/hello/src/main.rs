use anyhow::anyhow;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use lambda_http::{http::HeaderMap, run, service_fn, tracing, Body, Error, Request, Response};
use serde_json::json;
use std::env;

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

    let _ = validate_discord_signature(headers, event.body(), &PUB_KEY);
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

    let body_json: serde_json::Value =
        serde_json::from_str(std::str::from_utf8(event.body()).expect("non utf-8 body")).unwrap();
    println!("BODY_JSON: {:?}", body_json);

    let msg_type = body_json.get("type").expect("type not found");
    println!("MSG TYPE: {:?}", msg_type);
    match msg_type
        .to_string()
        .parse::<u32>()
        .expect("unable to parse type")
    {
        1 => {
            let json = json!({"type": 1});
            let resp: Response<Body> = Response::builder()
                .status(200)
                .body(Body::Text(json.to_string()))
                .unwrap();
            return Ok(resp);
        }
        2 => {
            let json = json!({"type": 4,"data": {"content": "Hello, World."}});
            let resp: Response<Body> = Response::builder()
                .status(200)
                .header("content-type", "application/json")
                .body(Body::Text(json.to_string()))
                .unwrap();
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
