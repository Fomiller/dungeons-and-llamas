use axum::http::StatusCode;
use axum::{
    extract::Path,
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, service_fn, tracing, Body, Error, Request, RequestExt, Response};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let app = Router::new()
        .route("/", get(root))
        .route("/api", get(get_api));

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

async fn get_api() -> Json<Value> {
    Json(json!({ "msg": "I am GET /api" }))
}
