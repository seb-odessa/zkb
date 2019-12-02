-- SQLite

CREATE TABLE IF NOT EXISTS objects_new(
    object_id INTEGER NOT NULL PRIMARY KEY ON CONFLICT IGNORE,
    category_id INTEGER NOT NULL,
    object_name TEXT NOT NULL,
    FOREIGN KEY(category_id) REFERENCES categories(category_id)
);
INSERT INTO objects_new SELECT * FROM objects;
DROP TABLE objects;
ALTER TABLE objects_new RENAME TO objects;