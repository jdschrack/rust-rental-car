PRAGMA foreign_keys = ON;

CREATE TABLE vehicles (
  id INTEGER PRIMARY KEY,
  vin TEXT NOT NULL UNIQUE CHECK (length(trim(vin)) > 0),
  make TEXT NOT NULL CHECK (length(trim(make)) > 0),
  model TEXT NOT NULL CHECK (length(trim(model)) > 0),
  year INTEGER NOT NULL CHECK (year BETWEEN 1980 AND 2100),
  daily_rate_cents INTEGER NOT NULL CHECK (daily_rate_cents > 0),
  status TEXT NOT NULL DEFAULT 'ready'
    CHECK (status IN ('ready', 'maintenance'))
) STRICT;

CREATE TABLE customers (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL CHECK (length(trim(name)) > 0),
  email TEXT NOT NULL UNIQUE COLLATE NOCASE CHECK (length(trim(email)) > 0)
) STRICT;

CREATE TABLE reservations (
  id INTEGER PRIMARY KEY,
  vehicle_id INTEGER NOT NULL REFERENCES vehicles(id),
  customer_id INTEGER NOT NULL REFERENCES customers(id),
  start_date TEXT NOT NULL CHECK (date(start_date) = start_date),
  end_date TEXT NOT NULL CHECK (date(end_date) = end_date),
  CHECK (end_date > start_date)
) STRICT;

CREATE INDEX idx_reservations_vehicle_dates
  ON reservations(vehicle_id, start_date, end_date);
CREATE INDEX idx_reservations_customer
  ON reservations(customer_id);

-- No double-booking (back-to-back rentals are allowed)
CREATE TRIGGER trg_reservations_no_overlap_ins
BEFORE INSERT ON reservations
BEGIN
  SELECT RAISE(ABORT, 'vehicle already reserved for those dates')
  WHERE EXISTS (
    SELECT 1 FROM reservations
    WHERE vehicle_id = NEW.vehicle_id
      AND NEW.start_date < end_date
      AND NEW.end_date   > start_date
  );
END;

CREATE TRIGGER trg_reservations_no_overlap_upd
BEFORE UPDATE OF vehicle_id, start_date, end_date ON reservations
BEGIN
  SELECT RAISE(ABORT, 'vehicle already reserved for those dates')
  WHERE EXISTS (
    SELECT 1 FROM reservations
    WHERE vehicle_id = NEW.vehicle_id
      AND id <> NEW.id
      AND NEW.start_date < end_date
      AND NEW.end_date   > start_date
  );
END;
