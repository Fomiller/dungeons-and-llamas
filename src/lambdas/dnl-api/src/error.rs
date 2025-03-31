use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use dnl_types::api::error::ApiResponseError;
use serde_json::json;
use thiserror::Error;

// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
// Make our own error that wraps `anyhow::Error`.
#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Generator(#[from] dnl_generators::Error),

    #[error("Failed store operation: {0}")]
    Store(#[from] dnl_store::Error),

    #[error("Failed vector database operation: {0}")]
    VectorDatabase(String),

    #[error("Unexpected error: {0}")]
    Unexpected(#[from] anyhow::Error),

    #[error("Serde Error: {0}")]
    Serde(#[from] serde_json::Error),
}

// Tell axum how to convert `ApiError` into a response.
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": format!("{}",self.to_string())})),
            ),
        }
        .into_response()
    }
}

pub fn handle_error(msg: String) -> Response {
    let error = ApiResponseError { error: msg };
    let res = (StatusCode::SERVICE_UNAVAILABLE, Json(error)).into_response();
    res
}
