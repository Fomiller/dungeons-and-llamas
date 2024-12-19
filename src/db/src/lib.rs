pub mod models;
pub mod schema;

use anyhow;
use diesel::debug_query;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use models::NewEmbedding;
use models::*;
use pgvector::{Vector, VectorExpressionMethods};
use schema::*;
use std::env;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct VectorDatabase {
    conn: PgConnection,
}

impl VectorDatabase {
    pub async fn new() -> Self {
        Self {
            conn: Self::connect_to_database().await,
        }
    }

    pub async fn connect_to_database() -> PgConnection {
        let port = 5432;
        let rds_user = env::var("RDS_USERNAME").expect("RDS_USERNAME must be set");
        let rds_pass = env::var("RDS_PASSWORD").expect("RDS_PASSWORD must be set");
        let database_endpoint =
            env::var("DATABASE_ENDPOINT").expect("DATABASE_ENDPOINT must be set");
        let database_name = env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            rds_user, rds_pass, database_endpoint, port, database_name
        );

        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        connection
    }

    pub async fn try_insert_new_embedding(
        &mut self,
        embedding: &NewEmbedding<'_>,
    ) -> anyhow::Result<Embedding> {
        match diesel::insert_into(embeddings::table)
            .values(embedding)
            .returning(Embedding::as_returning())
            .get_result(&mut self.conn)
        {
            Ok(e) => Ok(e),
            Err(_) => Err(anyhow::anyhow!("Error inserting Embedding")),
        }
    }

    pub async fn try_similarity_search(
        &mut self,
        limit: i64,
        embedding: Vector,
        user_id: &str,
        game_id: &str,
        debug: bool,
    ) -> anyhow::Result<Vec<Embedding>> {
        let query = embeddings::table
            .filter(embeddings::user_id.eq(user_id))
            .filter(embeddings::game_id.eq(game_id))
            .filter(embeddings::type_.eq("output"))
            .order(embeddings::vector.l2_distance(embedding))
            .limit(limit)
            .select(Embedding::as_select());

        if debug {
            let debug = debug_query::<diesel::pg::Pg, _>(&query);
            println!("QUERY: {:?}", debug);
        }

        let neighbors = query.load(&mut self.conn).expect("Error finding neighbors");
        Ok(neighbors)
    }

    pub fn get_context_from_neighbors(neighbors: Vec<Embedding>) -> Vec<String> {
        let contexts = neighbors
            .into_iter()
            .map(|e| format!("\n{}\n", e.text))
            .collect::<Vec<String>>();

        contexts
    }
}
