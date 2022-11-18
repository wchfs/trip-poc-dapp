PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS `proposal_votes` (
    id INTEGER NOT NULL PRIMARY KEY,
    vote INTEGER NOT NULL,
    proposal_id INTEGER NOT NULL,

    FOREIGN KEY (proposal_id) REFERENCES proposals (id)
);
