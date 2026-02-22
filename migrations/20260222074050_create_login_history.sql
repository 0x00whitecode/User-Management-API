-- Add migration script here
CREATE TABLE login_user (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES register_user(id) ON DELETE CASCADE,
    device TEXT,
    os TEXT,
    browser TEXT,
    ip_address INET,
    latitude TEXT,
    longitude TEXT,
    signin_success BOOLEAN DEFAULT FALSE,
    failure_reason TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_login_user_user ON login_user(user_id);