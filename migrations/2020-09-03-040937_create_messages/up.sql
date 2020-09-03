-- Your SQL goes here
CREATE TABLE IF NOT EXISTS messages (
    user_id INT NOT NULL,
    channel_id INT NOT NULL,
    id SERIAL PRIMARY KEY,
    discord_id TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);