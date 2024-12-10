-- Your SQL goes here
CREATE TABLE embeddings (
    id SERIAL PRIMARY KEY,
    embedding VECTOR(256)
);
