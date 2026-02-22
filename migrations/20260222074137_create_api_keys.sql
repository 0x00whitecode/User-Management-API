-- Add migration script here
CREATE TABLE api_keys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES register_user(id) ON DELETE CASCADE,

    key_hash TEXT UNIQUE NOT NULL,
    name TEXT,

    expires_at TIMESTAMP,
    revoked BOOLEAN DEFAULT FALSE,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);