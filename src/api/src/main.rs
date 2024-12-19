pub mod models;
pub mod routes;
pub mod services;

use crate::game::game_router;
use axum::{routing::get, Router};
use lambda_http::{run, tracing, Error};
use routes::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let api_router = Router::new().merge(game_router());

    let app = Router::new().route("/", get(root)).nest("/api", api_router);

    run(app).await
}
