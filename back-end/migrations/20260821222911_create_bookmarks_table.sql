-- Add migration script here
CREATE TABLE bookmarks (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    title TEXT NOT NULL,
    url TEXT NOT NULL,
    tags TEXT[] NOT NULL,
    created_at TIMESTAMPTZ
);
