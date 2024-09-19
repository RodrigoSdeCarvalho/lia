-- Add migration script here
CREATE TABLE commands (
    id UUID PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    description TEXT,
    command_text TEXT NOT NULL,
    tags TEXT[],
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);
