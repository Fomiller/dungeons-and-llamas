use crate::models::user::*;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::{routing::post, Router};
use axum_macros::debug_handler;
use dnl_store::Store;
use serde_json::json;

pub fn game_router() -> Router {
    let router: Router = Router::new().route("/new", post(post_new_game_handler));
    Router::new().nest("/game", router)
}

#[debug_handler]
pub async fn post_new_game_handler(Json(payload): Json<User>) -> impl IntoResponse {
    let client = Store::new().await;

    match client.try_new_game(&payload.id).await {
        Ok(_) => (
            StatusCode::CREATED,
            Json(json!({"detail": "new game created"})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": format!("{}",e)})),
        )
            .into_response(),
    }
}
