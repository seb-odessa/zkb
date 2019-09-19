-- Your SQL goes here
CREATE TABLE dates(
    id INTEGER NOT NULL PRIMARY KEY,
    year INTEGER NOT NULL,
    month INTEGER NOT NULL,
    day INTEGER NOT NULL
);
CREATE UNIQUE INDEX dates_idx ON dates(year, month, day);

CREATE TABLE kills(
    id INTEGER NOT NULL PRIMARY KEY,
    hash TEXT NOT NULL,
    date_id INTEGER NOT NULL,
    FOREIGN KEY (date_id) REFERENCES dates(id)
);