use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use game::store::Store;
use lambda_http::{run, tracing, Error};
use serde_json::json;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    id: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let api_router = Router::new().route("/game/new", post(post_new_game_handler));

    let app = Router::new().route("/", get(root)).nest("/api", api_router);

    run(app).await
}

#[debug_handler]
async fn root() -> impl IntoResponse {
    StatusCode::NO_CONTENT
}

#[debug_handler]
async fn post_new_game_handler(Json(payload): Json<User>) -> impl IntoResponse {
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
