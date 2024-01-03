-- Your SQL goes here
CREATE TABLE exchanges_rates (
                                 id INTEGER PRIMARY KEY AUTOINCREMENT,
                                 base_currency TEXT NOT NULL,
                                 target_currency TEXT NOT NULL,
                                 rate REAL NOT NULL,
                                 timestamp TEXT,
                                 UNIQUE(base_currency, target_currency)
)

