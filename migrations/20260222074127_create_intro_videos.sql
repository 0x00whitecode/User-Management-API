-- Add migration script here
CREATE TABLE intro_videos (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID REFERENCES register_user(id) ON DELETE CASCADE,
    description TEXT,
    file_type TEXT,
    duration INTERVAL,
    video_url TEXT NOT NULL,
    thumbnail_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);