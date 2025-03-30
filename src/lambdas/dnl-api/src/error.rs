use aws_sdk_dynamodb::error::SdkError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use dnl_types::api::error::ApiResponseError;
use serde_json::json;
use thiserror::Error;

// #[derive(thiserror::Error, Debug, serde::Serialize, serde::Deserialize)]
// pub enum ApiErrorEnum {
//     #[error("There was an error with the Store")]
//     StoreError,
//     #[error("There was an error with the Llm process")]
//     LlmError,
//     #[error("Unexpected error: {0}")]
//     Unexpected(String),
//     #[error("There was an error with the aws sdk: {0}")]
//     AwsError(String),
// }
//
// // Custom From<anyhow::Error> because anyhow does no implement Serialize/Deserialize
// impl From<anyhow::Error> for ApiErrorEnum {
//     fn from(err: anyhow::Error) -> Self {
//         ApiErrorEnum::Unexpected(err.to_string())
//     }
// }
//
// // Blanket implementation of generic AWS SdkError conversion
// impl<T> From<SdkError<T>> for ApiErrorEnum
// where
//     T: std::fmt::Display,
// {
//     fn from(err: SdkError<T>) -> Self {
//         ApiErrorEnum::AwsError(format!("{}", err))
//     }
// }

// // :TODO: handle different variants here
// impl IntoResponse for ApiErrorEnum {
//     fn into_response(self) -> Response {
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(json!({"error": format!("{}", self)})),
//         )
//             .into_response()
//     }
// }

// https://github.com/tokio-rs/axum/blob/main/examples/anyhow-error-response/src/main.rs
// Make our own error that wraps `anyhow::Error`.
#[derive(Error, Debug)]
pub enum ApiError {
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
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": format!("{}",self.to_string())})),
        )
            .into_response()
    }
}

pub fn handle_error(msg: String) -> Response {
    let error = ApiResponseError { error: msg };
    let res = (StatusCode::SERVICE_UNAVAILABLE, Json(error)).into_response();
    res
}
