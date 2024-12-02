use lambda_runtime::{run, service_fn, tracing, Error, LambdaEvent};
use serde_json::Value;

async fn function_handler(event: LambdaEvent<Value>) -> Result<(), Error> {
    // Extract some useful information from the request
    println!("{:?}", event);

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
