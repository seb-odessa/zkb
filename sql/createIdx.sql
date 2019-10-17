-- Your SQL goes here
.echo on


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




