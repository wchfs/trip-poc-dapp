PRAGMA foreign_keys = ON;

CREATE TABLE zones (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price TEXT NOT NULL,
    geo_json TEXT NOT NULL,
    owner_address TEXT NOT NULL
);
