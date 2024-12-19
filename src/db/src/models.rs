use diesel::{Insertable, Queryable, QueryableByName, Selectable};
use pgvector::Vector;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, QueryableByName, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::embeddings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Embedding {
    pub id: i32,
    pub vector: Vector,
    pub user_id: String,
    pub game_id: String,
    pub text: String,
    pub type_: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::embeddings)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmbedding<'a> {
    pub vector: Vector,
    pub user_id: &'a str,
    pub game_id: &'a str,
    pub text: &'a str,
    pub type_: String,
}
