PRAGMA foreign_keys = ON;

CREATE TABLE vehicles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    make TEXT NOT NULL CHECK (length(trim(make)) > 0),
    model TEXT NOT NULL CHECK (length(trim(model)) > 0),
    year INTEGER NOT NULL,
    daily_rate_cents INTEGER NOT NULL CHECK (daily_rate_cents > 0),
    status TEXT NOT NULL DEFAULT 'ready'
        CHECK (status IN ('ready', 'maintenance'))
);

CREATE TABLE customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL CHECK (length(trim(name)) > 0),
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE reservations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vehicle_id INTEGER NOT NULL REFERENCES vehicles(id),
    customer_id INTEGER NOT NULL REFERENCES customers(id),
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
    status TEXT NOT NULL DEFAULT 'confirmed'
        CHECK (status IN ('confirmed', 'cancelled')),
    CHECK (end_date > start_date)
);

CREATE INDEX idx_reservations_vehicle_dates
  ON reservations(vehicle_id, start_date, end_date);
CREATE INDEX idx_reservations_customer
  ON reservations(customer_id);


