-- Add migration script here
CREATE TABLE IF NOT EXISTS invite(
	id TEXT PRIMARY KEY NOT NULL,
	status TEXT NOT NULL,
	sender_id BLOB NOT NULL,
	created_at DATETIME NOT NULL,
	FOREIGN KEY(sender_id) REFERENCES user(id) ON DELETE CASCADE ON UPDATE CASCADE
);