CREATE TABLE zones (
    id INTEGER NOT NULL PRIMARY KEY,
    price_per_minute REAL NOT NULL,
    geo_json TEXT NOT NULL,
    owner_address TEXT NOT NULL
)
