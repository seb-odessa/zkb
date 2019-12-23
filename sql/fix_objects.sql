-- SQLite

CREATE TABLE IF NOT EXISTS objects_new(
    object_id INTEGER NOT NULL,
    category_id INTEGER NOT NULL,
    object_name TEXT NOT NULL,
    PRIMARY KEY(object_id, category_id) ON CONFLICT IGNORE,
    FOREIGN KEY(category_id) REFERENCES categories(category_id)
);
INSERT INTO objects_new SELECT * FROM objects;
DROP TABLE objects;
ALTER TABLE objects_new RENAME TO objects;