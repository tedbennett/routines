-- Add migration script here
CREATE TABLE IF NOT EXISTS session(
	id TEXT PRIMARY KEY NOT NULL,
	session TEXT NOT NULL,
	expiry TEXT
);
