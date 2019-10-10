-- Your SQL goes here
.echo on

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

CREATE INDEX IF NOT EXISTS time_idx   ON killmails(killmail_time);
CREATE INDEX IF NOT EXISTS system_idx ON killmails(solar_system_id);
CREATE INDEX IF NOT EXISTS moon_idx   ON killmails(moon_id);
CREATE INDEX IF NOT EXISTS war_idx    ON killmails(war_id);

CREATE INDEX IF NOT EXISTS a_ship_idx        ON attackers(ship_type_id);
CREATE INDEX IF NOT EXISTS a_alliance_idx    ON attackers(alliance_id);
CREATE INDEX IF NOT EXISTS a_character_idx   ON attackers(character_id);
CREATE INDEX IF NOT EXISTS a_corporation_idx ON attackers(corporation_id);
CREATE INDEX IF NOT EXISTS a_faction_idx     ON attackers(faction_id);
CREATE INDEX IF NOT EXISTS a_weapon_type_idx ON attackers(weapon_type_id);

CREATE INDEX IF NOT EXISTS v_ship_idx        ON victims(ship_type_id);
CREATE INDEX IF NOT EXISTS v_alliance_idx    ON victims(alliance_id);
CREATE INDEX IF NOT EXISTS v_character_idx   ON victims(character_id);
CREATE INDEX IF NOT EXISTS v_corporation_idx ON victims(corporation_id);
CREATE INDEX IF NOT EXISTS v_faction_idx     ON victims(faction_id);

CREATE INDEX IF NOT EXISTS items_idx ON items(item_type_id);




