use anyhow::anyhow;
use db;
use diesel::connection::Connection;
use diesel::pg::PgConnection;
use diesel_migrations::MigrationHarness;
use lambda_runtime::{
    run, service_fn,
    tracing::{self},
    Error, LambdaEvent,
};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize)]
struct Request {}

#[derive(Serialize)]
struct Response {
    msg: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();
    run(service_fn(function_handler)).await
}

async fn function_handler(_event: LambdaEvent<Request>) -> anyhow::Result<Response> {
    let database_url = try_create_database_url()?;

    let mut connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    match connection.run_pending_migrations(db::MIGRATIONS) {
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

fn try_create_database_url() -> anyhow::Result<String> {
    //TODO read user,pass,endpoint all from secrets manager
    let port = 5432;
    let rds_user = env::var("RDS_USERNAME").expect("RDS_USERNAME must be set");
    let rds_pass = env::var("RDS_PASSWORD").expect("RDS_PASSWORD must be set");
    let database_endpoint = env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT must be set");
    let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let url = format!(
        "postgres://{}:{}@{}:{}/{}",
        rds_user, rds_pass, database_endpoint, port, database_name
    );

    Ok(url)
}
