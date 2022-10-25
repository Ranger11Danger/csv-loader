-- Your SQL goes here
CREATE TABLE scans (
    id SERIAL PRIMARY KEY,
    ip TEXT NOT NULL,
    region TEXT NOT NULL
)