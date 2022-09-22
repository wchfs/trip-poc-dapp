CREATE TABLE zones (
    id INTEGER NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    start_hour TEXT NOT NULL,
    end_hour TEXT NOT NULL,
    geo_json TEXT NOT NULL,
    owner_address TEXT NOT NULL
)
