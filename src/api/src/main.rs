use axum::http::StatusCode;
use axum::{
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use game::client::Client;
use lambda_http::{run, tracing, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(root))
        .route("/api/new-game", post(post_new_game_handler));

    // let api_router = Router::new()
    //     .route("/hello", get(hello_from_the_server))
    //     .route("/todos", post(add_todo))

    // let app = Router::new()
    //     .nest("/api", api_router)
    //     .layer(CorsLayer::permissive())
    //     .route("/", get(hello))
    //     .route("/todos", get(another_page))
    //     .route("/dashboard", get(dashboard_page))
    //     .with_state(Arc::clone(&state))
    //     .nest_service(
    //         "/assets",
    //         ServeDir::new(format!("{}/assets", assets_path.to_str().unwrap())),
    //     )
    //     .with_state(Arc::clone(&state));

    run(app).await
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct User {
    user_id: String,
}

#[debug_handler]
async fn post_new_game_handler(Json(payload): Json<User>) -> impl IntoResponse {
    let client = Client::new().await;
    match client.try_new_game(&payload.user_id).await {
        Ok(_) => (StatusCode::OK, Json(json!({"detail": "new game created"}))).into_response(),
        Err(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "error creating new game"})),
        )
            .into_response(),
    }
    // // Ok(Json())
    // let body = json!({
    //     "details": format!("new game created"),  // Include the error details in the response
    // });
    // Ok(LambdaResponse::builder()
    //     .status(StatusCode::OK)
    //     .body(Body::Text(body.to_string()))
    //     .unwrap())
}

// async fn post_new_game_wrapper(
//     payload: Json<User>,
// ) -> anyhow::Result<impl IntoResponse, (StatusCode, Json<Value>)> {
//     match post_new_game_handler(payload).await {
//         Ok(response) => response,
//         Err(err) => {
//             eprintln!("Error creating new game: {:?}", err);
//
//             // Return a JSON error response
//             let error_body = json!({
//                 "error": "Failed to create new game",
//                 "details": format!("{}", err),  // Include the error details in the response
//             });
//
//             LambdaResponse::builder()
//                 .status(StatusCode::BAD_REQUEST) // 400 Bad Request for invalid input
//                 .header("Content-Type", "application/json") // Set the response as JSON
//                 .body(Body::Text(error_body.to_string())) // Serialize the JSON response
//                 .unwrap()
//         }
//     }
// }
