CREATE TABLE kills(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_hash TEXT NOT NULL,
    killmail_date DATE NOT NULL
);
CREATE INDEX IF NOT EXISTS dates_ids ON kills(killmail_date);


CREATE TABLE IF NOT EXISTS killmails(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_time DATETIME NOT NULL,
    solar_system_id INTEGER NOT NULL,
    moon_id INTEGER,
    war_id INTEGER
);

CREATE TABLE IF NOT EXISTS attackers(
    attacker_id INTEGER NOT NULL PRIMARY KEY,
    killmail_id INTEGER NOT NULL,
    security_status REAL NOT NULL,
    final_blow BOOLEAN NOT NULL,
    damage_done INTEGER NOT NULL,
    ship_type_id INTEGER,
    alliance_id INTEGER,
    character_id INTEGER,
    corporation_id INTEGER,
    faction_id INTEGER,
    weapon_type_id INTEGER,
    FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
);

CREATE TABLE IF NOT EXISTS victims(
    victim_id INTEGER NOT NULL PRIMARY KEY,
    killmail_id INTEGER NOT NULL,
    ship_type_id INTEGER NOT NULL,
    damage_taken INTEGER NOT NULL,
    alliance_id	INTEGER,
    character_id	INTEGER,
    corporation_id	INTEGER,
    faction_id	INTEGER,
    FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
);

CREATE TABLE IF NOT EXISTS items(
    item_id INTEGER NOT NULL PRIMARY KEY,
    killmail_id INTEGER NOT NULL,
    item_type_id INTEGER NOT NULL,
    singleton INTEGER NOT NULL,
    flag INTEGER NOT NULL,
    quantity_destroyed INTEGER,
    quantity_dropped INTEGER,
    FOREIGN KEY(killmail_id) REFERENCES killmails(killmail_id)
);
