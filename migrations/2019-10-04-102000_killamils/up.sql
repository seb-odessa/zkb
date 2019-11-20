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

CREATE TABLE IF NOT EXISTS systems(
    system_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    star_id INTEGER,
	security_status REAL NOT NULL,
	constellation_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS s_constellations_idx        ON systems(constellation_id);
CREATE INDEX IF NOT EXISTS s_security_status_idx       ON systems(security_status);

CREATE TABLE IF NOT EXISTS planets(
    planet_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
	type_id INTEGER NOT NULL,
	system_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS p_system_idx        ON planets(system_id);

CREATE TABLE IF NOT EXISTS constellations(
    constellation_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
	region_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS c_region_idx        ON constellations(region_id);

CREATE TABLE IF NOT EXISTS stargates(
    stargate_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
	type_id INTEGER NOT NULL,
	system_id INTEGER NOT NULL,
	dst_stargate_id INTEGER NOT NULL,
    dst_system_id INTEGER NOT NULL
);
CREATE INDEX IF NOT EXISTS s_system_idx        ON stargates(system_id);
CREATE INDEX IF NOT EXISTS s_dst_stargate_id   ON stargates(dst_stargate_id);
CREATE INDEX IF NOT EXISTS s_dst_system_id     ON stargates(dst_system_id);


DROP VIEW IF EXISTS named_victims;
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

DROP VIEW IF EXISTS named_attackers;
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
	facts.object_name  as faction_name,
	weapon_type_id	   as weapon_id,
	weapn.object_name  as weapon_name
FROM attackers
LEFT JOIN objects ships ON (ship_type_id = ships.object_id)
LEFT JOIN objects chars ON (character_id = chars.object_id)
LEFT JOIN objects corps ON (corporation_id = corps.object_id)
LEFT JOIN objects allis ON (alliance_id = allis.object_id)
LEFT JOIN objects facts ON (faction_id = facts.object_id)
LEFT JOIN objects weapn ON (weapon_type_id = weapn.object_id);

DROP VIEW IF EXISTS named_killmails;
CREATE VIEW IF NOT EXISTS named_killmails AS
SELECT
	killmail_id,
	killmail_time,
	system_names.object_id 			as system_id,
	system_names.object_name  		as system_name,
	constellation_names.object_id	as constellation_id,
    constellation_names.object_name as constellation_name,
	region_names.object_id			as region_id,
    region_names.object_name 		as region_name
FROM killmails
LEFT join systems ON (solar_system_id = systems.system_id)
LEFT join constellations ON (systems.constellation_id = constellations.constellation_id)
LEFT JOIN objects system_names ON (solar_system_id = system_names.object_id)
LEFT JOIN objects constellation_names ON (constellations.constellation_id = constellation_names.object_id)
LEFT JOIN objects region_names ON (constellations.region_id = region_names.object_id);

DROP VIEW IF EXISTS named_items;
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

DROP VIEW IF EXISTS named_constellations;
CREATE VIEW IF NOT EXISTS named_constellations AS
SELECT
	constellation_id,
	constellations_names.object_name AS constellation_name,
	constellations.region_id AS region_id,
	regions_names.object_name AS region_name
FROM constellations
JOIN objects constellations_names ON constellations.constellation_id = constellations_names.object_id
JOIN objects regions_names ON constellations.region_id = regions_names.object_id;

DROP VIEW IF EXISTS kills_10;
CREATE VIEW IF NOT EXISTS kills_10 AS
SELECT
	solar_system_id AS system_id,
	count(killmail_id) AS kills
FROM killmails
WHERE killmails.killmail_time >= Datetime('now',  "-10 minutes")
GROUP BY solar_system_id;

DROP VIEW IF EXISTS kills_60;
CREATE VIEW IF NOT EXISTS kills_60 AS
SELECT
	solar_system_id AS system_id,
	count(killmail_id) AS kills
FROM killmails
WHERE killmails.killmail_time >= Datetime('now', "-60 minutes")
GROUP BY solar_system_id;

DROP VIEW IF EXISTS kills_360;
CREATE VIEW IF NOT EXISTS kills_360 AS
SELECT
	solar_system_id AS system_id,
	count(killmail_id) AS kills
FROM killmails
WHERE killmails.killmail_time >= Datetime('now', "-360 minutes")
GROUP BY solar_system_id;


DROP VIEW IF EXISTS neighbors_regions;
CREATE VIEW IF NOT EXISTS neighbors_regions AS
SELECT DISTINCT
	own_c.region_id AS own_id,
	own_object.object_name AS own_name,
	neighbors_c.region_id AS neighbor_id,
	neighbors_object.object_name AS neighbor_name,
	sum(ifnull(kills_10.kills, 0)) AS ten_minutes,
	sum(ifnull(kills_60.kills, 0)) AS one_hour,
	sum(ifnull(kills_360.kills, 0)) AS six_hours
FROM stargates
JOIN systems own ON own.system_id = stargates.system_id
JOIN constellations own_c ON own.constellation_id = own_c.constellation_id
JOIN objects own_object ON own_c.region_id = own_object.object_id
JOIN systems neighbors ON neighbors.system_id = stargates.dst_system_id
JOIN constellations neighbors_c ON neighbors.constellation_id = neighbors_c.constellation_id
JOIN objects neighbors_object ON neighbors_c.region_id = neighbors_object.object_id
LEFT JOIN kills_10 ON neighbors.system_id = kills_10.system_id
LEFT JOIN kills_60 ON neighbors.system_id = kills_60.system_id
LEFT JOIN kills_360 ON neighbors.system_id = kills_360.system_id
WHERE neighbors_c.region_id != own_c.region_id
GROUP BY own_id, own_name, neighbor_id, neighbor_name;

DROP VIEW IF EXISTS neighbors_constellations;
CREATE VIEW IF NOT EXISTS neighbors_constellations AS
SELECT DISTINCT
	own.constellation_id AS own_id,
	own_object.object_name AS own_name,
	neighbors.constellation_id AS neighbor_id,
	neighbors_object.object_name AS neighbor_name,
	sum(ifnull(kills_10.kills, 0)) AS ten_minutes,
	sum(ifnull(kills_60.kills, 0)) AS one_hour,
	sum(ifnull(kills_360.kills, 0)) AS six_hours
FROM stargates
JOIN systems own ON own.system_id = stargates.system_id
JOIN objects own_object ON own.constellation_id = own_object.object_id
JOIN systems neighbors ON neighbors.system_id = stargates.dst_system_id
JOIN objects neighbors_object ON neighbors.constellation_id = neighbors_object.object_id
LEFT JOIN kills_10 ON neighbors.system_id = kills_10.system_id
LEFT JOIN kills_60 ON neighbors.system_id = kills_60.system_id
LEFT JOIN kills_360 ON neighbors.system_id = kills_360.system_id
WHERE neighbors.constellation_id != own.constellation_id
GROUP BY own_id, own_name, neighbor_id, neighbor_name;

DROP VIEW IF EXISTS neighbors_systems;
CREATE VIEW IF NOT EXISTS neighbors_systems AS
SELECT DISTINCT
	own.system_id AS own_id,
	own_object.object_name AS own_name,
	neighbors.system_id AS neighbor_id,
	neighbors_object.object_name AS neighbor_name,
	ifnull(kills_10.kills, 0) as ten_minutes,
	ifnull(kills_60.kills, 0) as one_hour,
	ifnull(kills_360.kills, 0) as six_hours
FROM stargates
JOIN systems own ON own.system_id = stargates.system_id
JOIN objects own_object ON own.system_id = own_object.object_id
JOIN systems neighbors ON neighbors.system_id = stargates.dst_system_id
JOIN objects neighbors_object ON neighbors.system_id = neighbors_object.object_id
LEFT JOIN kills_10 ON neighbors.system_id = kills_10.system_id
LEFT JOIN kills_60 ON neighbors.system_id = kills_60.system_id
LEFT JOIN kills_360 ON neighbors.system_id = kills_360.system_id
WHERE neighbors.system_id != own.system_id;


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




