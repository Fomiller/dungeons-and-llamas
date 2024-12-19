pub mod models;
pub mod routes;
pub mod services;

use crate::game::game_router;
use crate::llm::llm_router;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use lambda_http::{run, tracing, Error};
use routes::*;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let api_router = Router::new().merge(game_router()).merge(llm_router());

    let app = Router::new().route("/", get(root)).nest("/api", api_router);

    run(app).await
}

// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
// Make our own error that wraps `anyhow::Error`.
struct ApiError(anyhow::Error);
// Tell axum how to convert `ApiError` into a response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, AppError>`. That way you don't need to do that manually.
impl<E> From<E> for ApiError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
