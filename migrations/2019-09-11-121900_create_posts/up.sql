
CREATE TABLE kills(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_hash TEXT NOT NULL,
    killmail_date DATE NOT NULL
);

/*
CREATE TABLE IF NOT EXISTS killmails(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_time TEXT NOT NULL,
    solar_system_id INTEGER NOT NULL,
    moon_id INTEGER,
    war_id INTEGER,
    victim_id INTEGER NOT NULL,
    attackers_id INTEGER NOT NULL
);
*/
