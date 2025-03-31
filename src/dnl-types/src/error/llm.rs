use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LLMError {
    #[error("converse error")]
    Converse,
}

impl IntoResponse for LLMError {
    fn into_response(self) -> Response {
        match self {
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": format!("{}",self.to_string())})),
            ),
        }
        .into_response()
    }
}
