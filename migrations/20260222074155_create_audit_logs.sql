-- Add migration script here
CREATE TABLE audit_logs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES register_user(id),

    action TEXT NOT NULL,
    entity TEXT,
    entity_id TEXT,

    ip_address INET,
    user_agent TEXT,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);