-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages (
    user_id INT REFERENCES users(id) NOT NULL,
    channel_id INT REFERENCES channels(id) NOT NULL,
    id SERIAL PRIMARY KEY,
    discord_id TEXT NOT NULL UNIQUE,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);