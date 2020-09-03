-- Your SQL goes here
CREATE TABLE IF NOT EXISTS attachments (
    message_id INT NOT NULL,
    id SERIAL PRIMARY KEY,
    filename TEXT NOT NULL,
    binary_data BYTEA NOT NULL
);