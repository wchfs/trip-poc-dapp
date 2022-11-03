PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS `balances` (
    id INTEGER NOT NULL PRIMARY KEY,
    zone_id INTEGER NOT NULL,
    amount TEXT NOT NULL,

    FOREIGN KEY (zone_id) REFERENCES zones (id)
);
