pub mod schema;
use anyhow::anyhow;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lambda_runtime::{
    run, service_fn,
    tracing::{self, subscriber::fmt::format},
    Error, LambdaEvent,
};
use serde::{Deserialize, Serialize};
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Deserialize)]
struct Request {
    command: String,
}

#[derive(Serialize)]
struct Response {
    msg: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> anyhow::Result<Response> {
    let port = 5432;
    let rds_user = env::var("RDS_USERNAME").expect("RDS_USERNAME must be set");
    let rds_pass = env::var("RDS_PASSWORD").expect("RDS_PASSWORD must be set");
    let database_endpoint = env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        rds_user, rds_pass, database_endpoint, port, database_name
    );

    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_) => {
            println!("Database migrations applied successfully");
            Ok(Response {
                msg: "Success".to_string(),
            })
        }
        Err(e) => {
            println!("Error applying database migrations");
            println!("Error: {}", e);
            Err(anyhow!("Error applying database Migrations"))
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
