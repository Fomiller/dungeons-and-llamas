use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("Failed to generate text")]
    GenerateTextError,

    #[error(transparent)]
    Store(#[from] super::store::StoreError),

    #[error(transparent)]
    Llm(#[from] super::llm::LLMError),

    #[error(transparent)]
    Other(#[from] anyhow::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    #[error(transparent)]
    SerdeDynamo(#[from] serde_dynamo::Error),
}

// Tell axum how to convert `ApiError` into a response.
impl IntoResponse for GeneratorError {
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
