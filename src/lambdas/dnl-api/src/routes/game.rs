use crate::error::{handle_error, ApiError};
use dnl_store::Store;
use dnl_types::api::request::NewGameData;
use lambda_http::tracing::info;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::post, Router};
use axum_macros::debug_handler;

pub fn game_router() -> Router {
    let router: Router = Router::new().route("/new", post(post_new_game_handler));

    Router::new().nest("/game", router)
}

#[debug_handler]
pub async fn post_new_game_handler(Json(payload): Json<NewGameData>) -> Result<Response, ApiError> {
    info!("NewGameData: {:?}", payload);

    let client = Store::new().await;

    match client.try_new_game(payload).await {
        Ok(res) => {
            let res = (StatusCode::CREATED, Json(res)).into_response();
            info!("NewGameResponse: {:?}", res);
            return Ok(res);
        }
        Err(err) => {
            info!("New game error: {}", err);
            Ok(handle_error(format!("{:?}", err.to_string())))
        }
    }
}
