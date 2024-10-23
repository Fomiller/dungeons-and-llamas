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

    run(app).await
}
async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_api() -> Json<Value> {
    Json(json!({ "msg": "I am GET /api" }))
}
