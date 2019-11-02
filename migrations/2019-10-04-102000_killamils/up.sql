-- Your SQL goes here

CREATE TABLE IF NOT EXISTS categories(
    category_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    category_name TEXT NOT NULL UNIQUE ON CONFLICT IGNORE
);

CREATE TABLE IF NOT EXISTS objects(
    object_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    category_id INTEGER NOT NULL,
    object_name TEXT NOT NULL UNIQUE ON CONFLICT IGNORE,
    FOREIGN KEY(category_id) REFERENCES categories(category_id)
);

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
    FOREIGN KEY(killmail_id)    REFERENCES killmails(killmail_id)
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

CREATE VIEW IF NOT EXISTS named_victims AS
SELECT
	victim_id,
	killmail_id,
	damage_taken,
	ship_type_id       as ship_id,
	ships.object_name  as ship_name,
	character_id,
	chars.object_name  as character_name,
	corporation_id,
	corps.object_name  as corporation_name,
	alliance_id,
	allis.object_name  as alliance_name,
	faction_id,
	facts.object_name  as faction_name
FROM victims
LEFT JOIN objects ships ON (ship_type_id = ships.object_id)
LEFT JOIN objects chars ON (character_id = chars.object_id)
LEFT JOIN objects corps ON (corporation_id = corps.object_id)
LEFT JOIN objects allis ON (alliance_id = allis.object_id)
LEFT JOIN objects facts ON (faction_id = facts.object_id);

CREATE VIEW IF NOT EXISTS named_attackers AS
SELECT
	attacker_id,
	killmail_id,
	damage_done,
	final_blow,
	security_status,
	ship_type_id       as ship_id,
	ships.object_name  as ship_name,
	character_id,
	chars.object_name  as character_name,
	corporation_id,
	corps.object_name  as corporation_name,
	alliance_id,
	allis.object_name  as alliance_name,
	faction_id,
	facts.object_name  as faction_name
FROM attackers
LEFT JOIN objects ships ON (ship_type_id = ships.object_id)
LEFT JOIN objects chars ON (character_id = chars.object_id)
LEFT JOIN objects corps ON (corporation_id = corps.object_id)
LEFT JOIN objects allis ON (alliance_id = allis.object_id)
LEFT JOIN objects facts ON (faction_id = facts.object_id);

CREATE VIEW IF NOT EXISTS named_killmails AS
SELECT
	killmail_id,
	killmail_time,
	solar_system_id  	 as system_id,
	systems.object_name  as system_name,
	moon_id,
	war_id
FROM killmails
LEFT JOIN objects systems ON (solar_system_id = systems.object_id)
LEFT JOIN objects moons ON (moon_id = moons.object_id);

CREATE VIEW IF NOT EXISTS named_items AS
SELECT
	item_id,
	killmail_id,
	item_type_id,
	object_name as item_type_name,
	quantity_destroyed,
	quantity_dropped,
	singleton,
	flag
FROM items LEFT JOIN objects ON (item_type_id = object_id);

CREATE INDEX IF NOT EXISTS k_time_idx        ON killmails(killmail_time);
CREATE INDEX IF NOT EXISTS k_system_idx      ON killmails(solar_system_id);
CREATE INDEX IF NOT EXISTS k_moon_idx        ON killmails(moon_id);
CREATE INDEX IF NOT EXISTS k_war_idx         ON killmails(war_id);

CREATE INDEX IF NOT EXISTS a_ship_idx        ON attackers(ship_type_id);
CREATE INDEX IF NOT EXISTS a_alliance_idx    ON attackers(alliance_id);
CREATE INDEX IF NOT EXISTS a_character_idx   ON attackers(character_id);
CREATE INDEX IF NOT EXISTS a_corporation_idx ON attackers(corporation_id);
CREATE INDEX IF NOT EXISTS a_faction_idx     ON attackers(faction_id);
CREATE INDEX IF NOT EXISTS a_weapon_type_idx ON attackers(weapon_type_id);
CREATE INDEX IF NOT EXISTS a_killmail_idx    ON attackers(killmail_id);

CREATE INDEX IF NOT EXISTS v_ship_idx        ON victims(ship_type_id);
CREATE INDEX IF NOT EXISTS v_alliance_idx    ON victims(alliance_id);
CREATE INDEX IF NOT EXISTS v_character_idx   ON victims(character_id);
CREATE INDEX IF NOT EXISTS v_corporation_idx ON victims(corporation_id);
CREATE INDEX IF NOT EXISTS v_faction_idx     ON victims(faction_id);
CREATE INDEX IF NOT EXISTS v_killmail_idx    ON victims(killmail_id);

CREATE INDEX IF NOT EXISTS i_type_idx        ON items(item_type_id);
CREATE INDEX IF NOT EXISTS i_killmail_idx    ON items(killmail_id);

CREATE INDEX IF NOT EXISTS c_category_name_idx    ON categories(category_name);
CREATE INDEX IF NOT EXISTS o_object_name_idx      ON objects(object_name);




