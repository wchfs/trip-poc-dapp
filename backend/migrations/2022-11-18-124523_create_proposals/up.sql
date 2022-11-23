CREATE TABLE IF NOT EXISTS `proposals` (
    id INTEGER NOT NULL PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    status INTEGER NOT NULL,
    proposal_type INTEGER NOT NULL,
    created_at TEXT NOT NULL
);
