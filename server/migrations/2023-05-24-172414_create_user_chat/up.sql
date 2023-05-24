CREATE TABLE IF NOT EXISTS user_chat (
    id SERIAL PRIMARY KEY NOT NULL UNIQUE,
    user_id SERIAL REFERENCES users(id) ON DELETE CASCADE,
    chat_id VARCHAR(255) NOT NULL,
    date_joined TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    is_admin BOOLEAN     NOT NULL DEFAULT FALSE
);