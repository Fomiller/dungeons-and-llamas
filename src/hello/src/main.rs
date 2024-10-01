use lambda_http::{run, service_fn, tracing, Body, Error, Request, Response};
use nacl::sign::verify;
use serde_json::json;

const PUBLIC_KEY: &str = "d2fe3cc5121565c24dbd2eaa1eb1f3890f0be4cb62cdee654aea9a3dabe7d6ea";

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    println!("EVENT: {:?}", event);

    let headers = event.headers();
    println!("HEADERS: {:?}", headers);

    let body = std::str::from_utf8(event.body()).expect("non utf-8 body");
    println!("BODY: {:?}", body);

    let timestamp = headers
        .get("x-signature-timestamp")
        .expect("missing x-signature-timestamp header")
        .to_str()
        .unwrap();

    let signature = headers
        .get("x-signature-ed25519")
        .expect("missing x-signature-ed25519 header")
        .to_str()
        .unwrap();

    let message = timestamp.to_owned() + body;

    println!("MESSAGE: {:?}", message);

    let _ = match verify(
        message.as_bytes(),
        signature.as_bytes(),
        PUBLIC_KEY.as_bytes(),
    ) {
        Ok(res) => res,
        Err(_) => {
            let resp: Response<Body> = Response::builder()
                .status(401)
                .header("content-type", "text/html")
                .body("invalid request signature".into())
                .map_err(Box::new)?;
            return Ok(resp);
        }
    };

    let body_json = json!(body);
    println!("BODY_JSON: {:?}", json!(body_json));

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
