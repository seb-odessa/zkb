-- Your SQL goes here
CREATE TABLE IF NOT EXISTS dates(
    id INTEGER NOT NULL PRIMARY KEY,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL
);
CREATE UNIQUE INDEX dates_idx ON dates(year, month, day);

CREATE TABLE IF NOT EXISTS kills(
    id INTEGER NOT NULL PRIMARY KEY,
    hash BLOB NOT NULL,
    date_id INTEGER NOT NULL,
    FOREIGN KEY (date_id) REFERENCES dates(id)
);

CREATE TABLE IF NOT EXISTS killmails(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_time TEXT NOT NULL,
    solar_system_id INTEGER NOT NULL,
    moon_id INTEGER,
    war_id INTEGER,
    victim_id INTEGER NOT NULL,
    attackers_id INTEGER NOT NULL
);