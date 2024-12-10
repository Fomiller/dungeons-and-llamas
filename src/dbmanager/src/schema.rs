// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (id) {
        id -> Int4,
        embedding -> Nullable<Vector>,
    }
}
