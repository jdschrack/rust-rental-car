# RustRent — Seven-Day Rust REST API Course (Peer Review Corrections)

**Updated 26 October 2026.** This revision addresses the missing files, missing module declarations, incomplete handler implementations, absent SQLx glue, and omitted test wiring in the first edition.

**How to use this guide:** Execute each day's numbered steps **in order**, running its checkpoint before starting the next day. File paths are relative to the `rust-rent/` repository root. A **required** step is essential to the project; **stretch challenges** are optional. The Day 3 and final-day reference projects are supplied as separate ZIP downloads. The final ZIP also contains the complete API documentation and smoke script. Follow the exact file instructions and run each checkpoint. The reference has not been compiler-tested in this environment; report and resolve any compiler errors before continuing.

**Scope:** Educational local application, not a public production service. Authentication, authorization, payment processing, infrastructure hardening, and genuinely robust multitenant concurrency are outside scope.

## Course map

| Day | Outcome | Checkpoint |
|---|---|---|
| 1 | Vehicle domain, ownership, borrowing | `cargo run`, `cargo test` |
| 2 | In-memory vehicle HTTP API | POST and GET `/vehicles` |
| 3 | Modular in-memory CRUD and customers | `cargo check`; GET/POST/PUT/DELETE + customers |
| 4 | SQLite persistence using SQLx | Restart process; data persists |
| 5 | Reservations and conflict checks | Overlap -> 409; adjacent booking -> 201 |
| 6 | Automated domain/API tests | `cargo test` |
| 7 | Tracing, configuration, release, docs | `cargo clippy`; `cargo build --release` |

## Design choices that avoid ambiguity

- Days 1–2 use the tiny model from the introductory examples. **Day 3 intentionally replaces** the existing `main.rs` and moves models into modules. That day uses a simple `String` status and an in-memory `Arc<Mutex<_>>` store to focus on routing. Do not try to keep both the Day 2 and Day 3 versions of these types.
- **Day 4 intentionally replaces the in-memory application** with SQLx-backed handlers. Day 4 includes reservation-table migration in advance; reservation HTTP routes arrive on Day 5.
- Day 4 and later use SQLite integer identifiers (`i64`) consistently. Money is stored in integer cents, not floating point.
- The database's `status` field describes operational state (available or maintenance). We retain `rented` only for compatibility and do **not** set it during future bookings. Availability for a date interval is determined by reservations.
- A booking uses a half-open interval `[start_date, end_date)`. The return date is exclusive.
- In the final code, HTTP error responses are JSON and unexpected SQL errors are not revealed to clients.

## Prerequisites

Arch Linux shell, `rustup`, Rust stable with edition 2024 support, `curl`, and `jq`. Optional: VS Code/rust-analyzer. SQLx 0.9 requires **Rust 1.94.0 or newer** (the release-cycle MSRV). For SQLx, have a C compiler available (`base-devel`). We use the explicit `sqlite-bundled` feature to compile/link bundled SQLite; Arch `sqlite` supplies the `sqlite3` command-line tool used for inspection on Day 4. Commands are intended for a terminal located inside `rust-rent/`.

---

# Day 1: Rust fundamentals and the vehicle domain

**Goal:** Write a working Rust program without a web framework or database.

## Set up on Arch Linux

```bash
sudo pacman -S --needed rustup base-devel sqlite
rustup default stable
rustup update stable
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

### Required exercises

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

### Required exercises

- [ ] Create five vehicles with POST requests.
- [ ] Return 400 for an invalid daily rate.
- [ ] Add a query parameter to filter by make.
- [ ] Explain why request DTOs differ from response models.
- [ ] Restart the server and observe that all data disappears.

**Complete when:** You can explain Axum, Tokio, Serde, and router state.

---

---
# Day 3 — COMPLETE file-by-file refactor: CRUD and customers

**Starting point:** Your Day 2 application runs successfully. Stop the server. Commit your Day 2 work first (`git add . && git commit -m 'day 2 complete'`). The Day 3 snapshot below is **a replacement** of the Day 2 code, not an addition to it.

**Learning goals:** modules, visibility (`pub`), imports (`use`), application state, `Mutex`, HTTP errors, and separating domain models from handlers. Do not introduce SQLx yet.

### Step 1: create the required directories

```bash
mkdir -p src/domain src/handlers src/repository
```

At this stage create the following files. **All of these files are defined below.** There are no `reservation.rs`, `services/`, or `state.rs` files on Day 3 because they're not needed yet.

```text
src/
  main.rs
  error.rs
  domain/
    mod.rs
    vehicle.rs
    customer.rs
  handlers/
    mod.rs
    vehicles.rs
    customers.rs
  repository/
    mod.rs
    memory.rs
```

### Step 2: replace `Cargo.toml`

This is the full Day 3 manifest. It carries forward Day 2's dependencies without adding a database.

**`Cargo.toml`** — create this file with the following exact contents:

```toml
[package]
name = "rust-rent"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

### Step 3: domain models and module registration

`mod.rs` files make sibling modules visible; `pub` exposes fields/types to other modules. Begin with the module declarations, then the models.

**`src/domain/mod.rs`** — create this file with the following exact contents:

```rust
pub mod vehicle;
pub mod customer;
```

**`src/domain/vehicle.rs`** — create this file with the following exact contents:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Vehicle {
    pub id: u32,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicle {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVehicle {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
}

pub fn is_valid(make: &str, model: &str, rate: i64) -> bool {
    !make.trim().is_empty() && !model.trim().is_empty() && rate > 0
}
```

**`src/domain/customer.rs`** — create this file with the following exact contents:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct Customer {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
```

### Step 4: in-memory state and the repository module

The `Arc` handles shared ownership; the `Mutex` synchronizes each vector. The state is cloned as a lightweight handle into the router, not deep-copied on each request.

**`src/repository/mod.rs`** — create this file with the following exact contents:

```rust
pub mod memory;
```

**`src/repository/memory.rs`** — create this file with the following exact contents:

```rust
use std::sync::{Arc, Mutex};
use crate::domain::{customer::Customer, vehicle::Vehicle};

#[derive(Clone, Default)]
pub struct AppState {
    pub vehicles: Arc<Mutex<Vec<Vehicle>>>,
    pub customers: Arc<Mutex<Vec<Customer>>>,
}
```

### Step 5: define shared HTTP error responses

`IntoResponse` converts a domain/application error to an HTTP status and JSON body. An internal lock failure should not expose server internals.

**`src/error.rs`** — create this file with the following exact contents:

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError { BadRequest(String), NotFound(String), Conflict(String), Internal }

#[derive(Serialize)]
struct ErrorResponse { error: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error) = match self {
            Self::BadRequest(s) => (StatusCode::BAD_REQUEST, s),
            Self::NotFound(s) => (StatusCode::NOT_FOUND, s),
            Self::Conflict(s) => (StatusCode::CONFLICT, s),
            Self::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".into()),
        };
        (status, Json(ErrorResponse { error })).into_response()
    }
}
```

### Step 6: vehicle and customer handlers

Each handler extracts state and optional JSON/path inputs. Locks are held only for the short synchronous vector operation; no `.await` occurs while they are held. `create`, `get`, `update`, and `delete` all have complete bodies.

**`src/handlers/mod.rs`** — create this file with the following exact contents:

```rust
pub mod vehicles;
pub mod customers;
```

**`src/handlers/vehicles.rs`** — create this file with the following exact contents:

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::{domain::vehicle::{Vehicle, CreateVehicle, UpdateVehicle, is_valid}, error::AppError, repository::memory::AppState};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, AppError> {
    let cars = state.vehicles.lock().map_err(|_| AppError::Internal)?;
    Ok(Json(cars.clone()))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<u32>) -> Result<Json<Vehicle>, AppError> {
    let cars = state.vehicles.lock().map_err(|_| AppError::Internal)?;
    let car = cars.iter().find(|car| car.id == id).cloned()
        .ok_or_else(|| AppError::NotFound(format!("Vehicle {id} not found")))?;
    Ok(Json(car))
}

pub async fn create(State(state): State<AppState>, Json(input): Json<CreateVehicle>) -> Result<(StatusCode, Json<Vehicle>), AppError> {
    if !is_valid(&input.make, &input.model, input.daily_rate_cents) {
        return Err(AppError::BadRequest("Invalid vehicle".into()));
    }
    let mut cars = state.vehicles.lock().map_err(|_| AppError::Internal)?;
    let id = cars.iter().map(|car| car.id).max().unwrap_or(0) + 1;
    let car = Vehicle { id, make: input.make, model: input.model, year: input.year,
        daily_rate_cents: input.daily_rate_cents, status: "available".into() };
    cars.push(car.clone());
    Ok((StatusCode::CREATED, Json(car)))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<u32>, Json(input): Json<UpdateVehicle>) -> Result<Json<Vehicle>, AppError> {
    if !is_valid(&input.make, &input.model, input.daily_rate_cents) {
        return Err(AppError::BadRequest("Invalid vehicle".into()));
    }
    let mut cars = state.vehicles.lock().map_err(|_| AppError::Internal)?;
    let car = cars.iter_mut().find(|car| car.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Vehicle {id} not found")))?;
    car.make = input.make;
    car.model = input.model;
    car.year = input.year;
    car.daily_rate_cents = input.daily_rate_cents;
    Ok(Json(car.clone()))
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<u32>) -> Result<StatusCode, AppError> {
    let mut cars = state.vehicles.lock().map_err(|_| AppError::Internal)?;
    let car = cars.iter().find(|car| car.id == id)
        .ok_or_else(|| AppError::NotFound(format!("Vehicle {id} not found")))?;
    if car.status == "rented" { return Err(AppError::Conflict("Vehicle is currently rented".into())); }
    cars.retain(|car| car.id != id);
    Ok(StatusCode::NO_CONTENT)
}
```

**`src/handlers/customers.rs`** — create this file with the following exact contents:

```rust
use axum::{extract::State, http::StatusCode, Json};
use crate::{domain::customer::{Customer, CreateCustomer}, error::AppError, repository::memory::AppState};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Customer>>, AppError> {
    let customers = state.customers.lock().map_err(|_| AppError::Internal)?;
    Ok(Json(customers.clone()))
}

pub async fn create(State(state): State<AppState>, Json(input): Json<CreateCustomer>) -> Result<(StatusCode, Json<Customer>), AppError> {
    if input.first_name.trim().is_empty() || input.last_name.trim().is_empty() || !input.email.contains('@') {
        return Err(AppError::BadRequest("Invalid customer".into()));
    }
    let mut customers = state.customers.lock().map_err(|_| AppError::Internal)?;
    if customers.iter().any(|customer| customer.email == input.email) {
        return Err(AppError::Conflict("Email already exists".into()));
    }
    let id = customers.iter().map(|customer| customer.id).max().unwrap_or(0) + 1;
    let customer = Customer { id, first_name: input.first_name, last_name: input.last_name, email: input.email };
    customers.push(customer.clone());
    Ok((StatusCode::CREATED, Json(customer)))
}
```

### Step 7: replace `main.rs` with router wiring

`mod` declarations must match module files from the previous steps. The `.with_state(state)` call supplies the exact type extracted by handlers.

**`src/main.rs`** — create this file with the following exact contents:

```rust
mod domain;
mod error;
mod handlers;
mod repository;

use axum::{routing::get, Router};
use repository::memory::AppState;

#[tokio::main]
async fn main() {
    let state = AppState::default();
    let app = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(handlers::vehicles::list).post(handlers::vehicles::create))
        .route("/vehicles/{id}", get(handlers::vehicles::get)
            .put(handlers::vehicles::update).delete(handlers::vehicles::delete))
        .route("/customers", get(handlers::customers::list).post(handlers::customers::create))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Step 8: build and smoke test

```bash
cargo fmt
cargo check
cargo run
```

In a **second** terminal, run these in order:

```bash
curl -i http://127.0.0.1:3000/health
curl -i -X POST http://127.0.0.1:3000/vehicles -H 'content-type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'
curl -s http://127.0.0.1:3000/vehicles | jq
curl -i http://127.0.0.1:3000/vehicles/1
curl -i -X PUT http://127.0.0.1:3000/vehicles/1 -H 'content-type: application/json' \
  -d '{"make":"Toyota","model":"Corolla","year":2025,"daily_rate_cents":5900}'
curl -i -X POST http://127.0.0.1:3000/customers -H 'content-type: application/json' \
  -d '{"first_name":"Alex","last_name":"Smith","email":"alex@example.test"}'
curl -s http://127.0.0.1:3000/customers | jq
curl -i -X DELETE http://127.0.0.1:3000/vehicles/1
curl -i http://127.0.0.1:3000/vehicles/1
```

Expected statuses: health 200, vehicle POST 201, GET 200, PUT 200, customer POST 201, DELETE 204, and final GET 404. Return 409 for duplicate customer email. If you restart the server, lists reset — that is the problem we'll fix tomorrow.

**Checkpoint:** All commands above work and `cargo check` succeeds. Save progress with `git add . && git commit -m 'day 3 modular CRUD'`.

**Optional practice:** Refactor `is_valid` into a `Result` returning a custom enum; add a `GET /vehicles?make=...` query parameter. A repository trait is a useful *optional* design exercise here, not a required missing dependency.

---
# Day 4 — COMPLETE migration to SQLite

**Starting point:** Finished Day 3. Stop the server, commit, and back up the Day 3 version. The full reference files below replace the in-memory code. Rather than leave you with incompatible model types, we make the switch in one explicit step.

**Learning goals:** SQL migrations, row mapping, connection pools, `?` propagation, parameterized queries, SQLite foreign keys.

### Step 1: replace dependencies

**Versioning policy:** The manifest below is a course reference snapshot, not a claim that every listed crate is the newest. Commit `Cargo.lock` for repeatable application builds. Upgrade dependencies deliberately with `cargo update`, then rerun all checks. For an independently selected up-to-date dependency, `cargo add <crate>` resolves versions using the current registry; be aware of API differences from this tutorial.

Use the following full `Cargo.toml`. You will not need any extra dependencies on Days 5–7; this manifest includes test and logging packages up front.

**`Cargo.toml`** — create this file with the following exact contents:

```toml
[package]
name = "rust-rent"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
sqlx = { version = "0.9", default-features = false, features = ["runtime-tokio", "sqlite-bundled", "migrate", "macros", "derive"] }
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
tower-http = { version = "0.7", features = ["trace"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
tempfile = "3"
```

**New in SQLx 0.9:** SQL text parameters are constrained by `SqlSafeStr`. Static SQL string literals work directly. Do not build SQL statements with `format!` and user input. Bind values with `.bind(...)` instead. An explicit escape hatch (`AssertSqlSafe`) exists for reviewed dynamic SQL; it should not be used in this course. The explicit `sqlite-bundled` feature avoids enabling the broader umbrella `sqlite` feature set.

### Step 2: retire Day 3 state

Remove `src/repository/` from the active application. Its `Arc<Mutex<_>>` store has been replaced by a pooled SQLite connection. Remove `mod repository;` from `main.rs` when replacing it below. Run `mkdir -p migrations src/domain src/handlers` to ensure directories exist.

### Step 3: migration file

File names are significant to SQLx migrations. Use the exact path shown below, and do not edit a migration after it has been applied. To reset a disposable development database, delete `rust_rent.db` and restart.

**`migrations/202610260001_create_schema.sql`** — create this file with the following exact contents:

```sql
CREATE TABLE vehicles (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  make TEXT NOT NULL,
  model TEXT NOT NULL,
  year INTEGER NOT NULL CHECK (year BETWEEN 1900 AND 2200),
  daily_rate_cents INTEGER NOT NULL CHECK (daily_rate_cents > 0),
  status TEXT NOT NULL DEFAULT 'available' CHECK (status IN ('available', 'rented', 'maintenance'))
);
CREATE TABLE customers (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  first_name TEXT NOT NULL,
  last_name TEXT NOT NULL,
  email TEXT NOT NULL UNIQUE
);
CREATE TABLE reservations (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  vehicle_id INTEGER NOT NULL REFERENCES vehicles(id) ON DELETE RESTRICT,
  customer_id INTEGER NOT NULL REFERENCES customers(id) ON DELETE RESTRICT,
  start_date TEXT NOT NULL,
  end_date TEXT NOT NULL,
  total_cents INTEGER NOT NULL CHECK (total_cents >= 0),
  status TEXT NOT NULL DEFAULT 'confirmed' CHECK (status IN ('confirmed', 'cancelled')),
  CHECK (end_date > start_date)
);
CREATE INDEX idx_reservations_overlaps ON reservations(vehicle_id, status, start_date, end_date);
```

### Step 4: replace the domain types

The HTTP representation now uses SQLite `i64` IDs and the `FromRow` derive for SQL rows. Reservation types are provided in Day 5; **do not reference that module yet**. For Day 4, `domain/mod.rs` must declare only `vehicle` and `customer` (not `reservation`).

**`src/domain/vehicle.rs`** — create this file with the following exact contents:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub id: i64,
    pub make: String,
    pub model: String,
    pub year: i64,
    pub daily_rate_cents: i64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateVehicle {
    pub make: String,
    pub model: String,
    pub year: i64,
    pub daily_rate_cents: i64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateVehicle {
    pub make: String,
    pub model: String,
    pub year: i64,
    pub daily_rate_cents: i64,
    pub status: String,
}

pub fn valid_vehicle(make: &str, model: &str, year: i64, rate: i64) -> bool {
    !make.trim().is_empty() && !model.trim().is_empty() &&
        (1900..=2200).contains(&year) && rate > 0
}

pub fn valid_status(status: &str) -> bool {
    matches!(status, "available" | "rented" | "maintenance")
}
```

**`src/domain/customer.rs`** — create this file with the following exact contents:

```rust
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Customer {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomer {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
```

**`src/domain/mod.rs` (Day 4 only):**

```rust
pub mod vehicle;
pub mod customer;
```

### Step 5: replace errors and handlers

No function is intentionally omitted in these files. SQL errors are mapped to a safe 500 response; the customer uniqueness violation maps to 409.

**`src/error.rs`** — create this file with the following exact contents:

```rust
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Database(sqlx::Error),
}

#[derive(Serialize)]
struct ErrorEnvelope { error: ErrorBody }
#[derive(Serialize)]
struct ErrorBody { code: &'static str, message: String }

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", msg),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "NOT_FOUND", msg),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "CONFLICT", msg),
            Self::Database(err) => {
                tracing::error!(error = %err, "database operation failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL", "Internal server error".to_string())
            }
        };
        (status, Json(ErrorEnvelope { error: ErrorBody { code, message } })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self { Self::Database(value) }
}
```

**`src/handlers/vehicles.rs`** — create this file with the following exact contents:

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::{domain::vehicle::{CreateVehicle, UpdateVehicle, Vehicle, valid_status, valid_vehicle}, error::AppError, AppState};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, AppError> {
    let rows = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles ORDER BY id")
        .fetch_all(&state.pool).await?;
    Ok(Json(rows))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Vehicle>, AppError> {
    let vehicle = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles WHERE id = ?")
        .bind(id).fetch_optional(&state.pool).await?
        .ok_or_else(|| AppError::NotFound(format!("Vehicle {id} not found")))?;
    Ok(Json(vehicle))
}

pub async fn create(State(state): State<AppState>, Json(input): Json<CreateVehicle>) -> Result<(StatusCode, Json<Vehicle>), AppError> {
    if !valid_vehicle(&input.make, &input.model, input.year, input.daily_rate_cents) {
        return Err(AppError::BadRequest("Invalid vehicle data".into()));
    }
    let result = sqlx::query("INSERT INTO vehicles (make, model, year, daily_rate_cents) VALUES (?, ?, ?, ?)")
        .bind(&input.make).bind(&input.model).bind(input.year).bind(input.daily_rate_cents)
        .execute(&state.pool).await?;
    let vehicle = sqlx::query_as::<_, Vehicle>("SELECT * FROM vehicles WHERE id = ?")
        .bind(result.last_insert_rowid()).fetch_one(&state.pool).await?;
    Ok((StatusCode::CREATED, Json(vehicle)))
}

pub async fn update(State(state): State<AppState>, Path(id): Path<i64>, Json(input): Json<UpdateVehicle>) -> Result<Json<Vehicle>, AppError> {
    if !valid_vehicle(&input.make, &input.model, input.year, input.daily_rate_cents) || !valid_status(&input.status) {
        return Err(AppError::BadRequest("Invalid vehicle data".into()));
    }
    // Rental status is derived from reservations; users may not set it directly.
    if input.status == "rented" {
        return Err(AppError::BadRequest("Rented status is managed by reservations".into()));
    }
    let result = sqlx::query("UPDATE vehicles SET make=?, model=?, year=?, daily_rate_cents=?, status=? WHERE id=?")
        .bind(&input.make).bind(&input.model).bind(input.year).bind(input.daily_rate_cents)
        .bind(&input.status).bind(id).execute(&state.pool).await?;
    if result.rows_affected() == 0 { return Err(AppError::NotFound(format!("Vehicle {id} not found"))); }
    get(State(state), Path(id)).await
}

pub async fn delete(State(state): State<AppState>, Path(id): Path<i64>) -> Result<StatusCode, AppError> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM reservations WHERE vehicle_id=?")
        .bind(id).fetch_one(&state.pool).await?;
    if count.0 > 0 { return Err(AppError::Conflict("Cannot delete vehicle with reservation history".into())); }
    let result = sqlx::query("DELETE FROM vehicles WHERE id=?").bind(id).execute(&state.pool).await?;
    if result.rows_affected() == 0 { return Err(AppError::NotFound(format!("Vehicle {id} not found"))); }
    Ok(StatusCode::NO_CONTENT)
}
```

**`src/handlers/customers.rs`** — create this file with the following exact contents:

```rust
use axum::{extract::State, http::StatusCode, Json};
use crate::{domain::customer::{Customer, CreateCustomer}, error::AppError, AppState};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Customer>>, AppError> {
    Ok(Json(sqlx::query_as::<_, Customer>("SELECT * FROM customers ORDER BY id").fetch_all(&state.pool).await?))
}

pub async fn create(State(state): State<AppState>, Json(input): Json<CreateCustomer>) -> Result<(StatusCode, Json<Customer>), AppError> {
    if input.first_name.trim().is_empty() || input.last_name.trim().is_empty() || !input.email.contains('@') {
        return Err(AppError::BadRequest("Provide first_name, last_name and a valid-looking email".into()));
    }
    let result = sqlx::query("INSERT INTO customers (first_name, last_name, email) VALUES (?, ?, ?)")
        .bind(&input.first_name).bind(&input.last_name).bind(&input.email).execute(&state.pool).await;
    let result = match result {
        Ok(result) => result,
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => return Err(AppError::Conflict("Email already registered".into())),
        Err(err) => return Err(err.into()),
    };
    let customer = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id=?")
        .bind(result.last_insert_rowid()).fetch_one(&state.pool).await?;
    Ok((StatusCode::CREATED, Json(customer)))
}
```

**`src/handlers/mod.rs` (Day 4 only):**

```rust
pub mod vehicles;
pub mod customers;
```

### Step 6: database pool, router and startup

**Day 4 uses these temporary complete versions** instead of the final Day 5–7 `lib.rs` and `main.rs` (which contain routes not yet built). Create `src/lib.rs`:

```rust
pub mod domain;
pub mod error;
pub mod handlers;

use axum::{routing::get, Router};
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState { pub pool: SqlitePool }

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(handlers::vehicles::list).post(handlers::vehicles::create))
        .route("/vehicles/{id}", get(handlers::vehicles::get)
            .put(handlers::vehicles::update).delete(handlers::vehicles::delete))
        .route("/customers", get(handlers::customers::list).post(handlers::customers::create))
        .with_state(state)
}
```

Replace `src/main.rs`:

```rust
use rust_rent::{build_router, AppState};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::{str::FromStr, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str("sqlite://rust_rent.db")?
        .create_if_missing(true)
        .foreign_keys(true)
        .busy_timeout(Duration::from_secs(5));
    let pool = SqlitePoolOptions::new().max_connections(5)
        .connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let app = build_router(AppState { pool });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

The binary now imports the library crate (`rust_rent`) instead of duplicating all module declarations. The library target is picked up automatically from `src/lib.rs`.

### Step 7: confirm persistence

```bash
cargo fmt
cargo check
cargo run
```

POST a vehicle/customer using Day 3 requests. Stop and restart the process, then GET them again. They should still exist. The SQLite file should be in the repository root. To check manually:

```bash
sqlite3 rust_rent.db 'SELECT id, make, model FROM vehicles;'
sqlite3 rust_rent.db 'SELECT id, email FROM customers;'
```

**Checkpoint:** Data persists and `cargo check` succeeds. Commit Day 4. Do not copy Day 5's router yet.

---
# Day 5 — COMPLETE reservations implementation

**Starting point:** Day 4 persisted CRUD works. We now activate the table that the migration already created.

**Learning goals:** strong date types, half-open intervals, checked math, conflict checking and SQLite transaction boundaries.

### Step 1: create reservation domain model

**`src/domain/reservation.rs`** — create this file with the following exact contents:

```rust
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, FromRow)]
pub struct Reservation {
    pub id: i64,
    pub vehicle_id: i64,
    pub customer_id: i64,
    pub start_date: String,
    pub end_date: String,
    pub total_cents: i64,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateReservation {
    pub vehicle_id: i64,
    pub customer_id: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

pub fn calculate_total(rate: i64, start: NaiveDate, end: NaiveDate) -> Result<i64, &'static str> {
    let days = (end - start).num_days();
    if days <= 0 { return Err("end date must follow start date"); }
    if rate <= 0 { return Err("daily rate must be positive"); }
    rate.checked_mul(days).ok_or("price overflow")
}

pub fn ranges_overlap(a_start: NaiveDate, a_end: NaiveDate, b_start: NaiveDate, b_end: NaiveDate) -> bool {
    a_start < b_end && a_end > b_start
}

#[cfg(test)]
mod tests {
    use super::*;
    fn d(day: u32) -> NaiveDate { NaiveDate::from_ymd_opt(2026, 11, day).unwrap() }
    #[test]
    fn three_days_cost_195_dollars() { assert_eq!(calculate_total(6500, d(1), d(4)), Ok(19500)); }
    #[test]
    fn invalid_dates_rejected() { assert!(calculate_total(6500, d(4), d(1)).is_err()); }
    #[test]
    fn adjacent_ranges_are_allowed() { assert!(!ranges_overlap(d(1), d(4), d(4), d(7))); }
    #[test]
    fn crossing_ranges_overlap() { assert!(ranges_overlap(d(1), d(4), d(3), d(5))); }
}
```

### Step 2: export the new domain and handler modules

Replace `src/domain/mod.rs` and `src/handlers/mod.rs` with these final versions:

**`src/domain/mod.rs`** — create this file with the following exact contents:

```rust
pub mod reservation;
pub mod vehicle;
pub mod customer;
```

**`src/handlers/mod.rs`** — create this file with the following exact contents:

```rust
pub mod vehicles;
pub mod customers;
pub mod reservations;
```

### Step 3: create reservation HTTP handlers and database operations

The reservation uses SQLx `Pool::begin_with("BEGIN IMMEDIATE")` to acquire a SQLite write transaction before the overlap check. SQLx schedules rollback when an unfinished transaction is dropped, covering early `?` returns. Edge cases involving cancellation and disk-full errors are tracked upstream and are outside this introductory course. SQLite permits only one writer at a time. For PostgreSQL use its own constraints and transactional locking semantics. The source below is complete, including the helper function.

**`src/handlers/reservations.rs`** — create this file with the following exact contents:

```rust
use axum::{extract::{Path, State}, http::StatusCode, Json};
use crate::{domain::reservation::{CreateReservation, Reservation, calculate_total}, error::AppError, AppState};

pub async fn list(State(state): State<AppState>) -> Result<Json<Vec<Reservation>>, AppError> {
    Ok(Json(sqlx::query_as::<_, Reservation>("SELECT * FROM reservations ORDER BY id")
        .fetch_all(&state.pool).await?))
}

pub async fn get(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Reservation>, AppError> {
    let item = sqlx::query_as::<_, Reservation>("SELECT * FROM reservations WHERE id=?")
        .bind(id).fetch_optional(&state.pool).await?
        .ok_or_else(|| AppError::NotFound(format!("Reservation {id} not found")))?;
    Ok(Json(item))
}

pub async fn create(State(state): State<AppState>, Json(input): Json<CreateReservation>) -> Result<(StatusCode, Json<Reservation>), AppError> {
    if input.end_date <= input.start_date {
        return Err(AppError::BadRequest("end_date must be after start_date".into()));
    }
    // Acquire a SQLite write lock before checking for overlapping bookings.
    // Dropping an unfinished SQLx transaction schedules a rollback.
    let mut tx = state.pool.begin_with("BEGIN IMMEDIATE").await?;
    let reservation = create_under_lock(&mut tx, &input).await?;
    tx.commit().await?;
    Ok((StatusCode::CREATED, Json(reservation)))
}

async fn create_under_lock(
    tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
    input: &CreateReservation,
) -> Result<Reservation, AppError> {
    let vehicle: Option<(i64, String)> = sqlx::query_as(
        "SELECT daily_rate_cents, status FROM vehicles WHERE id = ?"
    )
    .bind(input.vehicle_id)
    .fetch_optional(&mut **tx)
    .await?;
    let (rate, status) = vehicle
        .ok_or_else(|| AppError::NotFound("Vehicle not found".into()))?;
    if status == "maintenance" {
        return Err(AppError::Conflict("Vehicle under maintenance".into()));
    }
    let customer: Option<(i64,)> = sqlx::query_as(
        "SELECT id FROM customers WHERE id = ?"
    )
    .bind(input.customer_id)
    .fetch_optional(&mut **tx)
    .await?;
    if customer.is_none() {
        return Err(AppError::NotFound("Customer not found".into()));
    }
    let overlap_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM reservations \
         WHERE vehicle_id = ? AND status = 'confirmed' \
         AND start_date < ? AND end_date > ?"
    )
    .bind(input.vehicle_id)
    .bind(input.end_date.to_string())
    .bind(input.start_date.to_string())
    .fetch_one(&mut **tx)
    .await?;
    if overlap_count.0 != 0 {
        return Err(AppError::Conflict("Vehicle already reserved for these dates".into()));
    }
    let total = calculate_total(rate, input.start_date, input.end_date)
        .map_err(|message| AppError::BadRequest(message.into()))?;
    let inserted = sqlx::query(
        "INSERT INTO reservations \
         (vehicle_id, customer_id, start_date, end_date, total_cents) \
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(input.vehicle_id)
    .bind(input.customer_id)
    .bind(input.start_date.to_string())
    .bind(input.end_date.to_string())
    .bind(total)
    .execute(&mut **tx)
    .await?;
    let reservation = sqlx::query_as::<_, Reservation>(
        "SELECT * FROM reservations WHERE id = ?"
    )
    .bind(inserted.last_insert_rowid())
    .fetch_one(&mut **tx)
    .await?;
    Ok(reservation)
}

pub async fn cancel(State(state): State<AppState>, Path(id): Path<i64>) -> Result<Json<Reservation>, AppError> {
    let result = sqlx::query("UPDATE reservations SET status='cancelled' WHERE id=?")
        .bind(id).execute(&state.pool).await?;
    if result.rows_affected() == 0 { return Err(AppError::NotFound(format!("Reservation {id} not found"))); }
    get(State(state), Path(id)).await
}
```

### Step 4: update router to expose reservations

Replace `src/lib.rs` with this complete version. It adds all three reservation routes to the Day 4 router.

**`src/lib.rs`** — create this file with the following exact contents:

```rust
pub mod domain;
pub mod error;
pub mod handlers;

use axum::{routing::{get, post}, Router};
use sqlx::SqlitePool;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct AppState { pub pool: SqlitePool }

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(handlers::vehicles::list).post(handlers::vehicles::create))
        .route("/vehicles/{id}", get(handlers::vehicles::get)
            .put(handlers::vehicles::update).delete(handlers::vehicles::delete))
        .route("/customers", get(handlers::customers::list).post(handlers::customers::create))
        .route("/reservations", get(handlers::reservations::list).post(handlers::reservations::create))
        .route("/reservations/{id}", get(handlers::reservations::get))
        .route("/reservations/{id}/cancel", post(handlers::reservations::cancel))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

### Step 5: exercise the API

Start `cargo run` and use a fresh database or note the existing IDs returned by your API. The following example assumes vehicle ID 1 and customer ID 1 exist:

```bash
curl -i -X POST localhost:3000/reservations -H 'content-type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-01","end_date":"2026-11-04"}'
curl -i -X POST localhost:3000/reservations -H 'content-type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-03","end_date":"2026-11-06"}'
curl -i -X POST localhost:3000/reservations -H 'content-type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-04","end_date":"2026-11-07"}'
curl -s localhost:3000/reservations | jq
curl -i -X POST localhost:3000/reservations/1/cancel
curl -i localhost:3000/reservations/1
```

Expected booking statuses: **201**, **409**, **201**. Cancellation returns **200** with status `cancelled`. If you cancel the first rental, those dates become available for a future booking. Duplicate cancellation is deliberately idempotent (returns 200).

### Understanding why the logic works

Intervals `[Nov 1, Nov 4)` and `[Nov 4, Nov 7)` do not overlap. The SQL conflict predicate is `existing.start_date < requested.end_date AND existing.end_date > requested.start_date`, restricted to confirmed reservations. `BEGIN IMMEDIATE` acquires SQLite's reserved write lock before checking overlaps; a competing SQLite writer must wait or fail rather than interleave its check/insert.

**Checkpoint:** Confirm 201 / 409 / 201 and run `cargo check`.

**Caveats:** Dates are simplified whole rental days, not timestamped pickups; a real rental platform must address time zones, maintenance schedules, payment holds, fleet transfers, vehicle status rules and authorization. The example treats an explicit `rented` operational status as legacy and only blocks `maintenance` on new bookings. Do not infer calendar availability from `vehicles.status` alone.

---
# Day 6 — COMPLETE automated tests

**Starting point:** Day 5 service works. Day 4 already introduced `src/lib.rs`, making handlers importable from integration tests.

### Step 1: domain unit tests

`src/domain/reservation.rs` **already contains** four `#[test]` functions under `#[cfg(test)]`. Inspect them, then run:

```bash
cargo test --lib
```

Note that the `#[cfg(test)]` block is excluded from release builds and only compiled for tests.

### Step 2: API integration tests

Create `tests/api.rs` exactly as follows. The test makes a fresh SQLite database inside a temporary directory, applies migrations, constructs the application router, and sends HTTP requests *without binding a TCP port*. That makes the test repeatable and avoids port conflicts.

**`tests/api.rs`** — create this file with the following exact contents:

```rust
use axum::{body::Body, http::{Request, StatusCode}};
use http_body_util::BodyExt;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, SqlitePool};
use std::{str::FromStr, time::Duration};
use tower::ServiceExt;

// Binary crates are not importable from integration tests. Day 4 created src/lib.rs; the integration tests import that library.
use rust_rent::{build_router, AppState};

async fn setup() -> (tempfile::TempDir, SqlitePool) {
    let dir = tempfile::tempdir().unwrap();
    let url = format!("sqlite://{}", dir.path().join("test.db").display());
    let options = SqliteConnectOptions::from_str(&url).unwrap()
        .create_if_missing(true).foreign_keys(true).busy_timeout(Duration::from_secs(5));
    let pool = SqlitePoolOptions::new().max_connections(3).connect_with(options).await.unwrap();
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();
    (dir, pool)
}

async fn post_json(app: axum::Router, path: &str, json: &str) -> (StatusCode, serde_json::Value) {
    let response = app.oneshot(Request::builder().method("POST").uri(path)
        .header("content-type", "application/json").body(Body::from(json.to_owned())).unwrap())
        .await.unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice(&bytes).unwrap();
    (status, value)
}

#[tokio::test]
async fn rejects_overlapping_reservation() {
    let (_dir, pool) = setup().await;
    let app = build_router(AppState { pool });
    let (status, _) = post_json(app.clone(), "/vehicles", r#"{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}"#).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, _) = post_json(app.clone(), "/customers", r#"{"first_name":"Alex","last_name":"Smith","email":"alex@example.test"}"#).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, _) = post_json(app.clone(), "/reservations", r#"{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-01","end_date":"2026-11-04"}"#).await;
    assert_eq!(status, StatusCode::CREATED);
    let (status, _) = post_json(app.clone(), "/reservations", r#"{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-03","end_date":"2026-11-06"}"#).await;
    assert_eq!(status, StatusCode::CONFLICT);
    let (status, _) = post_json(app, "/reservations", r#"{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-04","end_date":"2026-11-06"}"#).await;
    assert_eq!(status, StatusCode::CREATED);
}
```

### Step 3: execute and extend

```bash
cargo fmt
cargo test
cargo test rejects_overlapping_reservation -- --nocapture
cargo clippy --all-targets -- -D warnings
```

Expected: the overlap test creates a vehicle and customer, accepts one booking, rejects overlapping dates with 409, then accepts an adjacent booking with 201. **If Clippy finds a warning, correct it before advancing.**

Required follow-on exercises for learning (not required to compile): add tests for invalid dates (400), nonexistent vehicle (404), missing customer (404), price overflow (400), cancellation idempotence (200), and deleting a vehicle with reservation history (409). Each test must create a fresh database, not share IDs with other test cases.

**Checkpoint:** `cargo test` and `cargo clippy --all-targets -- -D warnings` pass.

---
# Day 7 — COMPLETE configuration, tracing, release and docs

**Starting point:** Day 6 tests pass. The full dependency manifest from Day 4 already includes `tracing`, `tracing-subscriber`, and `tower-http`. Initialize the tracing subscriber on Day 4 rather than delaying logs until today.

### Step 1: replace `src/main.rs` with configurable startup

`DATABASE_URL` defaults to a SQLite file in your working directory, `LISTEN_ADDR` defaults to local loopback, and `RUST_LOG` controls tracing. Note the `.foreign_keys(true)` setting: SQLite foreign key checking must be enabled for connections.

**`src/main.rs`** — create this file with the following exact contents:

```rust
use rust_rent::{build_router, AppState};
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}};
use std::{env, str::FromStr};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "rust_rent=info,tower_http=info".into()))
        .init();
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://rust_rent.db".into());
    let address = env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into());
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true).foreign_keys(true).busy_timeout(std::time::Duration::from_secs(5));
    let pool = SqlitePoolOptions::new().max_connections(5)
        .connect_with(options).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    let listener = tokio::net::TcpListener::bind(&address).await?;
    tracing::info!(%address, "RustRent listening");
    axum::serve(listener, build_router(AppState { pool })).await?;
    Ok(())
}
```

### Step 2: verify tracing and config

```bash
RUST_LOG=rust_rent=debug,tower_http=debug cargo run
curl -i http://127.0.0.1:3000/health
```

If port 3000 is taken, use `LISTEN_ADDR=127.0.0.1:3001` and curl port 3001. Avoid changing the database URL halfway through a learning checkpoint unless you intend to create a different SQLite file.

### Step 3: update the existing `.gitignore`

**`.gitignore`** — Cargo normally creates this file already. Merge the following entries without discarding any current patterns:

```gitignore
/target
*.db
*.db-shm
*.db-wal
.env
```

### Step 4: document the API

Copy the **complete** `docs/api.md` shipped with the final reference ZIP. The table below summarizes its contract:

| Method | Path | Expected success |
|---|---|---|
| GET | `/health` | 200 text `OK` |
| GET | `/vehicles` | 200 JSON array |
| GET | `/vehicles/{id}` | 200 JSON vehicle |
| POST | `/vehicles` | 201 JSON vehicle |
| PUT | `/vehicles/{id}` | 200 JSON vehicle |
| DELETE | `/vehicles/{id}` | 204 empty body |
| GET | `/customers` | 200 JSON array |
| POST | `/customers` | 201 JSON customer |
| GET | `/reservations` | 200 JSON array |
| GET | `/reservations/{id}` | 200 JSON reservation |
| POST | `/reservations` | 201 JSON reservation |
| POST | `/reservations/{id}/cancel` | 200 JSON reservation |

Error responses have the form `{"error":{"code":"CONFLICT","message":"..."}}`; validation/lookup/unique constraint statuses are 400/404/409 respectively. Axum's default extractor rejection for malformed JSON is separate and may have a text/plain body; standardizing *all* extractor failures is a follow-up hardening exercise.

### Step 5: run the automated smoke test

The final reference ZIP includes `scripts/smoke.sh`. Run `bash scripts/smoke.sh` against a running **Day 5 or later** server. The script creates its own unique customer and vehicle and checks a successful booking, a conflicting booking, an adjacent booking, and a 404. Requires `curl` and `jq`.

### Step 6: final quality and packaging

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo build --release
DATABASE_URL=sqlite://rust_rent.db ./target/release/rust-rent
```

Open another terminal and verify `/health`. The binary's working directory still matters for the SQLite relative path and migration behavior, so launch from the project root during the course.

**Checkpoint:** All four build/quality commands complete and the release server starts. Commit with `git add . && git commit -m 'day 7 release ready'`.

---
# Troubleshooting matrix

| Symptom | Common cause | Fix |
|---|---|---|
| `unresolved import crate::...` | `mod` or `pub mod` missing | Check exact `src/lib.rs`, `src/domain/mod.rs`, `src/handlers/mod.rs` contents for the day |
| `cannot find ...` on Day 3 | Skipped file creation | Revisit Day 3 Steps 1–7; do not use future-day files prematurely |
| `mismatched types u32/i64` | Mixed Day 3 and Day 4 types | Replace whole Day 4 files; SQLx models use `i64` IDs |
| `unable to open database file` | Bad working directory or DB URL | Run from root; use `sqlite://rust_rent.db` and `create_if_missing(true)` |
| `no such table` | Migration did not run | Check `migrations/` and startup `sqlx::migrate!` |
| `address already in use` | Previous server still running | Stop old process or change `LISTEN_ADDR` on Day 7 |
| `error returned 500` | Unhandled SQL error | Inspect tracing logs and database constraints |
| `database is locked` | SQLite write contention | Verify busy timeout and transaction scope; retries are out of scope |
| `duplicate identifier` | Data persisted from earlier run | Use IDs returned by POST or reset only your disposable local `.db` |
| `cargo test` cannot import app | Missing library target | Create `src/lib.rs` as shown on Day 4 |

# Final review checklist

- [ ] Each file referenced in an import is created in an earlier step or in the same step.
- [ ] Every `mod` has a matching Rust module file.
- [ ] All routes point to defined handlers.
- [ ] Day 3 works without any database or reservation module.
- [ ] Day 4 works without reservation handlers.
- [ ] Day 5 activates reservations and the new router.
- [ ] Day 6 uses a real independent test database and library module.
- [ ] Day 7 provides startup configuration, tracing, docs and release checks.

# References

- [Official Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Axum documentation](https://docs.rs/axum/latest/axum/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [SQLx 0.9 documentation](https://docs.rs/sqlx/0.9.0/sqlx/)
- [Serde](https://serde.rs/)

**Review status:** Peer review identified multiple defects, and targeted corrections were made; no claim of compiler-verified correctness is made. The environment used to prepare this document does not have the Rust toolchain installed; therefore `cargo check`, Clippy and `cargo test` have NOT been run here; treat the supplied checkpoint commands as essential verification on your Arch system. This course teaches a functional local reference implementation, not a security-hardened rental platform.



# Additional explanations and limits (peer-review notes)

- **`?` and `From`:** `?` returns early when the expression is `Err`. If the surrounding function has a different error type, Rust uses an available `From` implementation to convert it. Here `From<sqlx::Error> for AppError` lets SQLx failures propagate as application errors.
- **Derive macros:** `#[derive(Serialize)]` supplies JSON serialization through Serde. `#[derive(sqlx::FromRow)]` maps database column values into a Rust struct.
- **Shared state:** Cloning an `Arc` or SQLx pool clones a lightweight handle; it does not duplicate every vehicle or establish an independent pool.
- **Closures:** `ok_or_else(|| AppError::NotFound(...))` creates an error only when an `Option` is `None`; `map_err` converts an error without changing a successful result.
- **HTTP errors:** `IntoResponse` describes how domain errors become HTTP statuses and JSON response bodies. `Json` should be last among Axum body-consuming extractors.
- **Concurrency:** `std::sync::MutexGuard` must not remain held across `.await` in handlers; it can block unrelated executor tasks and cause Send-related compilation errors.
- **Request errors:** Valid JSON containing a reversed date range returns 400 with the domain error envelope. Malformed dates or JSON are extractor errors and can return 422 or 415 in plaintext.
- **Overflow testing:** To exercise `checked_mul`, create a vehicle with an extremely large positive daily rate, then request multiple rental days.
- **SQLite limitation:** `BEGIN IMMEDIATE` serializes writers in a single SQLite database; a distributed server with PostgreSQL will need its own conflict prevention strategy.

This course remains a Rust **REST API project** rather than comprehensive language coverage. The source code ZIP is a reference, not a substitute for understanding compiler diagnostics.


---

## Peer-review round 2: verification status (26 October 2026)

The source and tutorial have been updated for `sqlx = 0.9` with `sqlite-bundled`, `macros`, `migrate`, and `derive`; `tower-http = 0.7`; `Pool::begin_with("BEGIN IMMEDIATE")`; and the SQLx 0.9 Rust 1.94.0 MSRV. These changes have **not** been checked by `cargo check`, `cargo test`, or `cargo clippy` in this environment because no Rust toolchain is installed. They are reviewed against published API and feature documentation, not compile-verified.

On your Arch machine, after copying the source code for each day's checkpoint, execute:

```bash
rustup update stable
rustc --version
cargo check --all-targets
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo tree --depth 1
```

Capture the exact versions and outputs in your project notes. `cargo tree -i libsqlite3-sys` can reveal the selected SQLite binding package. If any check fails, do not assume that later days will build.

### Still-open teaching improvements

The inline explanations, exercise answer keys, and complete Day 3–5 format cleanup are not yet comprehensive. The guide is therefore still a work-in-progress learning resource, not a fully validated release.
