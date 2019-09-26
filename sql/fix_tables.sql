-- SQLite
CREATE TABLE kills_new(
    killmail_id INTEGER NOT NULL PRIMARY KEY,
    killmail_hash BLOB NOT NULL,
    killmail_date INTEGER NOT NULL
);

INSERT INTO kills_new 
SELECT kills.id, hash, strftime('%s', printf("%.4d-%.2d-%.2d", year, month,day)) 
FROM kills,dates WHERE dates.id=kills.date_id;

DROP INDEX dates_idx;
DROP TABLE kills;
DROP TABLE IF EXISTS killmails;
DROP TABLE dates;

ALTER TABLE kills_new RENAME TO kills;

ANALYZE;
VACUUM;
