-- SQLite

CREATE VIEW combatants AS
SELECT
   K.killmail_id  AS killmail_id,
   K.killmail_time AS killmail_time,
     
   V.character_id AS victim_character_id,
   V.character_name AS victim_character_name,
   V.corporation_id AS victim_corporation_id,
   V.corporation_name AS victim_corporation_name,
   V.alliance_id AS victim_alliance_id,
   V.alliance_name AS victim_alliance_name,
   V.faction_id AS victim_faction_id,
   V.faction_name AS victim_faction_name,
   V.ship_id AS victim_ship_id,
   V.ship_name AS victim_ship_name,
   V.damage_taken AS victim_damage_taken,
   
   A.character_id AS attacker_character_id,
   A.character_name AS attacker_character_name,
   A.corporation_id AS attacker_corporation_id,
   A.corporation_name AS attacker_corporation_name,
   A.alliance_id AS attacker_alliance_id,
   A.alliance_name AS attacker_alliance_name,
   A.faction_id AS attacker_faction_id,
   A.faction_name AS attacker_faction_name,
   A.ship_id AS attacker_ship_id,
   A.ship_name AS attacker_ship_name,
   A.damage_done AS attacker_damage_done,
   A.weapon_id AS attacker_weaponr_id,
   A.weapon_name AS attacker_weapon_name
      
FROM named_victims V
JOIN named_attackers A ON V.killmail_id = A.killmail_id
JOIN named_killmails K  ON K.killmail_id = v.killmail_id