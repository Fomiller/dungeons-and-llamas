use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApiResponseError {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Generator(#[from] super::generator::GeneratorError),

    #[error("Failed store operation: {0}")]
    Store(#[from] super::store::StoreError),

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
