-- Your SQL goes here
.echo on

DROP INDEX IF EXISTS k_time_idx;
DROP INDEX IF EXISTS k_system_idx;
DROP INDEX IF EXISTS k_moon_idx;
DROP INDEX IF EXISTS k_war_idx;

DROP INDEX IF EXISTS a_ship_idx;
DROP INDEX IF EXISTS a_alliance_idx;
DROP INDEX IF EXISTS a_character_idx;
DROP INDEX IF EXISTS a_corporation_idx;
DROP INDEX IF EXISTS a_faction_idx;
DROP INDEX IF EXISTS a_weapon_type_idx;
DROP INDEX IF EXISTS a_killmail_idx;

DROP INDEX IF EXISTS v_ship_idx;
DROP INDEX IF EXISTS v_alliance_idx;
DROP INDEX IF EXISTS v_character_idx;
DROP INDEX IF EXISTS v_corporation_idx;
DROP INDEX IF EXISTS v_faction_idx;
DROP INDEX IF EXISTS v_killmail_idx;

DROP INDEX IF EXISTS i_type_idx;
DROP INDEX IF EXISTS i_killmail_idx;

DROP INDEX IF EXISTS c_category_name_idx;
DROP INDEX IF EXISTS o_object_name_idx;


VACUUM;
ANALYZE;
VACUUM;


