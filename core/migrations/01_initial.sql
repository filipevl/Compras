CREATE TABLE categories (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    color_hex TEXT NOT NULL
);

CREATE TABLE products (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category_id TEXT,
    quantity TEXT NOT NULL,
    last_purchased_at TEXT,
    FOREIGN KEY(category_id) REFERENCES categories(id) ON DELETE SET NULL
);