-- Your SQL goes here
CREATE TABLE IF NOT EXISTS identities (
    user_id INT REFERENCES users(id) NOT NULL,
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    discriminator TEXT NOT NULL,
    nickname TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);