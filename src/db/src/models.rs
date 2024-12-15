use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use pgvector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, QueryableByName, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::embeddings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Embedding {
    pub id: i32,
    pub embedding: Vector,
    pub user_id: String,
    pub game_id: String,
    pub text: String,
    pub r#type: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::embeddings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmbedding {
    pub embedding: Vector,
    pub user_id: String,
    pub game_id: String,
    pub text: String,
    pub r#type: String,
}
