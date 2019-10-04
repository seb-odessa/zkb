-- Your SQL goes here
CREATE TABLE kills(
    killmail_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    killmail_hash TEXT NOT NULL,
    killmail_date DATE NOT NULL
);
CREATE INDEX IF NOT EXISTS dates_ids ON kills(killmail_date);