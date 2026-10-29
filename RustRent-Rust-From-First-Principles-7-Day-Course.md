# RustRent: Learn Rust From First Principles in Seven Days

**A beginner-first, explanation-led course culminating in a rental-car REST API**  
**Revision:** 27 October 2026 · **Edition:** Rust 2024 · **Status:** educational draft; compiler execution not verified in this environment

> **Start here.** This is a **new curriculum**, not a patch to the old RustRent guide. We will retain `Vehicle` as a domain model with methods and evolve it. We do not introduce a web framework until you understand the language underneath it.

## Who this is for

You need **no prior programming experience**. If you have programmed before, you can move faster through the syntax exercises, but **do not skip ownership and borrowing**. The promise of "seven days" describes seven **learning modules**, not seven fixed-length sessions. Expect **2–4 hours per module**, sometimes more. There is no deadline. If a checkpoint is unclear, repeat it before continuing.

## What you will build

A rental-car service called **RustRent**. You'll start with a command-line program that knows about vehicles, customers and bookings. You will preserve those types as you gradually introduce HTTP and persistence. The resulting local API will support listing and creating vehicles and customers, and reserving vehicles for non-overlapping date ranges. **This is an educational local API, not a public production system.** Authentication, payment, deployment hardening and distributed booking coordination are outside this beginner course.

## How this course teaches

Every module uses the same loop:

1. **Mental model:** explain the idea and what problem it solves.
2. **Small isolated experiment:** make the compiler show the rule.
3. **Application:** evolve RustRent without discarding good earlier designs.
4. **Checkpoint:** run actual commands and compare expected behavior.
5. **Explain it back:** answer questions without reading the code.
6. **Exercises and hints:** attempt them before looking at model answers.

Code is identified as **FULL FILE** (copyable), **ADD** (paste into an existing file), **ILLUSTRATION** (not a complete program), or **EXPECTED COMPILER ERROR** (intentionally invalid). When adding files, paths are relative to your project root.

## Learning roadmap

| Day | Primary lesson | RustRent feature | Must explain before proceeding |
|---|---|---|---|
| 1 | Types, variables, `struct`, `enum`, `impl`, `Result` | Vehicle model | Why `&self` differs from `&mut self` |
| 2 | Ownership, borrowing, slices, collections, modules | Vehicle fleet | Why moving isn't cloning |
| 3 | `Option`, errors, traits, iterators, tests | Customers and business rules | Why `Result<T,E>` differs from `Option<T>` |
| 4 | Async, `Send`/`Sync`, HTTP, Serde, Axum | First JSON API | Why handlers should be thin |
| 5 | Modules, API contracts, shared state, CRUD | Vehicles and customers API | Why borrowing through a lock is scoped |
| 6 | SQL, persistence, conversion, SQLx | Database-backed API | Why database row types aren't necessarily domain types |
| 7 | Date intervals, transactions, integration tests | Reservation API | Why check-then-insert must be atomic |

## Tools and prerequisites

Arch Linux example:

```bash
sudo pacman -S --needed rustup base-devel sqlite curl jq
rustup default stable
rustup update stable
rustup component add rustfmt clippy
rustc --version
cargo --version
cargo new rust-rent
cd rust-rent
cargo run
```

The course uses Rust **edition 2024**. Confirm `edition = "2024"` in Cargo.toml. `cargo new` normally creates `.git` and `.gitignore`; do not overwrite `.gitignore` carelessly. **Commit `Cargo.lock`** for this application. Once an HTTP framework is added, lockfile compatibility matters.

Recommended habits: use `cargo fmt`, `cargo check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`. Do not run `clippy -D warnings` while deliberately leaving unused exercise functions in a snapshot; complete or remove them first.

**Compiler status:** I can review the design and check official API documentation here, but I cannot execute `cargo` in this runtime. The course does **not** claim that its cumulative snapshots have been built. The commands at each checkpoint are your source of truth; do not mistake documentation review for compilation.

## Five Rust principles to keep in view

1. **Ownership is a resource rule:** a value has an owner and moves by default for non-`Copy` types.
2. **Borrowing separates access from ownership:** `&T` reads; `&mut T` provides exclusive mutable access.
3. **Prefer clear concrete types:** a `struct` plus its `impl` is idiomatic; use a trait when abstraction pays for itself.
4. **Make invalid states hard to represent:** constructors and private fields guard invariants.
5. **Separate what the app means from how it is delivered:** domain rules need not know HTTP or SQLite.

---

# Day 1 — Learn Rust's building blocks

**Outcome:** You can build a small command-line vehicle model, inspect values and handle recoverable failures.

## 1. Variables and types, before the application

Put this in `src/main.rs` as a **FULL FILE**, then `cargo run`:

```rust
fn main() {
    let make = "Toyota";           // &str: a borrowed string slice
    let model = String::from("Camry"); // String: owned, growable text
    let year: u16 = 2025;           // Unsigned 16-bit integer
    let daily_rate_cents: i64 = 6500;
    let available = true;

    println!("{year} {make} {model}");
    println!("Rate: {daily_rate_cents} cents");
    println!("Available? {available}");
}
```

**Why the types?** A `u16` can express ordinary model years without negative numbers; `i64` is deliberately large enough for integer money. Money is stored in **cents**, not floating-point dollars, because financial arithmetic should not depend on binary floating-point rounding. `String` owns its bytes; `&str` borrows text stored elsewhere. `println!` ends with `!` because it is a **macro**, not a regular function.

Experiment: replace `let available = true` with `let mut available = true; available = false;` and observe what `mut` means. Then remove `mut` and confirm the compiler refuses the reassignment. Rust variables are immutable by default **to make changes explicit**.

## 2. From five unrelated variables to one meaningful type

A struct groups related data. An enum enumerates allowed choices. An `impl` block attaches operations to a type. Use the following **FULL FILE**:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum VehicleStatus {
    Ready,
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

#[derive(Debug, PartialEq, Eq)]
enum VehicleError {
    EmptyMake,
    EmptyModel,
    InvalidRate,
    UnderMaintenance,
}

impl Vehicle {
    fn new(
        id: u32,
        make: String,
        model: String,
        year: u16,
        daily_rate_cents: i64,
    ) -> Result<Self, VehicleError> {
        if make.trim().is_empty() {
            return Err(VehicleError::EmptyMake);
        }
        if model.trim().is_empty() {
            return Err(VehicleError::EmptyModel);
        }
        if daily_rate_cents <= 0 {
            return Err(VehicleError::InvalidRate);
        }

        Ok(Self {
            id,
            make,
            model,
            year,
            daily_rate_cents,
            status: VehicleStatus::Ready,
        })
    }

    fn is_rentable(&self) -> bool {
        self.status == VehicleStatus::Ready
    }

    fn put_into_maintenance(&mut self) {
        self.status = VehicleStatus::Maintenance;
    }

    fn display(&self) {
        println!(
            "#{}: {} {} ({}), {} cents/day, {:?}",
            self.id, self.make, self.model, self.year,
            self.daily_rate_cents, self.status
        );
    }
}

fn main() {
    let mut car = match Vehicle::new(
        1,
        String::from("Toyota"),
        String::from("Camry"),
        2025,
        6500,
    ) {
        Ok(car) => car,
        Err(error) => {
            eprintln!("Could not create vehicle: {error:?}");
            return;
        }
    };

    car.display();
    println!("Rentable: {}", car.is_rentable());
    car.put_into_maintenance();
    car.display();
}
```

### Why these choices?

- `#[derive(Debug)]` asks the compiler to generate `Debug` formatting (used by `{:?}`). `Clone` generates an explicit copy-like operation for owned values; `Copy` is available to small value types like this fieldless enum, not automatically to a `String`-holding `Vehicle`.
- `Result<Self, VehicleError>` makes construction *fallible*. `Ok(vehicle)` means success; `Err(reason)` means failure. The type tells callers they must handle both possibilities.
- `Vehicle::new` is an **associated function**; it doesn't take `self` yet. `is_rentable(&self)` borrows an existing vehicle; `put_into_maintenance(&mut self)` borrows it for mutation.
- `self.status` lives with the vehicle because its rules belong to the vehicle. **This domain `impl` survives all seven days.**
- We do **not** add `Rented` to physical status. A car can be ready for use and reserved next weekend; the reservation calendar, not a permanent boolean, will determine date-specific availability on Day 7.

### Deliberately break it

**EXPECTED COMPILER ERROR:** Replace `let mut car` with `let car` and run `cargo check`. The compiler should report that you cannot borrow an immutable binding as mutable when calling `put_into_maintenance`. Restore `mut` afterward.

### Checkpoint

Run `cargo fmt && cargo run && cargo test`. Predict which operations modify a vehicle before executing them.

### Exercises

1. Add a `mark_ready(&mut self)` method and call it after maintenance.
2. Make a second vehicle with a bad rate. Print the error instead of panicking.
3. Explain the difference between `String` and `&str`, and why `new` takes ownership of `String` values.

**Hints:** 1. Assign `VehicleStatus::Ready`. 2. Use `match Vehicle::new(...)`. 3. A `String` is owned; `&str` is a view.

**Worked answers (read only after trying):**

```rust
// ADD inside impl Vehicle
fn mark_ready(&mut self) {
    self.status = VehicleStatus::Ready;
}

// ADD inside main
match Vehicle::new(2, "Ford".into(), "Focus".into(), 2020, -10) {
    Ok(_) => println!("Unexpected success"),
    Err(err) => println!("Correctly rejected: {err:?}"),
}
```

**Explain it back:** Why is `&mut self` required to change status? Why would a return type of `Vehicle` hide an important constructor failure? Why is status an enum instead of a free-form string?

---

# Day 2 — Ownership, borrowing, collections and modules

**Outcome:** Multiple vehicles live in a fleet; you can iterate without transferring ownership and can organize source files across modules.

## 1. Predict the compiler

**EXPECTED COMPILER ERROR** (run in a temporary scratch project, not the growing RustRent `main.rs`):

```rust
fn main() {
    let name = String::from("Camry");
    let other = name;
    println!("{name}"); // moved value: cannot use it here
    println!("{other}");
}
```

Moving transfers ownership; it does not duplicate memory. If both names independently owned the same allocation, freeing both could be unsafe. You have **three distinct options**:

```rust
let a = String::from("Camry");
let b = a.clone(); // duplicate contents (allocation)
println!("{a} {b}");

let a = String::from("Camry");
let borrowed = &a; // no transfer, no allocation
println!("{a} {borrowed}");

let a = String::from("Camry");
let b = a; // move: only b is used afterward
println!("{b}");
```

Use moves when handing off ownership, borrows for observation, and clones when *independent ownership* is actually needed. `clone()` is not taboo, but unexplained cloning masks design decisions.

### The borrowing rule

You may have multiple concurrent `&T` immutable borrows, or one active `&mut T` mutable borrow, but not conflicting borrows at the same time. These rules help prevent data races. **Lifetimes** describe how long a reference must remain valid relative to borrowed data. Most lifetimes here are inferred; explicit annotations arrive when you write functions returning borrowed data.

## 2. Introduce slices

A `Vec<Vehicle>` owns elements. A `&[Vehicle]` borrows a consecutive view of some elements; it can accept vectors and arrays. Functions that only *read* a collection should generally prefer slices to `&Vec<T>`.

Create `src/fleet.rs` (**FULL FILE**) after moving the definitions from Day 1 into `src/vehicle.rs`:

```rust
use crate::vehicle::Vehicle;

#[derive(Debug, Default)]
pub struct Fleet {
    vehicles: Vec<Vehicle>,
}

impl Fleet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, vehicle: Vehicle) {
        self.vehicles.push(vehicle);
    }

    pub fn all(&self) -> &[Vehicle] {
        &self.vehicles
    }

    pub fn find(&self, id: u32) -> Option<&Vehicle> {
        self.vehicles.iter().find(|vehicle| vehicle.id() == id)
    }

    pub fn available(&self) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.is_rentable())
            .collect()
    }
}
```

The `find` result is an `Option<&Vehicle>` because missing vehicles are normal, not necessarily errors. The `available` method returns *references*, not copied vehicles. Rust infers that those references cannot outlive the borrowed fleet.

## 3. Visibility and explicit public API

Create `src/vehicle.rs` by moving the Day 1 `VehicleStatus`, `Vehicle`, `VehicleError`, and `impl Vehicle` into the file. **Keep their implementations.** Add `pub` to the three type declarations and constructor/methods used outside that module. Keep vehicle fields **private** and add accessors:

```rust
// ADD inside impl Vehicle in src/vehicle.rs
pub fn id(&self) -> u32 {
    self.id
}

pub fn make(&self) -> &str {
    &self.make
}

pub fn model(&self) -> &str {
    &self.model
}

pub fn daily_rate_cents(&self) -> i64 {
    self.daily_rate_cents
}
```

Why private? If callers could freely change `daily_rate_cents` to `-10`, the constructor's invariant would be meaningless. Expose operations intentionally. To make the Day 1 constructor accessible, change its signature to `pub fn new`; likewise `pub fn is_rentable`, `pub fn display`, `pub fn put_into_maintenance`, and `pub fn mark_ready` if you added it. Keep the other field names unchanged.

Replace `src/main.rs` (**FULL FILE**) with:

```rust
mod fleet;
mod vehicle;

use fleet::Fleet;
use vehicle::Vehicle;

fn main() {
    let mut fleet = Fleet::new();
    fleet.add(Vehicle::new(1, "Toyota".into(), "Camry".into(), 2025, 6500)
        .expect("seed vehicle is valid"));
    fleet.add(Vehicle::new(2, "Jeep".into(), "Wrangler".into(), 2026, 14000)
        .expect("seed vehicle is valid"));

    for car in fleet.available() {
        car.display();
    }

    match fleet.find(2) {
        Some(car) => println!("Found {} {}", car.make(), car.model()),
        None => println!("Not found"),
    }
}
```

**Why `mod` and `use`?** `mod vehicle;` tells the compiler to include the module defined in `vehicle.rs`. `use vehicle::Vehicle;` brings its type name into scope. `pub` controls access *across module boundaries*; it is not inheritance or a property getter generator.

### Understanding closures and iterators

`iter()` creates an iterator over borrowed elements; `filter(|vehicle| ...)` uses a closure, an inline function-like value, to select items; `collect()` builds a collection of references. It is equally idiomatic to write a `for` loop when that is clearer. Avoid using iterator chains solely to appear sophisticated.

### Checkpoint and exercises

`cargo fmt && cargo check && cargo run && cargo test`

1. Add one vehicle, then print `fleet.all().len()`.
2. Return references to only vehicles costing less than 10,000 cents/day.
3. **EXPECTED COMPILER ERROR:** Try keeping an immutable reference from `fleet.find(1)` alive while calling `fleet.add(...)`, then using that reference. Explain the conflict and rewrite so the reference's final use occurs before `add`.
4. Compare `Vec<&Vehicle>` with `Vec<Vehicle>`. When would you choose the latter?

**Hints:** `.iter().filter(...).collect()`; Rust checks last uses rather than simply lexical scopes. An owned `Vec<Vehicle>` may be needed when values must outlive the fleet, but cloning could be required.

**Worked answer for 2:**

```rust
// ADD inside impl Fleet
pub fn affordable(&self, maximum_cents: i64) -> Vec<&Vehicle> {
    self.vehicles.iter()
        .filter(|vehicle| vehicle.daily_rate_cents() < maximum_cents)
        .collect()
}
```

**Explain it back:** Why is `all(&self) -> &[Vehicle]` more flexible than returning `&Vec<Vehicle>`? What will happen if the fleet is dropped while a returned reference is still being used?

---

# Day 3 — Express business rules with types, errors and tests

**Outcome:** You know `Option`, `Result`, `?`, traits, simple conversions, test organization, and why the data model should retain its own behavior.

## 1. `Option` is absence; `Result` is an operation that may fail

`fleet.find(900)` returns `None`: there's simply no car with that ID. Creating a vehicle with a negative rate returns `Err(VehicleError::InvalidRate)`: the caller attempted something invalid. These two meanings deserve separate types.

A common operation combines them:

```rust
// ILLUSTRATION
fn require_vehicle(fleet: &Fleet, id: u32) -> Result<&Vehicle, RentalError> {
    fleet.find(id).ok_or(RentalError::VehicleNotFound)
}
```

`.ok_or` turns absence into a meaningful domain error. Note: borrowing a result from a fleet is fine for reading; it cannot be returned from a function after its owner has been dropped.

## 2. The `?` operator

`?` is concise error propagation. Compare:

```rust
// ILLUSTRATION
let car = match Vehicle::new(1, make, model, year, rate) {
    Ok(car) => car,
    Err(error) => return Err(error),
};
```

With:

```rust
// ILLUSTRATION inside a function returning Result<_, VehicleError>
let car = Vehicle::new(1, make, model, year, rate)?;
```

The `?` operator returns early on `Err`, and can convert error types when a suitable `From` implementation exists. It does **not** catch a panic. We'll use it in service and HTTP layers.

## 3. Add customer data without sacrificing domain rules

Create `src/customer.rs` (**FULL FILE**):

```rust
#[derive(Debug, Clone)]
pub struct Customer {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, PartialEq, Eq)]
pub enum CustomerError {
    EmptyName,
    InvalidEmail,
}

impl Customer {
    pub fn new(id: u32, name: String, email: String)
        -> Result<Self, CustomerError>
    {
        if name.trim().is_empty() {
            return Err(CustomerError::EmptyName);
        }
        if !email.contains('@') {
            return Err(CustomerError::InvalidEmail);
        }
        Ok(Self { id, name, email })
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn name(&self) -> &str { &self.name }
    pub fn email(&self) -> &str { &self.email }
}
```

The email check is **deliberately simplified for practice**, not production-grade email validation. Validation belongs to the domain because every caller, including future HTTP handlers and tests, must obey the same rule.

Add `mod customer;` to `src/main.rs`. Construct two customers, handle a failed construction with `match`, and print them via getters.

## 4. Make tests part of the design

Tests are functions marked `#[test]`; `#[cfg(test)]` keeps their module out of normal builds. In `src/vehicle.rs`, add:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_negative_rate() {
        let result = Vehicle::new(
            1, "Toyota".into(), "Camry".into(), 2025, -1
        );
        assert!(matches!(result, Err(VehicleError::InvalidRate)));
    }

    #[test]
    fn maintenance_vehicle_is_not_rentable() {
        let mut car = Vehicle::new(
            1, "Toyota".into(), "Camry".into(), 2025, 6500
        ).expect("valid fixture");
        car.put_into_maintenance();
        assert!(!car.is_rentable());
    }
}
```

**Why `matches!`?** It checks an enum's variant without needing to compare the successful `Vehicle` type, which may not implement `PartialEq`.

## 5. Traits: use them when you need them

A trait describes behavior that several types can share, so code can work with any of them generically. For example:

```rust
// ILLUSTRATION ONLY: not part of our codebase
trait Identified {
    fn id(&self) -> u32;
}

impl Identified for Vehicle {
    fn id(&self) -> u32 {
        // Calls Vehicle's inherent `id()` getter (inherent methods
        // take priority), not this trait method.
        self.id()
    }
}
```

We won’t add this. Nothing in our code needs to treat vehicles and customers uniformly by ID, so the trait would only add a layer between callers and Vehicle::id(). Callers can use the concrete type and its getter directly.

Add a trait when a real need appears, such as:

- function that must accept several different types (fn log<T: Identified>(item: &T)),
- a collection of mixed types (Vec<Box<dyn Identified>>), or
- a seam you need for testing, such as a trait around a database or clock.

Until then, prefer concrete types. We also won’t mirror every struct with an IVehicle-style interface as you might in Java or C#. In Rust it adds code without adding capability, and it’s easy to introduce later if a need appears. 

## 6. Error-design exercise

Introduce a `RentalError` enum when you implement rental service operations, rather than returning `String`. Choose variants based on expected failures (`VehicleNotFound`, `CustomerNotFound`, `OverlappingReservation`, `VehicleUnavailable`). **Keep error enums at the layer that owns their meaning.** HTTP status codes do not belong inside the domain error enum.

### Checkpoint

`cargo fmt && cargo test && cargo check`

### Exercises

1. Write two tests for `Customer::new` (valid and invalid email).
2. Implement `Fleet::require(id) -> Result<&Vehicle, FleetError>` using `ok_or`.
3. Refactor one explicit `match` over a `Result` into a `?` expression inside a function that itself returns `Result`.
4. Explain the difference between an inherent method (`impl Vehicle`) and a trait implementation (`impl SomeTrait for Vehicle`).

**Worked answer for exercise 2:**

```rust
// ADD to src/fleet.rs
#[derive(Debug, PartialEq, Eq)]
pub enum FleetError {
    VehicleNotFound,
}

// ADD inside impl Fleet
pub fn require(&self, id: u32) -> Result<&Vehicle, FleetError> {
    self.find(id).ok_or(FleetError::VehicleNotFound)
}
```

**Explain it back:** When should a lookup return `Option` versus `Result`? Does `?` execute an exception handler? When should you avoid adding a trait?

---

# Day 4 — HTTP and asynchronous Rust, without discarding the domain

**Outcome:** Your existing vehicle domain powers a JSON web API. HTTP handlers are adapters, not the owners of business rules.

## 1. What changes when we add a server?

The request lifecycle is: client JSON → deserialize request into input DTO → call domain constructor/rule → serialize a response. Axum's job is HTTP; Serde's job is data representation; Tokio's job is running asynchronous tasks. **None of them replaces `Vehicle::new` or `Vehicle::is_rentable`.**

### Async mental model

`async fn` returns a future. `.await` yields while an asynchronous operation completes; it does not create a thread per request. A value held across `.await` may need to satisfy thread-safety (`Send`) requirements. `Arc<T>` shares ownership safely; a mutex protects simultaneous mutable access. Do not hold `std::sync::MutexGuard` across `.await` in an Axum handler. Prefer short lock scopes or, on Day 6, a database pool.

## 2. Add dependencies

From project root:

```bash
cargo add axum@0.8
cargo add tokio@1 --features full
cargo add serde@1 --features derive
cargo add serde_json@1
```

These commands create/modify Cargo.toml. Read the resulting feature flags. Preserve `Cargo.lock`.

## 3. Move to a library + small binary

A `lib.rs` module lets integration tests reuse the application. Move `src/vehicle.rs`, `src/fleet.rs`, `src/customer.rs` unchanged. Create `src/lib.rs` (**FULL FILE**):

```rust
pub mod customer;
pub mod fleet;
pub mod vehicle;
```

Remove `mod fleet; mod vehicle; mod customer;` from `main.rs`. Use `use rust_rent::...` to reference public library modules; Cargo converts package `rust-rent` to Rust crate identifier `rust_rent`.

## 4. Serialize explicit HTTP responses

Do not make every domain field public to satisfy JSON. Define a representation in `src/api.rs`. To do that cleanly, add a getter to `Vehicle`:

```rust
// ADD inside impl Vehicle in src/vehicle.rs
pub fn year(&self) -> u16 { self.year }
```

Create `src/api.rs` (**FULL FILE**):

```rust
use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::fleet::Fleet;
use crate::vehicle::Vehicle;

#[derive(Debug, Deserialize)]
pub struct CreateVehicle {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
}

#[derive(Debug, Serialize)]
pub struct VehicleResponse {
    id: u32,
    make: String,
    model: String,
    year: u16,
    daily_rate_cents: i64,
    rentable: bool,
}

impl From<&Vehicle> for VehicleResponse {
    fn from(v: &Vehicle) -> Self {
        Self {
            id: v.id(),
            make: v.make().to_owned(),
            model: v.model().to_owned(),
            year: v.year(),
            daily_rate_cents: v.daily_rate_cents(),
            rentable: v.is_rentable(),
        }
    }
}

#[derive(Clone, Default)]
pub struct AppState {
    pub fleet: Arc<Mutex<Fleet>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .with_state(state)
}

async fn list_vehicles(
    State(state): State<AppState>,
) -> Result<Json<Vec<VehicleResponse>>, StatusCode> {
    let fleet = state.fleet.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items = fleet.all().iter().map(VehicleResponse::from).collect();
    Ok(Json(items))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(input): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<VehicleResponse>), StatusCode> {
    let mut fleet = state.fleet.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let next_id = fleet.all().iter().map(|v| v.id()).max()
        .unwrap_or(0).checked_add(1)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let vehicle = Vehicle::new(
        next_id, input.make, input.model, input.year, input.daily_rate_cents
    ).map_err(|_| StatusCode::BAD_REQUEST)?;
    let response = VehicleResponse::from(&vehicle);
    fleet.add(vehicle);
    Ok((StatusCode::CREATED, Json(response)))
}
```

**Design walk-through:** `VehicleResponse` is an HTTP representation, not the domain model. `From<&Vehicle>` gives an explicit conversion; `.to_owned()` copies strings because the JSON response must own them after the mutex lock ends. This is a *justified clone*, not accidental duplication. The response type deliberately contains no `status` string implementation detail. `Json` is the final handler extractor because reading the request body must come after path/state/header extraction in Axum.

Add `pub mod api;` to `src/lib.rs`.

Replace `src/main.rs` (**FULL FILE**):

```rust
use rust_rent::api::{router, AppState};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = router(AppState::default());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("RustRent: http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
```

`Box<dyn std::error::Error>` here is only a convenient startup-error return type; it is not a replacement for typed domain errors. `#[tokio::main]` starts the async runtime.

## 5. Run your first API

Terminal one:

```bash
cargo fmt
cargo check
cargo run
```

Terminal two:

```bash
curl -i http://127.0.0.1:3000/health
curl -i -X POST http://127.0.0.1:3000/vehicles \
  -H 'Content-Type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'
curl -s http://127.0.0.1:3000/vehicles | jq
```

You should receive **201 Created** for the POST and a JSON array for GET. Restarting the server loses the data, by design—Day 6 introduces persistence.

### Checkpoint and exercises

1. Send a `0` daily rate and explain where the validation happens.
2. Call `/vehicles` twice. Why doesn't `get` mutate the fleet?
3. Predict what happens if you move the lock into a helper that calls `.await` before dropping the guard.
4. Why do `State` and `Json` appear in a handler's argument list rather than a framework-generated controller class?

**Explain it back:** Who owns the vehicles? Who owns the JSON response strings? What's the difference between a domain object and its API representation?

---

# Day 5 — REST design, errors and consistent application boundaries

**Outcome:** Implement lookup, update and delete endpoints and customer creation; learn how to shape APIs without embedding domain rules in transport code.

## 1. HTTP statuses and Rust errors

HTTP gives a client a standardized result, but the domain should speak in domain language. Map expected failures at the boundary: invalid input → 400, missing record → 404, conflicting operation → 409, unexpected infrastructure failure → 500. Malformed JSON/schema extraction may return Axum's default 400/415/422 text responses unless you customize rejections; **do not claim all errors are your JSON format yet**.

### Extend the vehicle's API intentionally

For update requests, do **not** expose `pub` fields and mutate them arbitrarily. Add domain methods that retain invariants. **ADD inside `impl Vehicle`**:

```rust
pub fn change_rate(&mut self, new_rate_cents: i64)
    -> Result<(), VehicleError>
{
    if new_rate_cents <= 0 {
        return Err(VehicleError::InvalidRate);
    }
    self.daily_rate_cents = new_rate_cents;
    Ok(())
}
```

The method protects the same invariant enforced by the constructor. This is a major point of idiomatic modeling: **construction invariants must remain true after construction**.

## 2. Add lookup and update routes

In `src/api.rs`, change your Axum imports to include `extract::Path`. Add:

```rust
// ADD near the other DTOs
#[derive(Debug, Deserialize)]
struct ChangeRate {
    daily_rate_cents: i64,
}

// ADD handlers
async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let fleet = state.fleet.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let vehicle = fleet.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(VehicleResponse::from(vehicle)))
}

async fn change_rate(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(input): Json<ChangeRate>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let mut fleet = state.fleet.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let vehicle = fleet.find_mut(id).ok_or(StatusCode::NOT_FOUND)?;
    vehicle.change_rate(input.daily_rate_cents)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(VehicleResponse::from(&*vehicle)))
}
```

To support the handler, **ADD to `impl Fleet`** in `src/fleet.rs`:

```rust
pub fn find_mut(&mut self, id: u32) -> Option<&mut Vehicle> {
    self.vehicles.iter_mut().find(|vehicle| vehicle.id() == id)
}
```

Replace the `Router::new()` chain in `router` with:

```rust
Router::new()
    .route("/health", get(|| async { "OK" }))
    .route("/vehicles", get(list_vehicles).post(create_vehicle))
    .route("/vehicles/{id}", get(get_vehicle).patch(change_rate))
    .with_state(state)
```

Why `PATCH` rather than `PUT` here? Because the request changes only a single field; it is a partial update. `PUT` normally means replacing a resource representation. The course makes its contract explicit rather than using the verbs interchangeably.

## 3. Deletion and borrowing

Add **inside `impl Fleet`**:

```rust
pub fn remove(&mut self, id: u32) -> Option<Vehicle> {
    let index = self.vehicles.iter().position(|vehicle| vehicle.id() == id)?;
    Some(self.vehicles.remove(index))
}
```

`?` works with `Option`, too: no matching index means return `None`. Here the removed vehicle is returned **by value** because the fleet no longer owns it. This is a useful contrast with `find`, which returns a borrowed reference.

**ADD handler** to `src/api.rs`:

```rust
async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let mut fleet = state.fleet.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    fleet.remove(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(StatusCode::NO_CONTENT)
}
```

Update the `/vehicles/{id}` route to:

```rust
.route("/vehicles/{id}",
    get(get_vehicle).patch(change_rate).delete(delete_vehicle))
```

This simplified Day 5 in-memory deletion is allowed because reservations haven't been persisted yet. Day 7 introduces database foreign keys and deletion rules.

## 4. Add customers — fully worked implementation

We will reuse `Customer::new`, rather than validating email again in the handler. This intentionally mirrors the vehicle design to teach how existing patterns transfer to new business concepts.

Create `src/customers.rs` (**FULL FILE**):

```rust
use crate::customer::Customer;

#[derive(Debug, Default)]
pub struct CustomerStore {
    customers: Vec<Customer>,
}

impl CustomerStore {
    pub fn all(&self) -> &[Customer] { &self.customers }

    pub fn next_id(&self) -> Option<u32> {
        self.customers.iter().map(Customer::id).max()
            .unwrap_or(0).checked_add(1)
    }

    pub fn has_email(&self, email: &str) -> bool {
        self.customers.iter().any(|customer| customer.email() == email)
    }

    pub fn add(&mut self, customer: Customer) {
        self.customers.push(customer);
    }
}
```

Add `pub mod customers;` to `src/lib.rs`. In `src/api.rs`, add `use crate::customer::Customer;` and `use crate::customers::CustomerStore;`.

Modify `AppState` (**REPLACE existing declaration**):

```rust
#[derive(Clone, Default)]
pub struct AppState {
    pub fleet: Arc<Mutex<Fleet>>,
    pub customers: Arc<Mutex<CustomerStore>>,
}
```

Define the HTTP request and response DTOs (**ADD to `src/api.rs`**):

```rust
#[derive(Debug, Deserialize)]
struct CreateCustomer {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct CustomerResponse {
    id: u32,
    name: String,
    email: String,
}

impl From<&Customer> for CustomerResponse {
    fn from(customer: &Customer) -> Self {
        Self {
            id: customer.id(),
            name: customer.name().to_owned(),
            email: customer.email().to_owned(),
        }
    }
}
```

Define the handlers (**ADD to `src/api.rs`**):

```rust
async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerResponse>>, StatusCode> {
    let customers = state.customers.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let responses = customers.all().iter().map(CustomerResponse::from).collect();
    Ok(Json(responses))
}

async fn create_customer(
    State(state): State<AppState>,
    Json(input): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<CustomerResponse>), StatusCode> {
    let mut store = state.customers.lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if store.has_email(&input.email) {
        return Err(StatusCode::CONFLICT);
    }
    let id = store.next_id().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let customer = Customer::new(id, input.name, input.email)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let response = CustomerResponse::from(&customer);
    store.add(customer);
    Ok((StatusCode::CREATED, Json(response)))
}
```

**ADD** to the router chain before `.with_state(state)`:

```rust
.route("/customers", get(list_customers).post(create_customer))
```

Test with:

```bash
curl -i -X POST http://127.0.0.1:3000/customers \
  -H 'Content-Type: application/json' \
  -d '{"name":"Avery","email":"avery@example.com"}'
curl -s http://127.0.0.1:3000/customers | jq
# Repeat the POST and check for HTTP 409.
```

The `CustomerStore` and `Fleet` are separate locks because they are independent in-memory collections. On Day 7, a reservation will need to coordinate data across tables transactionally instead.

## 5. Verify the endpoints

```bash
curl -i http://127.0.0.1:3000/vehicles/999
curl -i -X PATCH http://127.0.0.1:3000/vehicles/1 \
  -H 'Content-Type: application/json' \
  -d '{"daily_rate_cents":7200}'
curl -i -X DELETE http://127.0.0.1:3000/vehicles/1
```

Run `cargo fmt && cargo check && cargo test`. The sequence assumes you created vehicle 1 earlier in the same running process.

### Design challenge

Why should `Vehicle::change_rate` handle validation even if the handler already rejects negative prices? Because domain rules need to remain correct if the vehicle is manipulated by tests, a CLI, a background job or a future GraphQL transport.

**Explain it back:** When does `Option<Vehicle>` make sense instead of `Option<&Vehicle>`? Why is `get_vehicle` unable to change rate without obtaining a mutable vehicle? Why are HTTP status codes absent from `vehicle.rs`?

---

# Day 6 — Persist domain data with SQLite and SQLx

**Outcome:** You learn the difference between a domain entity, its database row, and an HTTP response. The application survives a restart.

## 1. Why a database changes the design

Your fleet currently lives in a `Vec`. It dies with the process. SQLite persists records. The problem changes from borrowing data in memory to converting between independently owned representations:

`SQLite row -> Vehicle domain type -> VehicleResponse`

These layers overlap in fields but have different responsibilities. Domain objects protect invariants; rows reflect storage; DTOs reflect HTTP contracts. Avoid treating SQLx's `FromRow` as a reason to eliminate domain methods.

## 2. Dependencies and migrations

Use the version documented for this curriculum's API examples. SQLx 0.9 requires a current Rust toolchain; verify your `rustc --version` (at least 1.94.0 for this line). Its official docs list the bundled SQLite feature as `sqlite` and the alternate explicit bundled flag in registry feature metadata; use the configuration from docs for the installed version and verify with `cargo tree -e features`.

```bash
cargo add sqlx@0.9 --no-default-features \
  --features runtime-tokio,sqlite,migrate,macros,derive
mkdir -p migrations
```

Create `migrations/202610270001_create_schema.sql` (**FULL FILE**):

```sql
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
    name TEXT NOT NULL,
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

CREATE INDEX reservations_vehicle_dates
ON reservations(vehicle_id, start_date, end_date);
```

The `status` column is physical operational status, **not** a signal that the vehicle is booked for every date. `start_date` is included, `end_date` excluded. ISO-formatted dates (`YYYY-MM-DD`) sort lexically in SQLite.

## 3. The pool is a shared handle, not a shared mutex

Create `src/database.rs` (**FULL FILE**):

```rust
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::str::FromStr;

pub async fn connect() -> Result<SqlitePool, Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str("sqlite://rust_rent.db")?
        .create_if_missing(true)
        .foreign_keys(true);
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await?;
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
```

`SqlitePool` is a cloneable handle to connection management. You generally do not wrap it in a mutex. Each query obtains a connection as needed. **Do not hold a DB transaction across unrelated network calls.**

## 4. Make `Vehicle` reconstitution explicit

Rows read from a database should be validated before becoming domain objects. Add a constructor to `impl Vehicle` called `reconstitute` that takes an existing ID, strings, year, positive rate, and a `VehicleStatus`. It should call `Vehicle::new(...)` first and then set the validated status. Add a `status(&self) -> VehicleStatus` getter. This is deliberately a small domain-level exercise: the compiler and your tests should enforce the API instead of allowing the repository to write private fields.

**Worked model solution (ADD to `impl Vehicle`):**

```rust
pub fn reconstitute(
    id: u32,
    make: String,
    model: String,
    year: u16,
    rate_cents: i64,
    status: VehicleStatus,
) -> Result<Self, VehicleError> {
    let mut vehicle = Self::new(id, make, model, year, rate_cents)?;
    vehicle.status = status;
    Ok(vehicle)
}

pub fn status(&self) -> VehicleStatus {
    self.status
}
```

## 5. Translate rows to domain values

Create `src/repository.rs` (**FULL FILE**, initial read-only repository):

```rust
use crate::vehicle::{Vehicle, VehicleStatus};
use sqlx::{FromRow, SqlitePool};

#[derive(Debug, FromRow)]
struct VehicleRow {
    id: i64,
    make: String,
    model: String,
    year: i64,
    daily_rate_cents: i64,
    status: String,
}

pub struct VehicleRepository {
    pool: SqlitePool,
}

impl VehicleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> Result<Vec<Vehicle>, Box<dyn std::error::Error>> {
        let rows: Vec<VehicleRow> = sqlx::query_as(
            "SELECT id, make, model, year, daily_rate_cents, status              FROM vehicles ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut vehicles = Vec::with_capacity(rows.len());
        for row in rows {
            let status = match row.status.as_str() {
                "ready" => VehicleStatus::Ready,
                "maintenance" => VehicleStatus::Maintenance,
                _ => return Err("Invalid vehicle status in database".into()),
            };
            let id = u32::try_from(row.id)?;
            let year = u16::try_from(row.year)?;
            let vehicle = Vehicle::reconstitute(
                id, row.make, row.model, year,
                row.daily_rate_cents, status
            ).map_err(|_| "Invalid vehicle row")?;
            vehicles.push(vehicle);
        }
        Ok(vehicles)
    }
}
```

The conversion from `i64` to `u32` / `u16` is **fallible**: database values can be outside the intended domain ranges. This is precisely why `TryFrom` and `?` matter. The `Box<dyn Error>` return is a teaching convenience only: a fuller service would use a concrete repository-error enum.

## 6. A deliberately narrow migration checkpoint

To avoid a giant "replace everything" step, Day 6 keeps the in-memory HTTP code from Day 5 running while you **independently prove the database layer**. Add `pub mod database; pub mod repository;` to `lib.rs`. Add an integration test to `tests/database.rs` that connects, runs migrations and calls `VehicleRepository::list()`. Start with an empty DB; it should return an empty vector. Insert a row using `sqlite3 rust_rent.db` or SQLx and run again. Do not pretend you have migrated all CRUD handlers yet.

Sample smoke commands after you have run the connection once:

```bash
sqlite3 rust_rent.db 'SELECT name FROM sqlite_master WHERE type="table";'
sqlite3 rust_rent.db 'SELECT * FROM vehicles;'
```

**Checkpoint definition:** A database layer compiles, migrates the schema, reads rows as `Vehicle` domain values, and passes at least one test. Day 7 completes its integration with HTTP. This is intentional incremental development.

### Exercises

1. Add `VehicleRepository::insert(&Vehicle)` using parameter-bound SQL and `sqlx::query(...).bind(...).execute(...)`.
2. Write a test showing `reconstitute` rejects invalid rates.
3. Explain why `try_from` is safer than casting a database ID using `as`.
4. Explain why a pool is not the same thing as `Arc<Mutex<Vec<_>>>`.

**Important:** The Day 6 material describes and exemplifies persistence mechanics but is not a claim that all Day 5 HTTP endpoints are now SQLite-backed. Your final integration work is defined on Day 7 below.

---

# Day 7 — Reservations, transactions and the full API design

**Outcome:** You implement time-based booking rules and understand the persistence boundary. This is the **largest module**; allow extra sessions.

## 1. What makes reservations harder than vehicles?

A vehicle may be ready for rental yet booked during a particular week. Booking availability is therefore **a question about an interval**, not a simple status field. For a half-open interval `[start, end)` (end date excluded):

```text
Existing: Nov 1 ├────────┤ Nov 4
Request:  Nov 4          ├─────┤ Nov 6   allowed
Request:  Nov 3      ├──────┤ Nov 5      conflict
```

The overlap predicate is: `start_a < end_b && end_a > start_b`. Cancelled reservations do not block dates.

## 2. Date arithmetic and pricing

```bash
cargo add chrono@0.4 --features serde
```

Create `src/reservation.rs` (**FULL FILE**):

```rust
use chrono::NaiveDate;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReservationStatus {
    Confirmed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservationError {
    InvalidDateRange,
    InvalidRate,
    PriceOverflow,
}

#[derive(Debug, Clone)]
pub struct Reservation {
    pub id: u32,
    pub vehicle_id: u32,
    pub customer_id: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_cents: i64,
    pub status: ReservationStatus,
}

impl Reservation {
    pub fn calculate_total(
        rate_cents: i64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<i64, ReservationError> {
        if rate_cents <= 0 {
            return Err(ReservationError::InvalidRate);
        }
        let days = (end_date - start_date).num_days();
        if days <= 0 {
            return Err(ReservationError::InvalidDateRange);
        }
        rate_cents.checked_mul(days)
            .ok_or(ReservationError::PriceOverflow)
    }

    pub fn overlaps(&self, start_date: NaiveDate, end_date: NaiveDate) -> bool {
        self.status == ReservationStatus::Confirmed
            && self.start_date < end_date
            && self.end_date > start_date
    }

    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_bookings_do_not_overlap() {
        let date = |d| NaiveDate::from_ymd_opt(2026, 11, d).unwrap();
        let booking = Reservation {
            id: 1, vehicle_id: 1, customer_id: 1,
            start_date: date(1), end_date: date(4),
            total_cents: 19500, status: ReservationStatus::Confirmed,
        };
        assert!(!booking.overlaps(date(4), date(7)));
        assert!(booking.overlaps(date(3), date(6)));
    }

    #[test]
    fn three_day_rate_is_exact_integer_money() {
        let date = |d| NaiveDate::from_ymd_opt(2026, 11, d).unwrap();
        assert_eq!(Reservation::calculate_total(6500, date(1), date(4)), Ok(19500));
    }
}
```

**Notice:** The price and interval rules are domain methods and unit tests; no Axum, SQLite or network calls are involved. That is *why we retained the domain model*.

## 3. SQL alone is not enough: concurrency

Naive algorithm:

1. SELECT whether dates are free.
2. If free, INSERT booking.

Two requests might both SELECT before either INSERTs. Use a single transaction that obtains the SQLite write lock **before** checking. SQLx 0.9 exposes `Pool::begin_with("BEGIN IMMEDIATE")`; for SQLite that creates a managed transaction. SQLx generally rolls back uncommitted transactions on drop; do not claim it perfectly handles every cancellation, disk-full or driver bug. This course is a single-database local teaching app, not a distributed booking system.

**ILLUSTRATION**, not a complete handler:

```rust
let mut tx = pool.begin_with("BEGIN IMMEDIATE").await?;
// SELECT vehicle, validate physical status and rate
// SELECT customer, ensure exists
// SELECT overlap with bind parameters using same transaction
// INSERT reservation if no overlap
// use &mut *tx for execute()/fetch_*() calls
// on ? early return: transaction guard is dropped
// on success:
tx.commit().await?;
```

SQLite supports one writer at a time; `BEGIN IMMEDIATE` gets that lock up front. **Do not interpolate inputs with `format!` SQL.** Use literal SQL plus `.bind(...)` to protect values and work with SQLx 0.9's SQL-string safety rules.

### Conflict SQL

```sql
SELECT COUNT(*)
FROM reservations
WHERE vehicle_id = ?
  AND status = 'confirmed'
  AND start_date < ?   -- bind requested end
  AND end_date > ?;   -- bind requested start
```

Always execute the conflict query and INSERT against the **same transaction**, not the general pool separately. Store dates as canonical ISO strings; do not use locale-dependent strings.

## 4. Make the final HTTP boundary explicit

Define a request DTO with `vehicle_id`, `customer_id`, `start_date`, `end_date`, with `NaiveDate` fields (Serde via the `chrono` feature). Define a response DTO with ID, dates, status and integer `total_cents`. A handler:

1. Extracts and validates JSON shape.
2. Invokes a `ReservationService` or repository operation.
3. Maps `InvalidDateRange` to 400, `NotFound` to 404, overlap to 409 and unexpected database failures to 500.
4. Returns 201 and a JSON body on success.

Be precise about Axum's defaults: malformed JSON and wrong content types can have different extractor rejection status codes and bodies than your explicit domain errors, unless you implement custom rejection mapping.

## 5. The required integration checklist

**This final integration is a capstone implementation assignment.** The fully wired SQL-backed vehicle/customer/reservation HTTP implementation is *not* silently provided in the snippets above. To call the project completed, implement and verify all these items:

- [ ] The running server initializes `SqlitePool` and migrations before binding the socket.
- [ ] Vehicle GET/POST are backed by SQLx rather than the temporary `Fleet` mutex.
- [ ] Customer GET/POST persist customers with a unique-email constraint.
- [ ] Reservation POST uses a write transaction that checks overlap and then inserts.
- [ ] Invalid ranges return a client error; overlaps return 409; adjacent bookings succeed.
- [ ] Cancellation changes reservation status and frees the interval.
- [ ] A restart preserves all records.
- [ ] Tests cover domain rules, HTTP 404, 400 and 409, database persistence, and concurrent booking conflict.
- [ ] `cargo fmt --check`, `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` pass.

**Why be this explicit?** Earlier course versions falsely implied that outline-level sketches were complete runnable code. This capstone is deliberately labeled as an implementation assignment rather than pretending the provided excerpts are a tested final application.

## 6. Test-first capstone scenarios

Before writing your reservation handler, make a table:

| Case | Business result | HTTP |
|---|---|---|
| New booking, valid dates | Created | 201 |
| End date equals start | Invalid range | 400 |
| Unknown vehicle | Missing record | 404 |
| Existing overlap | Conflict | 409 |
| Begins on previous end | Allowed | 201 |
| Cancelled previous booking | Slot available | 201 |
| Two concurrent overlapping requests | Exactly one commits | 201 + 409 |

The last case is important: happy-path tests are insufficient for transaction correctness.

## 7. Reflection: architecture after seven modules

`Vehicle` and `Reservation` continue to own domain invariants. A repository maps database rows to the domain. A service coordinates multiple domain entities and a transaction. An HTTP handler translates between JSON/HTTP and the application. This is not the only valid Rust architecture; it is a **clear, simple approach for this project's size**.

---

# Appendix A — Learning checkpoints and answers

### Day 1

Explain: `&self` is an immutable borrow, `&mut self` is exclusive mutable borrowing, and `Vehicle::new` has no receiver because the value doesn't exist yet. `Result` forces callers to consider a failed constructor.

### Day 2

Explain: moving transfers ownership; borrowing temporarily grants access; cloning explicitly duplicates values. An immutable slice accepts more kinds of data than `&Vec<_>`.

### Day 3

Explain: `Option` expresses absence, `Result` expresses success or failure. The `?` operator propagates a failure to the caller, possibly converting it through `From`. Traits describe shared behavior, but a struct doesn't need a matching trait to be idiomatic.

### Day 4

Explain: deserialization builds input values, `Vehicle::new` checks domain rules, and response conversion gives HTTP an owned representation. The lock guard is scoped to the handler's synchronous logic and must not live across an await.

### Day 5

Explain: an update must preserve construction invariants; `PATCH` changes part of a resource while `PUT` conventionally replaces one. Removing a vehicle returns an owned value, while finding one borrows.

### Day 6

Explain: database rows may be malformed or out of domain range; `TryFrom` and reconstitution guard that boundary. SQLx's pool manages connections; a mutex guards shared memory.

### Day 7

Explain: a car can be operational but already reserved on certain dates. Checking and inserting must be atomic; half-open ranges allow one booking to start at another's return date.

# Appendix B — Reference reading, with why it matters

- **The Rust Book** — https://doc.rust-lang.org/book/ — authoritative progression through variables, ownership, enums, traits, error handling and tests.
- **Rust by Example** — https://doc.rust-lang.org/rust-by-example/ — short, focused programs to try between lessons.
- **Rust API Guidelines** — https://rust-lang.github.io/api-guidelines/ — conventional naming, conversions, errors, common traits. Design guidance, not immutable law.
- **Rust Clippy** — https://doc.rust-lang.org/clippy/ — learn why common code patterns are flagged; do not blindly apply lints.
- **Axum docs** — https://docs.rs/axum/0.8/axum/ — routes, extractors, `State`, JSON and responses.
- **Tokio tutorial** — https://tokio.rs/tokio/tutorial — futures, async runtime, tasks and shared-state patterns.
- **SQLx 0.9 docs** — https://docs.rs/sqlx/0.9.0/sqlx/ — pools, queries, migrations and transactions.

# Appendix C — Common beginner stumbling blocks

| Compiler message or symptom | What it often means | First step |
|---|---|---|
| `use of moved value` | Ownership transferred | Decide: move, borrow or explicitly clone |
| `cannot borrow as mutable` | Binding is immutable or borrow conflicts | Check `mut` and active references |
| `private field` | A module boundary is doing its job | Add a domain method or getter |
| `? couldn't convert the error` | No matching `From` conversion | Map the error or define conversion |
| `trait bound ... not satisfied` | Function expects behavior type lacks | Read bound, derives, imports |
| `future cannot be sent between threads safely` | Non-Send value crosses await | Shorten lock scope; rethink captured data |
| `422` from JSON extractor | JSON structurally valid but wrong shape | Compare expected fields and types |
| `database is locked` | SQLite concurrent writes | Revisit transaction duration and busy timeout |

# Appendix D — Course quality standard and known limitations

A quality-reviewed course should ship compilable checkpoints, formatter output, tested reference projects and reproducible `Cargo.lock` files. This edition **has not passed that bar yet**; Rust/Cargo could not be run in the authoring environment. Its Day 5 customer extension and Day 7 full SQL-backed HTTP integration are explicitly labeled assignments rather than complete runnable milestones. These are meaningful remaining gaps if your requirement is a fully independently executable workbook. Verify each lesson locally, and treat compiler errors as issues to correct, not as evidence that you misunderstood the language.

**Next engineering validation:** package one `day-01` through `day-07` source snapshot per checkpoint; run `cargo check`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` on each in a clean Rust 2024 environment; run HTTP smoke tests for Days 4, 5 and 7. Record OS, `rustc --version`, resolved crate versions, and output logs. No such run is claimed here.

