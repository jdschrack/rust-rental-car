# Learn Rust in 7 Days: Build a Rental Car REST API

A project-based course from Rust fundamentals to a database-backed web service.

Over seven days, you'll build **RustRent**, a rental car management API. The course covers ownership, borrowing, traits, errors, asynchronous programming, HTTP APIs, database persistence, and testing.

**Target stack:** Rust (edition 2024), Axum 0.8, Tokio 1, Serde, SQLite, SQLx, Cargo.

> This is a guided project rather than a complete ready-to-run repository. Days 1 and 2 offer executable examples; later days include intentional implementation exercises.

## Learning path

| Day | Topic | Deliverable | Time |
|---|---|---|---|
| 1 | Rust fundamentals | Vehicle model and business rules | 90–120 min |
| 2 | First REST API | Vehicle listing and creation | 90–120 min |
| 3 | CRUD and architecture | Vehicle and customer handlers | 120 min |
| 4 | Persistence | SQLite-backed API | 120–150 min |
| 5 | Reservations | Booking and conflict detection | 120–150 min |
| 6 | Automated tests | Unit and integration tests | 90–120 min |
| 7 | Production readiness | Configuration, logging, documentation, release build | 120–150 min |

## Target API endpoints

| Method | Path | Description |
|---|---|---|
| GET | `/health` | Health check |
| GET | `/vehicles` | List vehicles |
| GET | `/vehicles/{id}` | Get vehicle |
| POST | `/vehicles` | Add vehicle |
| PUT | `/vehicles/{id}` | Update vehicle |
| DELETE | `/vehicles/{id}` | Delete vehicle |
| GET | `/customers` | List customers |
| POST | `/customers` | Register customer |
| GET | `/reservations` | List reservations |
| POST | `/reservations` | Reserve vehicle |
| GET | `/reservations/{id}` | Find reservation |
| POST | `/reservations/{id}/cancel` | Cancel reservation |

**Core rule:** A vehicle cannot have overlapping active reservations.

---

# Day 1: Rust fundamentals and the vehicle domain

**Goal:** Write a working Rust program without a web framework or database.

## Set up on Arch Linux

```bash
sudo pacman -S --needed rustup base-devel sqlite
rustup default stable
rustup component add rustfmt clippy
rustc --version
cargo --version
cargo new rust-rent
cd rust-rent
cargo run
```

Cargo combines dependency management, builds, and tests in one tool.

## Rust concepts to recognize

| Rust | Rough comparison |
|---|---|
| `struct` | Go struct |
| `enum` | Scala algebraic data type |
| `impl` | Methods on a Go type |
| `Option<T>` | Scala `Option` |
| `Result<T, E>` | Explicit success/failure |
| `trait` | Interface-like abstraction |
| `&T` | Immutable reference |
| `&mut T` | Exclusive mutable reference |

Rust uses ownership and borrowing rather than garbage collection.

## Build a vehicle model

Replace `src/main.rs`:

```rust
#[derive(Debug, Clone, PartialEq)]
enum VehicleStatus {
    Available,
    Rented,
    Maintenance,
}

#[derive(Debug, Clone)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u16,
    daily_rate_cents: i64,
    status: VehicleStatus,
}

impl Vehicle {
    fn new(
        id: u32,
        make: String,
        model: String,
        year: u16,
        daily_rate_cents: i64,
    ) -> Self {
        Self {
            id,
            make,
            model,
            year,
            daily_rate_cents,
            status: VehicleStatus::Available,
        }
    }

    fn is_available(&self) -> bool {
        self.status == VehicleStatus::Available
    }

    fn display(&self) {
        println!(
            "#{}: {} {} ({}) - ${:.2}/day - {:?}",
            self.id,
            self.make,
            self.model,
            self.year,
            self.daily_rate_cents as f64 / 100.0,
            self.status
        );
    }

    fn rent(&mut self) -> Result<(), String> {
        if !self.is_available() {
            return Err("Vehicle is not available".to_string());
        }
        self.status = VehicleStatus::Rented;
        Ok(())
    }
}

fn main() {
    let mut car = Vehicle::new(
        1,
        "Toyota".to_string(),
        "Camry".to_string(),
        2025,
        6500,
    );

    car.display();

    match car.rent() {
        Ok(()) => println!("Vehicle rented successfully"),
        Err(message) => println!("Error: {message}"),
    }

    car.display();
    println!("Second rental: {:?}", car.rent());
}
```

Run:

```bash
cargo run
```

Expected main output:

```text
#1: Toyota Camry (2025) - $65.00/day - Available
Vehicle rented successfully
#1: Toyota Camry (2025) - $65.00/day - Rented
Second rental: Err("Vehicle is not available")
```

## Understand ownership and borrowing

- `&self` borrows a value without allowing mutation.
- `&mut self` allows an exclusive mutable borrow.
- `Result<(), String>` represents either success or an error.

Add:

```rust
fn print_vehicle_name(vehicle: &Vehicle) {
    println!("{} {}", vehicle.make, vehicle.model);
}
```

Calling `print_vehicle_name(&car)` repeatedly works because it borrows. In contrast:

```rust
let vehicle_a = car;
let vehicle_b = car; // Compile error: use of moved value
```

The first assignment moves ownership.

### Exercises

- [ ] Store three vehicles in a `Vec<Vehicle>`.
- [ ] Write a function that returns available vehicles.
- [ ] Add a `return_vehicle()` method.
- [ ] Validate that the daily rate is positive.
- [ ] Replace `String` errors with a custom error enum.

**Complete when:** You can explain ownership, `&self`, `&mut self`, and `Result`.

```bash
git init
git add .
git commit -m "day 1: vehicle domain and Rust fundamentals"
```

---

# Day 2: Build your first REST API

**Goal:** Expose vehicle data over HTTP using Axum.

## Install dependencies

```bash
cargo add axum@0.8
cargo add tokio@1 --features full
cargo add serde@1 --features derive
cargo add serde_json@1
```

Replace `src/main.rs` with:

```rust
use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum VehicleStatus {
    Available,
    Rented,
    Maintenance,
}

#[derive(Debug, Clone, Serialize)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u16,
    daily_rate_cents: i64,
    status: VehicleStatus,
}

#[derive(Debug, Deserialize)]
struct CreateVehicle {
    make: String,
    model: String,
    year: u16,
    daily_rate_cents: i64,
}

type VehicleStore = Arc<Mutex<Vec<Vehicle>>>;

#[derive(Clone)]
struct AppState {
    vehicles: VehicleStore,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        vehicles: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind server");

    println!("RustRent listening on http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Server failure");
}

async fn health() -> &'static str {
    "OK"
}

async fn list_vehicles(
    State(state): State<AppState>,
) -> Result<Json<Vec<Vehicle>>, StatusCode> {
    let vehicles = state
        .vehicles
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vehicles.clone()))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(payload): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<Vehicle>), StatusCode> {
    if payload.make.trim().is_empty()
        || payload.model.trim().is_empty()
        || payload.daily_rate_cents <= 0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut vehicles = state
        .vehicles
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let next_id = vehicles
        .iter()
        .map(|v| v.id)
        .max()
        .unwrap_or(0)
        + 1;

    let vehicle = Vehicle {
        id: next_id,
        make: payload.make,
        model: payload.model,
        year: payload.year,
        daily_rate_cents: payload.daily_rate_cents,
        status: VehicleStatus::Available,
    };

    vehicles.push(vehicle.clone());
    Ok((StatusCode::CREATED, Json(vehicle)))
}
```

## Try the API

Start it:

```bash
cargo run
```

In a second terminal:

```bash
curl http://localhost:3000/health

curl -i -X POST http://localhost:3000/vehicles \
  -H "Content-Type: application/json" \
  -d '{
    "make": "Toyota",
    "model": "Camry",
    "year": 2025,
    "daily_rate_cents": 6500
  }'

curl -s http://localhost:3000/vehicles | jq
```

The creation endpoint should return HTTP 201 with a JSON vehicle.

## Understand the pieces

- `#[tokio::main]` creates an asynchronous runtime.
- `async fn` returns a future that is executed when awaited or scheduled.
- `Arc<T>` enables shared ownership.
- `Mutex<T>` protects a mutable collection.
- `State<AppState>` extracts router state.
- `Json<CreateVehicle>` deserializes the request body.

This `Mutex` is only for teaching; avoid holding synchronous mutex locks across `.await`. On Day 4, replace it with SQLx pooling.

### Exercises

- [ ] Create five vehicles with POST requests.
- [ ] Return 400 for an invalid daily rate.
- [ ] Add a query parameter to filter by make.
- [ ] Explain why request DTOs differ from response models.
- [ ] Restart the server and observe that all data disappears.

**Complete when:** You can explain Axum, Tokio, Serde, and router state.

---

# Day 3: CRUD and application architecture

**Goal:** Organize modules and implement complete vehicle CRUD plus customer creation.

## Refactor the source tree

```text
src/
├── main.rs
├── domain/
│   ├── mod.rs
│   ├── vehicle.rs
│   ├── customer.rs
│   └── reservation.rs
├── handlers/
│   ├── mod.rs
│   ├── vehicles.rs
│   └── customers.rs
├── repository/
│   ├── mod.rs
│   └── memory.rs
├── error.rs
└── state.rs
```

In `src/domain/mod.rs`:

```rust
pub mod vehicle;
pub mod customer;
pub mod reservation;
```

In `src/main.rs` declare each top-level module:

```rust
mod domain;
mod handlers;
mod repository;
mod error;
mod state;
```

## Traits and repositories

Define a repository behavior contract:

```rust
pub trait VehicleRepository {
    fn find_all(&self) -> Result<Vec<Vehicle>, RepositoryError>;

    fn find_by_id(
        &self,
        id: u32,
    ) -> Result<Option<Vehicle>, RepositoryError>;
}
```

These types must be imported or defined in the module where this trait lives. Implement it using the in-memory store for now. Traits share some similarities with Go interfaces but also support associated types, defaults, and bounds.

## Add vehicle lookup

Using the shared state model from Day 2:

```rust
use axum::extract::Path;

async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<Vehicle>, StatusCode> {
    let vehicles = state
        .vehicles
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let vehicle = vehicles
        .iter()
        .find(|v| v.id == id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(vehicle))
}
```

Add to the router:

```rust
.route("/vehicles/{id}", get(get_vehicle))
```

The `{id}` format is Axum 0.8 syntax.

Test:

```bash
curl -i http://localhost:3000/vehicles/1
curl -i http://localhost:3000/vehicles/999
```

## Implement update and delete

Write handlers with these signatures:

```rust
async fn update_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(payload): Json<UpdateVehicle>,
) -> Result<Json<Vehicle>, StatusCode>
```

```rust
async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode>
```

Use:

```rust
.route(
    "/vehicles/{id}",
    get(get_vehicle)
        .put(update_vehicle)
        .delete(delete_vehicle),
)
```

Expected behavior: GET returns 200 or 404; PUT returns 200, 400, or 404; DELETE returns 204, 404, or 409. Reject deleting vehicles that have active rentals.

## Add customers

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
```

Implement `GET /customers` and `POST /customers`.

### Exercises

- [ ] Complete update and delete.
- [ ] Add customer registration.
- [ ] Define `AppError` variants for NotFound, Validation, Conflict, and Internal.
- [ ] Implement Axum's `IntoResponse` for errors.
- [ ] Return consistent JSON error objects.

**Complete when:** HTTP parsing and domain logic are separated.

---

# Day 4: SQLite persistence

**Goal:** Replace the in-memory store with a database.

## Add SQLx

```bash
cargo add sqlx@0.9 --no-default-features \
  --features runtime-tokio,sqlite,migrate,derive
```

## Create a migration

```bash
mkdir -p migrations
```

Create `migrations/202610250001_create_schema.sql`:

```sql
CREATE TABLE vehicles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER NOT NULL,
    daily_rate_cents INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'available',
    CHECK (daily_rate_cents > 0),
    CHECK (year BETWEEN 1900 AND 2200),
    CHECK (status IN ('available', 'rented', 'maintenance'))
);

CREATE TABLE customers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE
);

CREATE TABLE reservations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    vehicle_id INTEGER NOT NULL,
    customer_id INTEGER NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    total_cents INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'confirmed',
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id),
    FOREIGN KEY (customer_id) REFERENCES customers(id),
    CHECK (end_date > start_date),
    CHECK (total_cents >= 0),
    CHECK (status IN ('confirmed', 'cancelled'))
);

CREATE INDEX idx_reservations_vehicle_dates
ON reservations(vehicle_id, start_date, end_date);
```

Dates use ISO 8601 calendar strings. The end date is exclusive.

## Create the connection pool

```rust
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::str::FromStr;

async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://rust_rent.db")?
        .create_if_missing(true)
        .foreign_keys(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    Ok(pool)
}
```

Change app state to:

```rust
#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}
```

## Database row mapping

```rust
#[derive(Debug, sqlx::FromRow)]
struct VehicleRow {
    id: i64,
    make: String,
    model: String,
    year: i64,
    daily_rate_cents: i64,
    status: String,
}
```

Query all vehicles:

```rust
let vehicles = sqlx::query_as::<_, VehicleRow>(
    "SELECT id, make, model, year,
            daily_rate_cents, status
     FROM vehicles
     ORDER BY id"
)
.fetch_all(&pool)
.await?;
```

Insert a vehicle:

```rust
let result = sqlx::query(
    "INSERT INTO vehicles
        (make, model, year, daily_rate_cents)
     VALUES (?, ?, ?, ?)"
)
.bind(&payload.make)
.bind(&payload.model)
.bind(i64::from(payload.year))
.bind(payload.daily_rate_cents)
.execute(&pool)
.await?;

let new_id = result.last_insert_rowid();
```

These snippets assume the surrounding function defines `pool` and `payload`. Translate database row status strings and year integers into validated domain types rather than trusting arbitrary DB values.

### Exercises

- [ ] Migrate vehicle list and creation.
- [ ] Migrate update and delete.
- [ ] Migrate customers.
- [ ] Return 409 for duplicate email addresses.
- [ ] Restart and verify persistence.
- [ ] Reject deletion when reservation history exists.

**Complete when:** SQLite is the source of truth.

---

# Day 5: Rental reservations and conflict detection

**Goal:** Implement business rules safely across concurrent requests.

## Add date handling

```bash
cargo add chrono@0.4 --features serde
```

```rust
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreateReservation {
    vehicle_id: i64,
    customer_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
}
```

Example input:

```json
{
  "vehicle_id": 1,
  "customer_id": 1,
  "start_date": "2026-11-01",
  "end_date": "2026-11-04"
}
```

## Calculate price in cents

```rust
fn calculate_total(
    daily_rate_cents: i64,
    start: NaiveDate,
    end: NaiveDate,
) -> Result<i64, &'static str> {
    let days = (end - start).num_days();

    if days <= 0 {
        return Err("End date must follow start date");
    }

    daily_rate_cents
        .checked_mul(days)
        .ok_or("Rental price overflow")
}
```

Three days at $65/day equals `19500` cents.

## Date overlap rule

For half-open intervals `[start, end)`, overlapping reservations satisfy:

```text
requested_start < existing_end
AND
requested_end > existing_start
```

A booking ending November 4 does **not** conflict with one starting November 4.

Query existing overlaps:

```sql
SELECT COUNT(*)
FROM reservations
WHERE vehicle_id = ?
  AND status = 'confirmed'
  AND start_date < ?
  AND end_date > ?;
```

Rust binding example (inside a transaction connection):

```rust
let overlaps: (i64,) = sqlx::query_as(
    "SELECT COUNT(*)
     FROM reservations
     WHERE vehicle_id = ?
       AND status = 'confirmed'
       AND start_date < ?
       AND end_date > ?"
)
.bind(vehicle_id)
.bind(end_date.to_string())
.bind(start_date.to_string())
.fetch_one(&mut *connection)
.await?;
```

## Make checking and inserting atomic

Doing a read and a later insert outside one write transaction can allow double bookings.

For SQLite, acquire the write transaction **before** the overlap check:

```rust
let mut connection = pool.acquire().await?;

sqlx::query("BEGIN IMMEDIATE")
    .execute(&mut *connection)
    .await?;

// Within this transaction:
// 1. Validate vehicle and customer.
// 2. Reject unavailable/maintenance vehicles.
// 3. Check overlap.
// 4. Compute total.
// 5. Insert reservation.

sqlx::query("COMMIT")
    .execute(&mut *connection)
    .await?;
```

**Important:** This is an outline, not a complete transaction-safe handler. Ensure every error path rolls back before the connection is reused, or use an API that guarantees rollback. Do not return pooled connections carrying open manual transactions. PostgreSQL would need its own database-level concurrency design.

### Exercises

- [ ] Implement POST `/reservations`.
- [ ] Reject invalid date ranges.
- [ ] Reject missing customer or vehicle.
- [ ] Return 409 for overlapping dates.
- [ ] Allow adjacent bookings.
- [ ] Implement reservation cancellation.
- [ ] Ignore cancelled bookings in availability checks.

**Complete when:** Concurrent overlapping requests cannot both succeed.

---

# Day 6: Automated testing

**Goal:** Validate domain rules, HTTP behavior, and database persistence.

## Unit tests for price calculation

Place this in the same module as `calculate_total`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn date(year: i32, month: u32, day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[test]
    fn calculates_three_day_rental() {
        let result = calculate_total(
            6500,
            date(2026, 11, 1),
            date(2026, 11, 4),
        );

        assert_eq!(result, Ok(19500));
    }

    #[test]
    fn rejects_zero_day_rental() {
        let result = calculate_total(
            6500,
            date(2026, 11, 1),
            date(2026, 11, 1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_reversed_dates() {
        let result = calculate_total(
            6500,
            date(2026, 11, 5),
            date(2026, 11, 1),
        );

        assert!(result.is_err());
    }

    #[test]
    fn rejects_price_overflow() {
        let result = calculate_total(
            i64::MAX,
            date(2026, 11, 1),
            date(2026, 11, 4),
        );

        assert!(result.is_err());
    }
}
```

Run:

```bash
cargo test
```

## Pure date-overlap function

```rust
fn ranges_overlap(
    a_start: NaiveDate,
    a_end: NaiveDate,
    b_start: NaiveDate,
    b_end: NaiveDate,
) -> bool {
    a_start < b_end && a_end > b_start
}
```

Test an adjacent booking:

```rust
#[test]
fn consecutive_reservations_do_not_overlap() {
    let result = ranges_overlap(
        date(2026, 11, 1),
        date(2026, 11, 4),
        date(2026, 11, 4),
        date(2026, 11, 7),
    );

    assert!(!result);
}
```

## Integration testing plan

Create `tests/reservation_api.rs` and exercise the router using an isolated SQLite test database. Use `tower`'s `ServiceExt::oneshot` and `http-body-util` to inspect responses.

For each test:
1. Create a fresh test database.
2. Apply migrations.
3. Seed test vehicles and customers.
4. Send HTTP requests into the router.
5. Assert HTTP status and JSON.
6. Query persisted database state.

| Scenario | Expected |
|---|---|
| Create vehicle | 201 |
| Missing vehicle | 404 |
| Invalid date | 400 |
| Missing customer | 404 |
| Reserve available car | 201 |
| Overlapping reservation | 409 |
| Adjacent reservation | 201 |
| Cancel reservation | 200 |
| Two concurrent conflicting bookings | Exactly one succeeds |

## Code quality

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

Avoid panicking on recoverable request errors (`unwrap()` / `expect()` in handlers).

**Complete when:** Critical business invariants have passing automated tests.

---

# Day 7: Production readiness and documentation

**Goal:** Build an application other developers can configure, observe, and package.

## Logging and tracing

```bash
cargo add tracing
cargo add tracing-subscriber --features env-filter
cargo add tower-http --features trace
```

Initialize tracing in startup:

```rust
use tracing_subscriber::EnvFilter;

tracing_subscriber::fmt()
    .with_env_filter(
        EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rust_rent=info,tower_http=info".into())
    )
    .init();
```

Add the middleware to your router:

```rust
use tower_http::trace::TraceLayer;

let app = Router::new()
    // Your routes here
    .layer(TraceLayer::new_for_http())
    .with_state(state);
```

Run:

```bash
RUST_LOG=debug cargo run
```

Never log private customer data or secrets.

## Environment-based configuration

```rust
use std::env;

struct Config {
    database_url: String,
    listen_addr: String,
}

impl Config {
    fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://rust_rent.db".to_string()),
            listen_addr: env::var("LISTEN_ADDR")
                .unwrap_or_else(|_| "127.0.0.1:3000".to_string()),
        }
    }
}
```

## Consistent API errors

Example response:

```json
{
  "error": {
    "code": "VEHICLE_NOT_AVAILABLE",
    "message": "Vehicle 17 is already reserved for those dates"
  }
}
```

| Domain error | HTTP status |
|---|---|
| Invalid input | 400 |
| Record missing | 404 |
| Booking conflict | 409 |
| Unexpected database failure | 500 |

Log server-side details and avoid leaking raw database errors to callers.

## Document the API

Create `docs/api.md` with request/response examples for every endpoint. As a challenge, write `openapi.yaml` using OpenAPI 3.1 and preview it using Swagger UI.

## Release build

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
```

Run the resulting binary:

```bash
DATABASE_URL=sqlite://rust_rent.db \
LISTEN_ADDR=127.0.0.1:3000 \
./target/release/rust-rent
```

Keep it local for now. Before exposing this API publicly, add authentication, authorization, TLS, appropriate rate limits, and operational controls.

## Final acceptance checklist

- [ ] Create two vehicles.
- [ ] Register a customer.
- [ ] Reserve a vehicle for November 1–4.
- [ ] Attempt November 2–5 and receive 409.
- [ ] Reserve the same car for November 4–7.
- [ ] Cancel the first reservation.
- [ ] Confirm November 1–4 is available again.
- [ ] Restart and confirm persistence.
- [ ] Pass all tests, formatting, and Clippy checks.

---

# Suggested final project structure

```text
rust-rent/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── openapi.yaml
├── migrations/
│   └── 202610250001_create_schema.sql
├── docs/
│   └── api.md
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── error.rs
│   ├── state.rs
│   ├── domain/
│   │   ├── mod.rs
│   │   ├── vehicle.rs
│   │   ├── customer.rs
│   │   └── reservation.rs
│   ├── handlers/
│   │   ├── mod.rs
│   │   ├── vehicles.rs
│   │   ├── customers.rs
│   │   └── reservations.rs
│   ├── repository/
│   │   ├── mod.rs
│   │   ├── vehicles.rs
│   │   ├── customers.rs
│   │   └── reservations.rs
│   └── services/
│       ├── mod.rs
│       └── reservations.rs
└── tests/
    ├── vehicle_api.rs
    └── reservation_api.rs
```

# References

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Axum API documentation](https://docs.rs/axum/latest/axum/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx documentation](https://docs.rs/sqlx/latest/sqlx/)

## What to learn next

After the seven days, migrate to PostgreSQL and explore JWT authentication, role-based permissions, pagination, generated OpenAPI specs, Docker, and GitHub Actions CI. Pay extra attention to ownership, lifetimes, `Result`, traits, and the differences between Rust concurrency and Go goroutines.
