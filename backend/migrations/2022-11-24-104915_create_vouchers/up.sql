CREATE TABLE IF NOT EXISTS `vouchers` (
    id INTEGER NOT NULL PRIMARY KEY,
    epoch_index INTEGER NOT NULL,
    input_index INTEGER NOT NULL,
    voucher_index INTEGER NULL,
    requested_by TEXT NOT NULL
);
