use crate::error::ApiError;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::post, Router};
use axum_macros::debug_handler;
use llm::llm::*;
use serde_json::json;

use crate::models::llm::LlmConverseInput;

pub fn llm_router() -> Router {
    let router: Router = Router::new().route("/converse", post(post_llm_converse));
    Router::new().nest("/llm", router)
}

#[debug_handler]
pub async fn post_llm_converse(
    Json(payload): Json<LlmConverseInput>,
) -> Result<Response, ApiError> {
    println!("Payload: {:?}", payload);

    let mut llm = LlmHandler::new(payload.model, payload.system, payload.instructions).await;

    let input = llm.create_input(None, &payload.prompt);

    llm.set_messages(&input)?;

    match llm.converse().await?.get_text() {
        Ok(text) => {
            let status = StatusCode::OK;
            let json = Json(json!({"detail": text}));
            Ok((status, json).into_response())
        }
        Err(e) => {
            let status = StatusCode::BAD_REQUEST;
            let json = Json(json!({"error": format!("{}",e)}));
            Ok((status, json).into_response())
        }
    }
}
