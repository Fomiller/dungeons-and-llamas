pub mod game;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_macros::debug_handler;

#[debug_handler]
pub async fn root() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}
