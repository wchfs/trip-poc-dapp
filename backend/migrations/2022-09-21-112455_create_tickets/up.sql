CREATE TABLE tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    license TEXT NOT NULL,
    longitude INTEGER NOT NULL,
    latitude INTEGER NOT NULL,
    started_at TEXT NOT NULL,
    owner_address TEXT NOT NULL,
    purchased_at TEXT NOT NULL,
    duration INTEGER NOT NULL,
    zone_id INTEGER NOT NULL,
    status INTEGER NOT NULL
)
