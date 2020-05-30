-- SQLite

PRAGMA cache_size = 4000; 
PRAGMA synchronous = OFF; 
PRAGMA journal_mode = OFF;
PRAGMA locking_mode = EXCLUSIVE; 
PRAGMA count_changes = OFF; 
PRAGMA auto_vacuum = NONE;

BEGIN;

DELETE FROM items WHERE item_id IN (
SELECT items.item_id FROM killmails JOIN items ON killmails.killmail_id = items.killmail_id  WHERE killmails.killmail_time <  DATE('now','localtime', 'start of month','-6 month')
);

DELETE FROM attackers WHERE attacker_id IN (
SELECT attackers.attacker_id FROM killmails JOIN attackers ON killmails.killmail_id = attackers.killmail_id  WHERE killmails.killmail_time <  DATE('now','localtime', 'start of month','-6 month')
);

DELETE FROM victims WHERE victim_id IN (
SELECT victims.victim_id FROM killmails JOIN victims ON killmails.killmail_id = victims.killmail_id  WHERE killmails.killmail_time <  DATE('now','localtime', 'start of month','-6 month')
);

DELETE FROM killmails WHERE  killmails.killmail_time <  DATE('now','localtime', 'start of month','-6 month');

COMMIT;

VACUUM;

ANALYZE;



