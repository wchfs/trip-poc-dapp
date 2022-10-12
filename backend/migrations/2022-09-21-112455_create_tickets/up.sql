PRAGMA foreign_keys = ON;

CREATE TABLE tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    license TEXT NOT NULL,
    longitude REAL NOT NULL,
    latitude REAL NOT NULL,
    started_at TEXT NOT NULL,
    owner_address TEXT NOT NULL,
    purchased_at TEXT NOT NULL,
    duration INTEGER NOT NULL,
    zone_id INTEGER NOT NULL,
    paid TEXT NOT NULL,
    to_pay TEXT NOT NULL,
    status INTEGER NOT NULL,

    FOREIGN KEY (zone_id) REFERENCES zones (id)
);
