-- Your SQL goes here
CREATE TABLE IF NOT EXISTS channels (
    id SERIAL PRIMARY KEY,
    discord_id TEXT NOT NULL UNIQUE,
    name TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);