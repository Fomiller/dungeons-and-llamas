use anyhow::anyhow;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use hex;
use lambda_http::{http::HeaderMap, Body};
use std::env;

lazy_static::lazy_static! {
    pub static ref PUB_KEY: VerifyingKey = VerifyingKey::from_bytes(
        hex::decode(
            env::var("DISCORD_PUBLIC_KEY")
              .expect("Expected DISCORD_PUBLIC_KEY to be set in the environment")
        )
        .expect("Couldn't hex::decode the DISCORD_PUBLIC_KEY").as_slice().try_into().unwrap()
    )
    .expect("Couldn't create a PublicKey from DISCORD_PUBLIC_KEY bytes");
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
