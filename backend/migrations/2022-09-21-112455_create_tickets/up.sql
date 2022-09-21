CREATE TABLE tickets (
    id INTEGER NOT NULL PRIMARY KEY,
    owner_address TEXT NOT NULL,
    purchased_at TEXT NOT NULL,
    zone_id INTEGER NOT NULL,
    status INTEGER NOT NULL
)
