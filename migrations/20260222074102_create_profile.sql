-- Add migration script here
CREATE TABLE profile (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID UNIQUE REFERENCES register_user(id) ON DELETE CASCADE,
    profile_picture TEXT,
    date_of_birth DATE NOT NULL,
    nationality TEXT,
    state_of_origin VARCHAR(100),
    bvn VARCHAR(20) NOT NULL,
    nin VARCHAR(20) NOT NULL,
    address TEXT,
    bank_account VARCHAR(20),
    bank_name TEXT,
    phone_number VARCHAR(20),
    kyc_verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);