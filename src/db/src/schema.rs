// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::*;
    use pgvector::sql_types::*;

    embeddings (id) {
        id -> Int4,
        user_id -> Text,
        game_id -> Text,
        text -> Text,
        embedding -> Vector,
        #[sql_name = "type"]
        type_ -> Text,
    }
}
