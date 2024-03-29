-- Add migration script here
CREATE TABLE IF NOT EXISTS account(
	id TEXT PRIMARY KEY NOT NULL,
	provider TEXT NOT NULL,
	user_id BLOB NOT NULL,
	FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE ON UPDATE CASCADE
);
