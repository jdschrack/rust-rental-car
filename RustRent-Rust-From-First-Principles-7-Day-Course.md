# RustRent: Learn Rust From First Principles in Seven Days

**A beginner-first, explanation-led course culminating in a rental-car REST API**  
**Edition:** Rust 2024 · **Project:** RustRent

> **Start here.** You'll build a `Vehicle` domain model with methods and evolve it across the course. We do not introduce a web framework until you understand the language underneath it.

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


## Course contract: what “complete” means

This is a **beginner-first learning course**, not a source dump. Seven days are seven modules; a first-time programmer may spend multiple sessions on each. **Do not advance while an earlier checkpoint fails.** Every day has three distinct categories:

- **Required build:** files and commands needed for that day's checkpoint, with exact paths. These are never left to guesswork.
- **Discovery exercise:** a deliberately open implementation choice, always specifying the starting point, expected behavior, and way to verify it.
- **Optional extension:** may require further reading and is not a prerequisite for later days.

When you encounter **ILLUSTRATION**, do not paste it into the project expecting it to compile by itself. If a lesson requests a new function but doesn't supply every line, it must give a signature, test expectation, and hints. If a later day depends on an exercise, it is **required**, not optional.

### What a “service” means in this course

A **domain object** (e.g. `Vehicle`) owns rules about one object. A **collection** (e.g. `Fleet`) owns a set of objects and search/add operations. A **service** coordinates multiple objects in one business operation (e.g. making a reservation for a customer and vehicle). A **repository** reads and writes persistent data. An **HTTP handler** parses requests and returns responses. These boundaries are useful here, not universal rules for all Rust programs.

**There is no `RentalService` in Days 1–6.** Day 3 discusses what its error type could look like *later*, but does not require implementing it. Day 7 implements a guided `ReservationService` and SQL-backed tests. We will never tell you to complete an unspecified future operation.

### Quality and reproducibility

Use each day's commands and expected results to verify your progress on your computer. Appendix B links to the primary documentation for the Rust concepts and libraries used. Keep `Cargo.lock` with your project so the dependency versions are reproducible.

**Common Rust misunderstandings:** `pub struct` does not make its fields public; `use` does not grant visibility; `filter` returns an iterator, `find` returns an `Option`; `clone` may allocate; and `?` propagates errors but does not catch panics. We revisit these as they arise.

## Checking your progress

After you add or replace files, complete all named steps before running a checkpoint: a `pub mod` declaration requires its matching source file to exist. Run `cargo fmt` to format your code, `cargo check` to check compilation, and `cargo test` to run tests. Once you reach Day 4, also use `cargo clippy --all-targets -- -D warnings` where the lesson requires it.

**Warnings versus errors:** Early examples intentionally retain some methods that are not called by the current program. Rust may report these as `dead_code` warnings, even if a method is `pub` inside a binary crate. A warning does not by itself mean that a required example failed. Clippy's `-D warnings` option deliberately makes warnings fail the check, which is why the strict gate starts in Day 4.

If a lesson replaces `src/main.rs` or another file, that new version becomes the starting point for the next step. Your earlier experiments are still useful for learning, but do not blindly append old demonstration code to a replacement file. If something fails, compare the current files with the labeled **FULL FILE** or **ADD** instructions before continuing.

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

Recommended habits: use `cargo fmt`, `cargo check`, and `cargo test` from Day 1. Starting on Day 4, also run `cargo clippy --all-targets -- -D warnings` at the required checkpoints. Earlier binary examples intentionally retain some unused demonstration methods, so their ordinary dead-code warnings are not a failure of the Day 1–3 checkpoints.

**Your local compiler is the authority:** run the commands at each checkpoint. If a required command fails, inspect the error and verify you've completed every file edit for that checkpoint before advancing.

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

1. Add `mark_ready(&mut self)` inside `impl Vehicle`; in `main`, immediately after `car.put_into_maintenance(); car.display();`, add `car.mark_ready(); car.display();`. The final display should show `Ready`.
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

Moving transfers ownership; it does not duplicate memory. If both names independently owned the same allocation, freeing both could be unsafe. You have **three distinct options** (**ILLUSTRATION**: wrap these statements in a scratch `fn main()`):

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

## 2. Refactor into modules without losing the domain model

**Starting point:** Day 1's complete `src/main.rs` compiles and uses `Vehicle`, `VehicleStatus` and `VehicleError`.

**Goal:** extract those same types and their methods to `src/vehicle.rs`, add a `Fleet` collection, and have `main.rs` use both. This is a move of existing behavior, **not** a replacement with a data-only struct.

### Why modules change visibility

Rust items are private by default to code outside their module. Making a type `pub` does **not** make its fields or its methods public. The compiler checks each boundary separately. Prefer **private fields with purposeful methods** for a type with validated invariants; direct public fields are fine for simple transport data.

Create `src/vehicle.rs` (**FULL FILE**):

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleStatus {
    Ready,
    Maintenance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VehicleError {
    EmptyMake,
    EmptyModel,
    InvalidRate,
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u16,
    daily_rate_cents: i64,
    status: VehicleStatus,
}

impl Vehicle {
    pub fn new(
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

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn make(&self) -> &str {
        &self.make
    }

    pub fn model(&self) -> &str {
        &self.model
    }

    pub fn year(&self) -> u16 {
        self.year
    }

    pub fn daily_rate_cents(&self) -> i64 {
        self.daily_rate_cents
    }

    pub fn is_rentable(&self) -> bool {
        self.status == VehicleStatus::Ready
    }

    pub fn put_into_maintenance(&mut self) {
        self.status = VehicleStatus::Maintenance;
    }

    pub fn mark_ready(&mut self) {
        self.status = VehicleStatus::Ready;
    }

    pub fn display(&self) {
        println!(
            "#{}: {} {} ({}), {} cents/day, {:?}",
            self.id,
            self.make,
            self.model,
            self.year,
            self.daily_rate_cents,
            self.status
        );
    }
}
```

**Design note:** Rust may warn about unused methods or variants after the Day 2 `main.rs` replacement. Even `pub` methods in a binary crate can be reported as dead code. This is expected for the learning stage, not a build error. Strict Clippy is required after the Day 4 library refactor.

Create `src/fleet.rs` (**FULL FILE**):

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

**Why these methods?** `add` consumes a `Vehicle` (the fleet becomes its owner). `all` borrows a slice without copying. `find` yields zero or one borrowed match (`Option<&Vehicle>`), while `filter` yields an iterator of all matching elements; `collect` materializes those references into `Vec<&Vehicle>`. Don't substitute `filter` for `find`: their return types differ.

Replace `src/main.rs` (**FULL FILE**):

```rust
mod fleet;
mod vehicle;

use fleet::Fleet;
use vehicle::Vehicle;

fn main() {
    let mut fleet = Fleet::new();
    fleet.add(
        Vehicle::new(1, "Toyota".into(), "Camry".into(), 2025, 6500)
            .expect("valid example vehicle"),
    );
    fleet.add(
        Vehicle::new(2, "Jeep".into(), "Wrangler".into(), 2026, 14000)
            .expect("valid example vehicle"),
    );

    for car in fleet.available() {
        car.display();
    }

    match fleet.find(2) {
        Some(car) => println!("Found {} {}", car.make(), car.model()),
        None => println!("Vehicle not found"),
    }
}
```

`mod` declares the module; `use` imports a name into scope; `pub` decides whether it is accessible. We only use `expect` for deterministic seed data—not for handling arbitrary input in HTTP handlers.

**Day transition:** The Day 2 FULL FILE intentionally replaces Day 1's `main` demonstration and exercise calls, but preserves all vehicle methods in `src/vehicle.rs`. Commit the Day 1 snapshot if you want to replay its CLI demonstration.

### Visibility experiment (deliberately break, then restore)

1. Remove `pub` from `struct Vehicle`. Run `cargo check` and find the privacy diagnostic; restore.
2. Remove `pub` from `Vehicle::new`. The type is accessible, but constructing it from `main` is not; restore.
3. Try `car.make` rather than `car.make()` in `main`. The field is private; the public method is available.
4. Compare returning `&str` from a getter with returning a cloned `String`. The former borrows from the vehicle; it cannot outlive that vehicle.

### Day 2 exercises

**Required:** In `src/fleet.rs`, write `pub fn affordable(&self, maximum_cents: i64) -> Vec<&Vehicle>`. Test it by printing the IDs of vehicles priced below 10,000 cents/day in `main.rs`. Expected: the Camry appears, the Wrangler does not. **Hint:** `iter().filter(...).collect()` and `Vehicle::daily_rate_cents()`.

**Discovery (in a disposable copy of the Day 2 RustRent project, not a new empty crate):** Create a program that keeps a borrowed result from `fleet.find(1)`, then tries to modify the fleet with `add` before the borrow's last use. Observe the borrow-checker diagnostic. Rearrange the last use to make the code compile. Explain *why* it becomes valid.

**Required call site:** In `src/main.rs`, after adding the two vehicles, write `for car in fleet.affordable(10_000) { println!("Affordable: #{}", car.id()); }`. Expect only `Affordable: #1`.

**Why not index by ID?** `fleet.all()[0]` panics on an empty collection, `fleet.all().get(0)` returns `Option<&Vehicle>`, and `fleet.find(1)` searches by a vehicle's actual ID, not its position.

**Worked solution for required exercise:**

```rust
// ADD inside impl Fleet, in src/fleet.rs
pub fn affordable(&self, maximum_cents: i64) -> Vec<&Vehicle> {
    self.vehicles
        .iter()
        .filter(|vehicle| vehicle.daily_rate_cents() < maximum_cents)
        .collect()
}
```

**Checkpoint:** `cargo fmt && cargo check && cargo run && cargo test`. No private-access or mismatched-iterator-type errors. Explain the difference between owned `Vec<Vehicle>` and borrowed `Vec<&Vehicle>`.

---



# Day 3 — Express business rules with types, errors and tests

**Outcome:** You know `Option`, `Result`, `?`, traits, simple conversions, test organization, and why the data model should retain its own behavior.

## 1. `Option` is absence; `Result` is an operation that may fail

`fleet.find(900)` returns `None`: there's simply no car with that ID. Creating a vehicle with a negative rate returns `Err(VehicleError::InvalidRate)`: the caller attempted something invalid. These two meanings deserve separate types.

A common operation combines them:

```rust
// ILLUSTRATION
enum LookupError { VehicleNotFound }
fn require_vehicle(fleet: &Fleet, id: u32) -> Result<&Vehicle, LookupError> {
    fleet.find(id).ok_or(LookupError::VehicleNotFound)
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

In `src/main.rs`, **ADD** `mod customer;` with the other module declarations and `use customer::Customer;` with the imports. At the end of `main`, add:

```rust
for (id, name, email) in [(1, "Avery", "avery@example.com"), (2, "Jordan", "jordan@example.com"), (3, "Invalid", "no-at-sign")] {
    match Customer::new(id, name.into(), email.into()) {
        Ok(c) => println!("Customer #{}: {} ({})", c.id(), c.name(), c.email()),
        Err(e) => println!("Customer rejected: {e:?}"),
    }
}
```

Expected: two valid customers followed by `Customer rejected: InvalidEmail`.

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

## 6. Error-design lab: now versus later

**Where you are:** Day 3 has `Vehicle`, `Fleet`, and `Customer`. It does **not** have a reservation service, database, or HTTP routes yet.

**Required now:** `Fleet::require(id) -> Result<&Vehicle, FleetError>` is the only new service-like lookup. `FleetError` belongs in `src/fleet.rs` because `Fleet` owns the lookup. Don't create `RentalService` yet.

**Illustration of a future design (not an assignment today):** On Day 7 we introduce reservation operations such as `create_reservation(vehicle_id, customer_id, start_date, end_date)` and cancellation. Those operations will need an error representing “vehicle not found,” “customer not found,” “date conflict,” and “vehicle under maintenance.” An enum named `RentalError` would be a reasonable choice. It should **not** contain HTTP status numbers: the domain does not know whether its caller is HTTP, CLI, or a test.

**Discovery question (optional, no implementation required):** Should `VehicleNotFound` include the requested ID? What becomes easier to log or test if it does? Write a proposed enum on paper. We'll revisit this only once the reservation operation actually exists.

**Why not assign it now?** A type without an operation that returns it is speculative architecture, not a lesson in `Result`. The Day 3 checkpoint requires only the real `FleetError` exercise below.


### Checkpoint

`cargo fmt && cargo test && cargo check`

### Exercises

1. In `src/customer.rs`, add a `#[cfg(test)] mod tests { use super::*; /* tests here */ }` at the end, mirroring the vehicle tests. Test valid and invalid email; expect two more passing tests.
2. Implement `Fleet::require(id) -> Result<&Vehicle, FleetError>` using `ok_or`.
3. In `src/main.rs`, change the import to `use vehicle::{Vehicle, VehicleError};` and define `fn example_vehicle(rate: i64) -> Result<Vehicle, VehicleError> { let vehicle = Vehicle::new(3, "Honda".into(), "Civic".into(), 2025, rate)?; println!("Constructed vehicle #{}", vehicle.id()); Ok(vehicle) }` outside `main`. Inside `main`, call `example_vehicle(6500)` and `example_vehicle(-5)`, using `match` for both. Expected: one `Ok` and one `Err(InvalidRate)`. The `println!` after `?` makes it clear that execution continues only for a valid vehicle. This deliberately uses `?` to practice early error propagation, not just to shorten a one-expression function.
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

# Day 4 — From domain model to HTTP: async, shared state, Serde, and Axum

**Outcome:** Run an HTTP API without discarding the `Vehicle` model. You will understand what each new abstraction solves *before* combining it with the server.

**Starting point:** Day 3's `src/vehicle.rs`, `src/fleet.rs`, and `src/customer.rs` exist and `cargo test` passes. The fleet owns vehicles; vehicle fields remain private. Today **only** `GET /health`, `GET /vehicles`, and `POST /vehicles` are required; Day 5 expands the API.

**Estimated time:** This module may take multiple sessions. The source listings below are complete for every *new* file. Any existing domain file not explicitly changed remains unchanged.

## 1. What problem does asynchronous Rust solve?

A server spends time waiting for network I/O. If one request is waiting for data, an async runtime can poll other ready tasks rather than dedicating a blocked OS thread to each wait. An `async fn` produces a **future**, a value describing work that can be polled. Calling an async function alone does not execute it to completion. `.await` waits for its future to finish while allowing the executor to schedule other work. It does not automatically spawn a thread or guarantee parallel execution.

**Temporary experiment:** create a separate scratch project, *outside RustRent*, with `cargo new async-experiment`; in it run `cargo add tokio@1 --features full`. Replace its `src/main.rs` with:

```rust
use tokio::time::{sleep, Duration};

async fn greet() {
    println!("Before waiting");
    sleep(Duration::from_millis(100)).await;
    println!("After waiting");
}

#[tokio::main]
async fn main() {
    let future = greet();
    println!("Future created");
    future.await;
}
```

Predict the output *before* running `cargo run`. You should see “Future created” before “Before waiting.” Why? `greet()` constructs a future; awaiting it drives it. In this experiment, the runtime is supplied by the `#[tokio::main]` attribute. Return to the `rust-rent` project afterward.

**Discovery:** remove `future.await`; predict whether the greeting prints. Then restore it. Read [The Rust Book on futures and async](https://doc.rust-lang.org/book/ch17-00-async-await.html) and [Tokio's tutorial](https://tokio.rs/tokio/tutorial) if the result surprises you.

## 2. Why state must be shared

Each HTTP request invokes a handler. If each handler creates its own `Fleet`, one request could create a vehicle and the next would see an empty collection. We need a shared, in-memory fleet *for this stage of the course*.

`Arc<T>` (atomically reference-counted ownership) lets independently owned handles refer to the same allocation; cloning an `Arc` does **not** clone the entire fleet. `Mutex<T>` permits one task/thread at a time to mutate the enclosed value. `lock()` returns a guard, which unlocks the mutex when dropped.

**Small mental exercise:** `Arc<Mutex<Fleet>>` is three different jobs: `Fleet` contains domain data; `Mutex` synchronizes access; `Arc` allows multiple owners of the same synchronized allocation.

For handlers below, locking is brief and no `.await` happens while a `std::sync::MutexGuard` is held. A synchronous mutex is acceptable for this short in-memory critical section. Holding that guard across `.await` in a multithreaded handler can make the future non-`Send` or cause contention/deadlock. We will use a database pool later, not an indefinitely growing shared-memory lock.

**Experiment (optional, not required for the app):** in a scratch program, clone an `Arc<Mutex<Vec<i32>>>`, push to the vector through one clone, then read through the other. Explain why both see the same vector.

## 3. Add HTTP and JSON dependencies

From the `rust-rent` project root:

```bash
cargo add axum@0.8
cargo add tokio@1 --features full
cargo add serde@1 --features derive
cargo add serde_json@1
```

**Why these crates?** Tokio schedules async I/O; Axum routes HTTP requests and extracts parameters/body; Serde serializes Rust data to formats such as JSON and deserializes requests into Rust types; `serde_json` provides the JSON format implementation. Versions shown correspond to this course's API examples, not a claim that they are permanently newest.

### Serde experiment before HTTP

Create `src/bin/json_experiment.rs` (**FULL FILE**; `src/bin` is a directory that Cargo treats as another executable):

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Example {
    make: String,
    year: u16,
}

fn main() -> Result<(), serde_json::Error> {
    let car = Example {
        make: "Toyota".into(),
        year: 2025,
    };
    let json = serde_json::to_string(&car)?;
    println!("{json}");
    let decoded: Example = serde_json::from_str(&json)?;
    println!("{decoded:?}");
    Ok(())
}
```

Run `cargo run --bin json_experiment`. Expected first line: `{"make":"Toyota","year":2025}`. `Serialize` means a type can be turned into a format; `Deserialize` means a type can be built from input. Derive macros generate the boilerplate. `?` propagates conversion errors as you learned on Day 3. Leave this file in place; when the package has multiple binaries, **use `cargo run --bin rust-rent`** to start the server.

## 4. Create a library crate without moving files

**Do not move `src/vehicle.rs`, `src/fleet.rs`, or `src/customer.rs`.** They're already in the correct directory. We are changing *which crate root declares the modules*, not relocating them.

Before Day 4:
```text
src/
  main.rs       (contains mod vehicle; mod fleet; mod customer;)
  vehicle.rs
  fleet.rs
  customer.rs
```

Create `src/lib.rs` (**FULL FILE**):

```rust
pub mod customer;
pub mod fleet;
pub mod vehicle;
pub mod api;
```

The `api` module is created in Section 6. Complete `src/api.rs` before running the compiler. **Remove** `mod vehicle;`, `mod fleet;`, and `mod customer;` declarations from `src/main.rs` when you replace it below. The library's `lib.rs` now declares these modules once. `main.rs` uses `rust_rent::...` to import them from the library. Package name `rust-rent` becomes the crate identifier `rust_rent` under normal Cargo conventions.

Rust visibility still applies: `pub mod vehicle` allows external code to name the module, but individual fields remain private. We already exposed purposeful methods such as `Vehicle::make()` and `Vehicle::year()` on Day 2. **Do not add `year()` a second time.**

## 5. Why HTTP request and response types differ from domain types

Our domain `Vehicle` validates its own construction and hides fields. A JSON request should not be able to assign an ID or physical status arbitrarily. Therefore:

- `CreateVehicle` describes what a client can submit.
- `VehicleResponse` describes what clients can read.
- `Vehicle` remains the authoritative validated domain model.

Before using it in a handler, learn the standard library's `From` trait:

```rust
// ILLUSTRATION: this is not a standalone file.
struct Dollars(i64);
struct Cents(i64);

impl From<Dollars> for Cents {
    fn from(value: Dollars) -> Self {
        Cents(value.0 * 100)
    }
}

let cents = Cents::from(Dollars(5));
```

`From<Source> for Target` describes an *infallible* conversion that can always produce the target value. When a conversion can fail, prefer `TryFrom`. For our API, converting `&Vehicle` into an owned `VehicleResponse` cannot fail. The reference means we **borrow** the vehicle; its strings are duplicated into the response so the JSON can outlive the lock guard. That deliberate allocation is different from arbitrarily cloning the entire domain object.

## 6. Implement the HTTP module

Create `src/api.rs` (**FULL FILE**):

```rust
use std::sync::{Arc, Mutex};

use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

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
    fn from(vehicle: &Vehicle) -> Self {
        Self {
            id: vehicle.id(),
            make: vehicle.make().to_owned(),
            model: vehicle.model().to_owned(),
            year: vehicle.year(),
            daily_rate_cents: vehicle.daily_rate_cents(),
            rentable: vehicle.is_rentable(),
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
    let fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let items: Vec<VehicleResponse> =
        fleet.all().iter().map(VehicleResponse::from).collect();
    Ok(Json(items))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(input): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<VehicleResponse>), StatusCode> {
    // Input is deserialized before the handler runs. Domain validation
    // remains in Vehicle::new rather than duplicated in HTTP code.
    let mut fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let next_id = fleet
        .all()
        .iter()
        .map(|vehicle| vehicle.id())
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let vehicle = Vehicle::new(
        next_id,
        input.make,
        input.model,
        input.year,
        input.daily_rate_cents,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let response = VehicleResponse::from(&vehicle);
    fleet.add(vehicle);
    Ok((StatusCode::CREATED, Json(response)))
}
```

### Read the handler line by line

`State(state): State<AppState>` obtains a shared handle configured on the router. `Json(input): Json<CreateVehicle>` asks Axum/Serde to parse the request body. It goes **last** because it consumes the request body, whereas `State` does not. This is an Axum extractor constraint, documented in [Axum's extractor guide](https://docs.rs/axum/0.8/axum/extract/index.html#the-order-of-extractors).

`fleet.lock()` gives temporary exclusive access to the vector. `map_err` turns a poisoned mutex error into HTTP 500; the `?` operator exits the handler early if locking fails. The return type `Result<(StatusCode, Json<VehicleResponse>), StatusCode>` says success produces a status and JSON body; failure produces a status code. Day 5 expands error responses.

The ID expression is short but combines several operations. Read it as: iterate over vehicles; select their IDs; take the maximum (`Option<u32>`); use zero if there are none; add one safely with `checked_add`; or return HTTP 500 on overflow. This is a **temporary in-memory strategy**, not a database ID policy or a solution for distributed servers.

`Vehicle::new` preserves the Day 1 validation rules. `VehicleResponse::from(&vehicle)` borrows the domain object and creates an owned HTTP representation. `fleet.add(vehicle)` then **moves ownership** of the vehicle into the collection. This is exactly the ownership model from Day 2, now applied to HTTP.

**Design limitation:** A request with structurally invalid JSON can be rejected by Axum before this handler runs (with framework-defined status/body). A syntactically valid request with `daily_rate_cents <= 0` gets our explicit 400, currently without JSON detail. Day 5 improves domain error mapping. Don't claim all errors already share one format.

## 7. Start the server

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

`#[tokio::main]` supplies the runtime, `TcpListener::bind(...).await` awaits an asynchronous socket bind, and `axum::serve` handles requests. `Box<dyn Error>` is a convenient startup error type, not a recommendation to replace typed domain errors.

Project layout now:

```text
rust-rent/
  Cargo.toml
  Cargo.lock
  src/
    lib.rs
    main.rs
    api.rs
    vehicle.rs
    fleet.rs
    customer.rs
    bin/
      json_experiment.rs
```

From terminal one:

```bash
cargo fmt
cargo check --all-targets
cargo test
cargo run --bin rust-rent
```

From terminal two:

```bash
curl -i http://127.0.0.1:3000/health
curl -i -X POST http://127.0.0.1:3000/vehicles \
  -H 'Content-Type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'
curl -s http://127.0.0.1:3000/vehicles | jq
curl -i -X POST http://127.0.0.1:3000/vehicles \
  -H 'Content-Type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":0}'
```

Expected: `/health` returns HTTP 200 with `OK`; valid POST returns HTTP 201 and a JSON vehicle; GET lists that vehicle; zero-rate POST returns HTTP 400. Data disappears when the server restarts because our fleet lives in memory.

## 8. Exercises: defined objectives, room to discover

**Exercise A (required): Count vehicles.** The `VehicleCount` type and `count_vehicles` handler do not exist yet in the Day 4 FULL FILE; add both from the worked answer after attempting the exercise. Before completion, the Day 4 application does not expose `/vehicles/count`. Starting from the working Day 4 `src/api.rs`, implement `GET /vehicles/count`. It must return JSON `{"count":2}` after creating two vehicles. Reuse `AppState` and read-only fleet access; do not create a new fleet or mutate one. **Hint:** create a small `#[derive(Serialize)]` response struct and a handler returning `Json<...>`. Add the more specific `/vehicles/count` route before Day 5 introduces `/vehicles/{id}`. Verify with `curl`.

**Exercise B (discovery): Ownership of response strings.** In the `From<&Vehicle>` implementation, attempt to hold borrowed `&str` fields in a response struct. Can that response safely outlive the mutex guard? Why does returning owned `String` avoid that lifetime requirement? Restore the original response afterward. This is deliberately exploratory: the expected outcome is an explanation, not necessarily a compiling borrowed DTO.

**Exercise C (optional extension): Structured validation errors.** For a request with a zero daily rate, return `{"error":"invalid daily rate"}` alongside HTTP 400. Keep the actual validation inside `Vehicle::new`; translate the returned `VehicleError` at the HTTP boundary. Explain why that error string doesn't belong inside the domain model. Day 5 will provide a more systematic error shape.

**Verification checkpoint:** stop the server and run `cargo fmt && cargo fmt --check && cargo check --all-targets && cargo test && cargo clippy --all-targets -- -D warnings`; then repeat the four HTTP checks above. Tell another learner what `Arc`, `Mutex`, `From<&Vehicle>`, `Json`, `State`, and `.await` each contribute. If you cannot explain one, revisit its isolated experiment before continuing.

**Worked answer for Exercise A (read after attempting):**

```rust
// ADD to src/api.rs, after VehicleResponse
#[derive(Serialize)]
struct VehicleCount {
    count: usize,
}

// ADD to src/api.rs, alongside the other handlers
async fn count_vehicles(State(state): State<AppState>) -> Result<Json<VehicleCount>, StatusCode> {
    let fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(VehicleCount {
        count: fleet.all().len(),
    }))
}

// ADD inside router(), before .with_state(state)
.route("/vehicles/count", get(count_vehicles))
```

**Before Day 5:** Keep your domain models unchanged; do not add the Day 5 CRUD endpoints yet. The new Rust features this day were async futures, a runtime, shared ownership and synchronization, Serde derives, conversion traits, and Axum extractors—not new vehicle business rules.

---

# Day 5 — REST design, errors and consistent application boundaries

**Outcome:** Implement lookup, update and delete endpoints and customer creation; learn how to shape APIs without embedding domain rules in transport code.

## 1. HTTP statuses and Rust errors

In Axum, `Result<Json<T>, StatusCode>` produces a default **200 OK** plus JSON on success; `Result<(StatusCode, Json<T>), StatusCode>` explicitly chooses a different success status, such as **201 Created**. Both are idiomatic; the appropriate signature follows HTTP semantics.

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

In `src/api.rs`, change the Axum import to include both extractors: `use axum::{extract::{Path, State}, http::StatusCode, routing::get, Json, Router};`. Add the `ChangeRate` DTO after existing DTOs and the handlers after `create_vehicle`:

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
    .route("/vehicles/count", get(count_vehicles)) // retain Day 4 required route
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

This simplified Day 5 in-memory deletion is allowed because reservations haven't been persisted yet. Day 6 introduces SQLite foreign keys and the handling of deletion conflicts.

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

First POST a vehicle to the newly started in-memory server; the previous day's data disappears on restart:

```bash
curl -i -X POST http://127.0.0.1:3000/vehicles -H 'Content-Type: application/json' -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'
```

Confirm `GET /vehicles/count` still works, then:

```bash
curl -i http://127.0.0.1:3000/vehicles/999
curl -i -X PATCH http://127.0.0.1:3000/vehicles/1 \
  -H 'Content-Type: application/json' \
  -d '{"daily_rate_cents":7200}'
curl -i -X DELETE http://127.0.0.1:3000/vehicles/1
```

Run `cargo fmt && cargo check && cargo test && cargo clippy --all-targets -- -D warnings`. The sequence assumes you created vehicle 1 earlier in the same running process.

### Design challenge

Why should `Vehicle::change_rate` handle validation even if the handler already rejects negative prices? Because domain rules need to remain correct if the vehicle is manipulated by tests, a CLI, a background job or a future GraphQL transport.

**Explain it back:** When does `Option<Vehicle>` make sense instead of `Option<&Vehicle>`? Why is `get_vehicle` unable to change rate without obtaining a mutable vehicle? Why are HTTP status codes absent from `vehicle.rs`?

---

# Day 6 — Replace in-memory storage with SQLite and SQLx

**Outcome:** Every existing vehicle and customer HTTP endpoint uses SQLite, not the Day 5 in-memory stores. Records survive an application restart. The `Vehicle` and `Customer` domain models remain responsible for validation.

**Starting point:** Day 5 routes work: `GET/POST /vehicles`, `GET/PATCH/DELETE /vehicles/{id}`, and `GET/POST /customers`. You already have `src/api.rs`, `src/main.rs`, `src/lib.rs`, `src/vehicle.rs`, and `src/customer.rs`.

**Required stages:** (A) create migration/database files, (B) finish the repository module, (C) complete HTTP integration, (D) test persistence. **Do not run the first compilation at A:** `lib.rs` refers to `repository` before that file is created in B. We replace entire files where indicated so there is no guesswork about how code fits together. This is a longer module; complete each checkpoint before proceeding.

## 1. Mental model: why add another layer?

Day 5's `Arc<Mutex<Fleet>>` owns a vector inside one process. When the server stops, its contents disappear. We want to store records independently of the process. SQLite provides the persistent data; SQLx provides async access.

The request path we will implement is:

`HTTP request → Axum handler → VehicleRepository → SQLx connection pool → SQLite`

A read traverses the reverse path: `SQLite row → validated Vehicle → VehicleResponse → JSON`. `#[derive(FromRow)]` decodes SQL columns into a storage representation (`VehicleRow`); `TryFrom<VehicleRow>` then validates numeric ranges and reconstitutes the domain object. They are different steps. `u16::try_from(100_000_i64)` fails rather than silently truncating.

Why *not* place SQL in `impl Vehicle`? A vehicle's daily-rate and maintenance rules should work in unit tests or a future CLI without needing SQLite. The repository owns storage decisions. A repository is a concrete struct with an `impl`, **not** a trait merely because it has a repository name.

We intentionally replace both customer and vehicle stores **at the same time**. From this day forward, `AppState` holds a single database pool, not the two in-memory mutexes. `fleet.rs` and `customers.rs` can remain in the project as historical learning artifacts, but the HTTP handlers no longer import or use them.

## 2. Dependencies and schema — Checkpoint A

From the project root, confirm `rustc --version` is at least 1.94 for this course's SQLx 0.9 examples. Add SQLx and a temporary-directory test dependency:

```bash
cargo add sqlx@0.9 --no-default-features \
  --features runtime-tokio,sqlite-bundled,migrate,macros,derive
cargo add --dev tempfile
mkdir -p migrations tests
```

The explicit `sqlite-bundled` feature compiles against a bundled SQLite distribution; your Arch `sqlite` package is useful for the `sqlite3` command-line inspector. SQLx's migration macro needs both `migrate` and `macros`. Keep `Cargo.lock` checked in. If installed crate APIs or features differ, check [SQLx 0.9 documentation](https://docs.rs/sqlx/0.9.0/sqlx/) before modifying code.

Create `migrations/202610300001_initial.sql` (**FULL FILE**):

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

CREATE INDEX reservations_vehicle_dates
ON reservations(vehicle_id, start_date, end_date);
```

This schema uses SQLite `INTEGER` identifiers (`i64` in SQLx), while the Day 1 domain types use `u32`. We **check** conversions instead of casting with `as`, because database integers could exceed a domain type's range. On Day 7 the reservations table becomes active. Dates will be ISO strings representing half-open intervals (`[start_date, end_date)`). The SQL constraints supplement, rather than replace, domain validation.

Create `src/database.rs` (**FULL FILE**):

```rust
use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use std::str::FromStr;

pub async fn connect(database_url: &str) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let options = SqliteConnectOptions::from_str(database_url)?
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

Add to `src/lib.rs` if they are not already present:

```rust
pub mod database;
pub mod repository;
```

The `repository` module is created in the next step; create it before running the compiler. **Why is `connect` async?** Establishing database connections requires I/O. **Why doesn't the pool need an external `Mutex`?** It already coordinates checked-out connections internally, and cloned `SqlitePool` values refer to the same pool. **Why pass a URL?** A test can use its own temporary file instead of touching your real application data.

## 3. Repository: SQL rows become domain objects — Checkpoint B

We'll deliberately use a concrete `VehicleRepository` and `CustomerRepository`. Each owns a cheap cloneable pool handle. Mapping database rows into a domain type is *fallible*, so the return type distinguishes a failed database read from a valid row.

Add the following inside `impl Vehicle` in `src/vehicle.rs` (**ADD**, not a replacement):

```rust
pub fn status(&self) -> VehicleStatus {
    self.status
}

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
```

The first method provides a read-only way to observe operational status; the second preserves construction rules when rebuilding a vehicle from storage. It **does not** let the repository write private fields. Add this to `impl Customer` in `src/customer.rs` (**ADD**):

```rust
pub fn reconstitute(
    id: u32,
    name: String,
    email: String,
) -> Result<Self, CustomerError> {
    Self::new(id, name, email)
}
```

Create `src/repository.rs` (**FULL FILE**):

```rust
use crate::{
    customer::Customer,
    vehicle::{Vehicle, VehicleStatus},
};
use sqlx::{FromRow, SqlitePool};

type RepoResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn invalid_row(message: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, message.to_owned())
}

#[derive(FromRow)]
struct VehicleRow {
    id: i64,
    make: String,
    model: String,
    year: i64,
    daily_rate_cents: i64,
    status: String,
}

impl TryFrom<VehicleRow> for Vehicle {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(row: VehicleRow) -> RepoResult<Self> {
        let status = match row.status.as_str() {
            "ready" => VehicleStatus::Ready,
            "maintenance" => VehicleStatus::Maintenance,
            _ => return Err(invalid_row("unknown vehicle status").into()),
        };
        let id = u32::try_from(row.id)?;
        let year = u16::try_from(row.year)?;
        Vehicle::reconstitute(
            id,
            row.make,
            row.model,
            year,
            row.daily_rate_cents,
            status,
        )
        .map_err(|_| invalid_row("invalid vehicle data").into())
    }
}

pub struct VehicleRepository {
    pool: SqlitePool,
}

impl VehicleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn count(&self) -> RepoResult<usize> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vehicles")
            .fetch_one(&self.pool)
            .await?;
        Ok(usize::try_from(count)?)
    }

    pub async fn list(&self) -> RepoResult<Vec<Vehicle>> {
        let rows: Vec<VehicleRow> = sqlx::query_as(
            "SELECT id, make, model, year, daily_rate_cents, status \
             FROM vehicles ORDER BY id",
        )
        .fetch_all(&self.pool)
        .await?;
        rows.into_iter().map(Vehicle::try_from).collect()
    }

    pub async fn find(&self, id: u32) -> RepoResult<Option<Vehicle>> {
        let row: Option<VehicleRow> = sqlx::query_as(
            "SELECT id, make, model, year, daily_rate_cents, status \
             FROM vehicles WHERE id = ?",
        )
        .bind(i64::from(id))
        .fetch_optional(&self.pool)
        .await?;
        row.map(Vehicle::try_from).transpose()
    }

    pub async fn create(
        &self,
        make: String,
        model: String,
        year: u16,
        rate: i64,
    ) -> RepoResult<Vehicle> {
        let result = sqlx::query(
            "INSERT INTO vehicles (make, model, year, daily_rate_cents, status) \
             VALUES (?, ?, ?, ?, 'ready')",
        )
        .bind(&make)
        .bind(&model)
        .bind(i64::from(year))
        .bind(rate)
        .execute(&self.pool)
        .await?;
        let id = u32::try_from(result.last_insert_rowid())?;
        Ok(Vehicle::new(id, make, model, year, rate)
            .map_err(|_| invalid_row("invalid inserted vehicle"))?)
    }

    pub async fn change_rate(&self, id: u32, rate: i64) -> RepoResult<Option<Vehicle>> {
        let result = sqlx::query(
            "UPDATE vehicles SET daily_rate_cents = ? WHERE id = ?",
        )
        .bind(rate)
        .bind(i64::from(id))
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            return Ok(None);
        }
        self.find(id).await
    }

    pub async fn delete(&self, id: u32) -> RepoResult<bool> {
        let result = sqlx::query("DELETE FROM vehicles WHERE id = ?")
            .bind(i64::from(id))
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}

#[derive(FromRow)]
struct CustomerRow {
    id: i64,
    name: String,
    email: String,
}

fn customer_from_row(row: CustomerRow) -> RepoResult<Customer> {
    let id = u32::try_from(row.id)?;
    Customer::reconstitute(id, row.name, row.email)
        .map_err(|_| invalid_row("invalid customer data").into())
}

pub struct CustomerRepository {
    pool: SqlitePool,
}

impl CustomerRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn list(&self) -> RepoResult<Vec<Customer>> {
        let rows: Vec<CustomerRow> =
            sqlx::query_as("SELECT id, name, email FROM customers ORDER BY id")
                .fetch_all(&self.pool)
                .await?;
        rows.into_iter().map(customer_from_row).collect()
    }

    pub async fn create(&self, name: String, email: String) -> RepoResult<Customer> {
        let result = sqlx::query(
            "INSERT INTO customers (name, email) VALUES (?, ?)",
        )
        .bind(&name)
        .bind(&email)
        .execute(&self.pool)
        .await?;
        let id = u32::try_from(result.last_insert_rowid())?;
        Ok(Customer::new(id, name, email)
            .map_err(|_| invalid_row("invalid inserted customer"))?)
    }

    pub async fn email_exists(&self, email: &str) -> RepoResult<bool> {
        let found: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM customers WHERE email = ?")
                .bind(email)
                .fetch_optional(&self.pool)
                .await?;
        Ok(found.is_some())
    }
}
```

**What is Rust teaching here?** `FromRow` is a derive macro for decoding SQL columns; `TryFrom<VehicleRow>` is a fallible conversion into the *validated* domain. `?` propagates either SQL errors or integer conversion errors. `row.map(...).transpose()` turns `Option<Result<T, E>>` into `Result<Option<T>, E>`: a missing database record is not the same as an I/O failure.

**Intentional simplification:** `Box<dyn Error + Send + Sync>` permits several error sources without inventing an unintroduced error hierarchy. A later production design should use a typed repository error. These methods do not by themselves make the HTTP layer persistent; the next section wires that boundary.

## 4. Connect the pool to application state — Checkpoint C

**Replace** the Day 5 `AppState`. Previously it had `Arc<Mutex<Fleet>>` and `Arc<Mutex<CustomerStore>>`. It will now contain **only** a database pool. Remove the old `Arc`, `Mutex`, `Fleet` and `CustomerStore` imports from the API module. Don't attempt to access `state.fleet` or `state.customers` from new handlers—they no longer exist.

The persistent state becomes:

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}
```

Unlike `AppState::default()`, building this state can fail: connecting to SQLite requires I/O and migrations might be invalid. That work belongs in `main.rs` before serving HTTP. **Replace `src/main.rs` (FULL FILE):**

```rust
use rust_rent::{
    api::{router, AppState},
    database,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite://rust_rent.db".to_owned());
    let pool = database::connect(&database_url).await?;
    let app = router(AppState { pool });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("RustRent: http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
```

Why can `AppState` derive `Clone`? `SqlitePool::clone()` clones a shared pool handle; it doesn't copy the SQLite database or all database connections. Why isn't there a mutex? SQLx manages connections internally. This is the concrete answer to the Day 6 question “where do we use the connection?”

## 5. Replace the entire HTTP adapter (FULL FILE `src/api.rs`)

This listing preserves the **Day 5 endpoint contracts**: list/get/create vehicles, PATCH the rate, delete a vehicle, and list/create customers. All now use SQLite. This is a replacement, **not** code to paste below the old handlers.

```rust
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::{
    customer::Customer,
    repository::{CustomerRepository, VehicleRepository},
    vehicle::Vehicle,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

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

#[derive(Debug, Serialize)]
struct VehicleCount { count: usize }

impl From<&Vehicle> for VehicleResponse {
    fn from(vehicle: &Vehicle) -> Self {
        Self {
            id: vehicle.id(),
            make: vehicle.make().to_owned(),
            model: vehicle.model().to_owned(),
            year: vehicle.year(),
            daily_rate_cents: vehicle.daily_rate_cents(),
            rentable: vehicle.is_rentable(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChangeRate {
    daily_rate_cents: i64,
}

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

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .route("/vehicles/count", get(count_vehicles))
        .route(
            "/vehicles/{id}",
            get(get_vehicle).patch(change_rate).delete(delete_vehicle),
        )
        .route("/customers", get(list_customers).post(create_customer))
        .with_state(state)
}

async fn list_vehicles(
    State(state): State<AppState>,
) -> Result<Json<Vec<VehicleResponse>>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let vehicles = repository.list().await.map_err(internal_error)?;
    Ok(Json(vehicles.iter().map(VehicleResponse::from).collect()))
}

async fn count_vehicles(State(state): State<AppState>) -> Result<Json<VehicleCount>, StatusCode> {
    let count = VehicleRepository::new(state.pool.clone())
        .count()
        .await
        .map_err(internal_error)?;
    Ok(Json(VehicleCount { count }))
}

async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let vehicle = repository
        .find(id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(VehicleResponse::from(&vehicle)))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(input): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<VehicleResponse>), StatusCode> {
    // Validate using the domain model before attempting SQL.
    Vehicle::new(
        0,
        input.make.clone(),
        input.model.clone(),
        input.year,
        input.daily_rate_cents,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let repository = VehicleRepository::new(state.pool.clone());
    let vehicle = repository
        .create(
            input.make,
            input.model,
            input.year,
            input.daily_rate_cents,
        )
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(VehicleResponse::from(&vehicle))))
}

async fn change_rate(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(input): Json<ChangeRate>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let mut vehicle = repository
        .find(id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    vehicle
        .change_rate(input.daily_rate_cents)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let updated = repository
        .change_rate(id, vehicle.daily_rate_cents())
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(VehicleResponse::from(&updated)))
}

async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let deleted = repository.delete(id).await.map_err(delete_error_status)?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerResponse>>, StatusCode> {
    let repository = CustomerRepository::new(state.pool.clone());
    let customers = repository.list().await.map_err(internal_error)?;
    Ok(Json(customers.iter().map(CustomerResponse::from).collect()))
}

async fn create_customer(
    State(state): State<AppState>,
    Json(input): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<CustomerResponse>), StatusCode> {
    Customer::new(0, input.name.clone(), input.email.clone())
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let repository = CustomerRepository::new(state.pool.clone());
    // Database uniqueness protects against concurrent duplicate requests.
    let email_for_conflict_check = input.email.clone();
    match repository.create(input.name, input.email).await {
        Ok(customer) => Ok((
            StatusCode::CREATED,
            Json(CustomerResponse::from(&customer)),
        )),
        Err(error) => {
            // The simple boxed repository error hides the structured SQLx error.
            // This follow-up SELECT is sufficient for the local learning exercise,
            // but a typed repository error is preferable for real applications.
            if let Ok(true) = repository.email_exists(&email_for_conflict_check).await {
                return Err(StatusCode::CONFLICT);
            }
            eprintln!("Customer insertion failed: {error}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

fn delete_error_status(error: Box<dyn std::error::Error + Send + Sync>) -> StatusCode {
    if let Some(sqlx::Error::Database(db)) = error.downcast_ref::<sqlx::Error>()
        && db.is_foreign_key_violation()
    {
        return StatusCode::CONFLICT;
    }
    internal_error(error)
}

fn internal_error(error: Box<dyn std::error::Error + Send + Sync>) -> StatusCode {
    eprintln!("Database or row conversion error: {error}");
    StatusCode::INTERNAL_SERVER_ERROR
}
```

**Why this error mapping works:** The repository returns a boxed error, so `downcast_ref::<sqlx::Error>()` asks whether the underlying error is a SQLx error, without taking ownership. `if let Some(sqlx::Error::Database(db)) = ...` extracts only a database error; the `&&` condition additionally requires a foreign-key violation. SQLx provides `is_foreign_key_violation()` so we do not need to memorize SQLite-specific numeric codes. Other errors continue to map to HTTP 500.

**Why counting uses SQL:** `VehicleRepository::count()` returns the aggregate from `SELECT COUNT(*)` without fetching every vehicle row or constructing domain objects. The checked `usize::try_from` handles a theoretical count outside the Rust target type's range. The in-memory Day 4 and Day 5 handlers still count the fleet slice.

**Ownership note:** The email is cloned before moving fields into `repository.create`. This allows a lookup for the conflict branch. This fallback is educational, not a substitute for a typed uniqueness error; the optional extension improves it.

## 6. Prove persistence and test isolation — Checkpoint D

Create `tests/database.rs` (**FULL FILE**):

```rust
use rust_rent::{
    database,
    repository::{CustomerRepository, VehicleRepository},
};
use tempfile::tempdir;

#[tokio::test]
async fn vehicles_and_customers_survive_new_pool_connections() {
    let directory = tempdir().expect("temp directory");
    let path = directory.path().join("rent-test.db");
    let url = format!("sqlite://{}", path.display());

    let pool = database::connect(&url).await.expect("database and migrations");
    let vehicles = VehicleRepository::new(pool.clone());
    let customers = CustomerRepository::new(pool.clone());

    let created_vehicle = vehicles
        .create("Toyota".into(), "Camry".into(), 2025, 6500)
        .await
        .expect("vehicle insert");
    let created_customer = customers
        .create("Avery".into(), "avery@example.com".into())
        .await
        .expect("customer insert");

    assert_eq!(created_vehicle.make(), "Toyota");
    assert_eq!(created_customer.email(), "avery@example.com");
    pool.close().await;

    let reopened = database::connect(&url).await.expect("reopen database");
    let saved_vehicles = VehicleRepository::new(reopened.clone())
        .list()
        .await
        .expect("vehicle list");
    let saved_customers = CustomerRepository::new(reopened)
        .list()
        .await
        .expect("customer list");

    assert_eq!(saved_vehicles.len(), 1);
    assert_eq!(saved_customers.len(), 1);
}
```

**Why temporary directories?** A test should never delete or depend on `rust_rent.db`. The test creates its own SQLite file, applies migrations, inserts records, closes the original pool, opens another pool and verifies data still exists. This tests persistence independently of the development server. Note that this does not yet test every HTTP route; Day 7 adds integration and transaction tests.

Run:

```bash
cargo fmt
cargo check --all-targets
cargo test
cargo clippy --all-targets -- -D warnings
cargo run --bin rust-rent
```

In a second terminal, while the server runs:

```bash
curl -i -X POST http://127.0.0.1:3000/vehicles \
  -H 'Content-Type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'
curl -i -X POST http://127.0.0.1:3000/customers \
  -H 'Content-Type: application/json' \
  -d '{"name":"Avery","email":"avery@example.com"}'
curl -s http://127.0.0.1:3000/vehicles | jq
curl -s http://127.0.0.1:3000/customers | jq
```

Stop the server with Ctrl+C, restart it with `cargo run --bin rust-rent`, and repeat the two GET requests. **The records should still be there.** Do not repeat the same customer POST expecting another success: the schema makes email unique.

You can also inspect the file using `sqlite3 rust_rent.db 'SELECT * FROM vehicles;'`.

## 7. Exercise boundaries and what to understand

**Required:** Observe the **already-completed** Section 5 email ownership pattern: `email_for_conflict_check` is cloned before moving the input into the repository. No additional fix is needed. Verify persistence through a restart.

**Discovery A:** Why did we use `TryFrom<VehicleRow>` instead of making every SQL row field public in the domain? Predict what happens if `year` in SQLite exceeds `u16::MAX`.

**Discovery B:** `delete_vehicle` performs an irreversible database operation, while Day 5's `Fleet::remove` returned an owned `Vehicle`. Explain why SQL delete and vector removal can have different APIs.

**Optional extension:** Make customer duplication handling use a *typed* SQLx error, not the extra `email_exists` query. Keep unique email enforcement at the database level; checking the email before insert is insufficient under concurrent requests.

**Day 6 complete means:** all six vehicle operations (including `/vehicles/count`) and both customer operations from Day 5 use SQLite, a restart retains data, isolated persistence tests pass, and you can explain `SqlitePool`, `FromRow`, `TryFrom`, `?`, and why domain validation belongs in `Vehicle` and `Customer`.

**Evidence:** [SQLx pool](https://docs.rs/sqlx/0.9.0/sqlx/struct.Pool.html), [SQLx migrations](https://docs.rs/sqlx/0.9.0/sqlx/macro.migrate.html), [Axum state](https://docs.rs/axum/0.8/axum/extract/struct.State.html), [Rust `TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html). Check this code in your own project with the commands above before continuing.

---

# Day 7 — Build the reservation API: domain rules, atomic transactions and tests

**Outcome:** Extend the **working SQL-backed Day 6 API** with `POST /reservations`, `GET /reservations/{id}`, and `POST /reservations/{id}/cancel`. Learn why a reservation is different from a vehicle, how typed domain errors cross into HTTP, and why a database transaction is necessary to prevent double booking.

**Starting point:** The Day 6 `SqlitePool` is already created in `main.rs` and stored in `AppState { pool }`; vehicle and customer routes already use SQLx. Keep the Day 6 migrations and repository unchanged unless instructed below. The `reservations` table already exists; **do not create the table a second time**. This module reuses the same pool, not a new `Arc<Mutex<...>>`.

**Success contract:** A valid reservation returns 201, invalid dates 400, missing vehicle/customer 404, and overlapping dates 409. A vehicle manually marked `maintenance` in SQLite also produces 409; **there is no HTTP maintenance update route**. Cancellation returns 200 and releases the interval. Deleting a vehicle referenced by a reservation returns 409 via the Day 6 foreign-key mapping, **even when the reservation is cancelled**: cancellation changes its status but does not delete the database row or its foreign-key reference. Unreserved vehicles can still be deleted. The booking record survives a restart.

## 1. Why a reservation is not a vehicle status

A vehicle's `Ready` or `Maintenance` state describes whether it can operate. A booking answers **whether this vehicle is free between two dates**. One car may be reserved next week but available today. Never set a single `VehicleStatus::Rented` flag to represent bookings for all dates.

We treat dates as half-open intervals `[start_date, end_date)`. A booking from 1 November until 4 November is **three billed days**. A new booking starting on 4 November is allowed.

Two ranges overlap precisely when:

```rust
// ILLUSTRATION: predicate used in our domain and SQL
existing_start < requested_end && existing_end > requested_start
```

The predicate is a **business rule**, not something the Rust compiler invents. We adopt it to make back-to-back rentals possible.

## 2. Add dates and define a validated Reservation

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
    id: u32,
    vehicle_id: u32,
    customer_id: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total_cents: i64,
    status: ReservationStatus,
}

impl Reservation {
    pub fn calculate_total(
        daily_rate_cents: i64,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<i64, ReservationError> {
        if daily_rate_cents <= 0 {
            return Err(ReservationError::InvalidRate);
        }
        let days = (end_date - start_date).num_days();
        if days <= 0 {
            return Err(ReservationError::InvalidDateRange);
        }
        daily_rate_cents
            .checked_mul(days)
            .ok_or(ReservationError::PriceOverflow)
    }

    pub fn new(
        id: u32,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        daily_rate_cents: i64,
    ) -> Result<Self, ReservationError> {
        let total_cents =
            Self::calculate_total(daily_rate_cents, start_date, end_date)?;
        Ok(Self {
            id,
            vehicle_id,
            customer_id,
            start_date,
            end_date,
            total_cents,
            status: ReservationStatus::Confirmed,
        })
    }

    // Reconstitution accepts the stored total. Historical bookings retain
    // their original price even if the vehicle's current rate changes.
    pub fn from_storage(
        id: u32,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
        total_cents: i64,
        status: ReservationStatus,
    ) -> Result<Self, ReservationError> {
        if end_date <= start_date {
            return Err(ReservationError::InvalidDateRange);
        }
        if total_cents <= 0 {
            return Err(ReservationError::InvalidRate);
        }
        Ok(Self {
            id, vehicle_id, customer_id, start_date, end_date, total_cents, status,
        })
    }

    pub fn id(&self) -> u32 { self.id }
    pub fn vehicle_id(&self) -> u32 { self.vehicle_id }
    pub fn customer_id(&self) -> u32 { self.customer_id }
    pub fn start_date(&self) -> NaiveDate { self.start_date }
    pub fn end_date(&self) -> NaiveDate { self.end_date }
    pub fn total_cents(&self) -> i64 { self.total_cents }
    pub fn status(&self) -> ReservationStatus { self.status }

    pub fn overlaps(&self, start: NaiveDate, end: NaiveDate) -> bool {
        self.status == ReservationStatus::Confirmed
            && self.start_date < end
            && self.end_date > start
    }

    pub fn cancel(&mut self) {
        self.status = ReservationStatus::Cancelled;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn date(day: u32) -> NaiveDate {
        NaiveDate::from_ymd_opt(2026, 11, day).expect("test date")
    }

    #[test]
    fn price_and_adjacent_intervals() {
        let booking = Reservation::new(1, 1, 1, date(1), date(4), 6500)
            .expect("valid booking");
        assert_eq!(booking.total_cents(), 19500);
        assert!(!booking.overlaps(date(4), date(7)));
        assert!(booking.overlaps(date(3), date(5)));
    }

    #[test]
    fn invalid_ranges_rejected() {
        assert!(matches!(
            Reservation::new(1, 1, 1, date(4), date(4), 6500),
            Err(ReservationError::InvalidDateRange)
        ));
    }

    #[test]
    fn cancellation_removes_conflict() {
        let mut booking =
            Reservation::new(1, 1, 1, date(1), date(4), 6500).unwrap();
        booking.cancel();
        assert!(!booking.overlaps(date(2), date(3)));
    }
}
```

**Why private fields?** We kept `Vehicle` private to protect its daily-rate invariant. `Reservation` also has invariants: a positive price, ordered dates and controlled status changes. Public fields would bypass those rules. `from_storage` is a separate constructor because the historical total should not be recalculated using today's rate. Its storage validation is intentionally modest for this learning project.

Add `pub mod reservation;` and `pub mod reservation_service;` to `src/lib.rs`. Do so before the final compilation; the second module will be created next.

## 3. Introducing the service: a specific job, not a mystery abstraction

A vehicle's `impl` manages one vehicle's rules. A `ReservationService` coordinates a *vehicle row*, a *customer row*, *existing bookings* and *one insertion*. That coordination crosses entity boundaries, so it is a reasonable use of a service struct. It is **not** a Rust requirement to create services for every operation.

The service owns a cloned `SqlitePool` handle, just as Day 6's repositories do. Its public methods are `create`, `find` and `cancel`.

### Typed errors and `From`

The service can fail for expected business reasons (`NotFound`, `VehicleUnavailable`, `Overlap`, `InvalidDateRange`) or due to SQLx. We use a typed enum so HTTP can map distinct outcomes to status codes *without parsing strings*.

Create `src/reservation_service.rs` (**FULL FILE**):

```rust
use chrono::NaiveDate;
use sqlx::{FromRow, SqlitePool};

use crate::reservation::{
    Reservation, ReservationError, ReservationStatus,
};

#[derive(Debug)]
pub enum RentalError {
    VehicleNotFound,
    CustomerNotFound,
    ReservationNotFound,
    VehicleUnavailable,
    OverlappingReservation,
    InvalidDateRange,
    InvalidRate,
    PriceOverflow,
    Database(sqlx::Error),
    InvalidStoredData,
}

impl From<sqlx::Error> for RentalError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value)
    }
}

impl From<ReservationError> for RentalError {
    fn from(value: ReservationError) -> Self {
        match value {
            ReservationError::InvalidDateRange => Self::InvalidDateRange,
            ReservationError::InvalidRate => Self::InvalidRate,
            ReservationError::PriceOverflow => Self::PriceOverflow,
        }
    }
}

#[derive(FromRow)]
struct StoredReservation {
    id: i64,
    vehicle_id: i64,
    customer_id: i64,
    start_date: String,
    end_date: String,
    total_cents: i64,
    status: String,
}

fn decode(row: StoredReservation) -> Result<Reservation, RentalError> {
    let id = u32::try_from(row.id).map_err(|_| RentalError::InvalidStoredData)?;
    let vehicle_id =
        u32::try_from(row.vehicle_id).map_err(|_| RentalError::InvalidStoredData)?;
    let customer_id =
        u32::try_from(row.customer_id).map_err(|_| RentalError::InvalidStoredData)?;
    let start = NaiveDate::parse_from_str(&row.start_date, "%Y-%m-%d")
        .map_err(|_| RentalError::InvalidStoredData)?;
    let end = NaiveDate::parse_from_str(&row.end_date, "%Y-%m-%d")
        .map_err(|_| RentalError::InvalidStoredData)?;
    let status = match row.status.as_str() {
        "confirmed" => ReservationStatus::Confirmed,
        "cancelled" => ReservationStatus::Cancelled,
        _ => return Err(RentalError::InvalidStoredData),
    };
    Reservation::from_storage(
        id, vehicle_id, customer_id, start, end, row.total_cents, status,
    )
    .map_err(|_| RentalError::InvalidStoredData)
}

pub struct ReservationService {
    pool: SqlitePool,
}

impl ReservationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn find(&self, id: u32) -> Result<Option<Reservation>, RentalError> {
        let row: Option<StoredReservation> = sqlx::query_as(
            "SELECT id, vehicle_id, customer_id, start_date, end_date, \
             total_cents, status FROM reservations WHERE id = ?",
        )
        .bind(i64::from(id))
        .fetch_optional(&self.pool)
        .await?;
        row.map(decode).transpose()
    }

    pub async fn create(
        &self,
        vehicle_id: u32,
        customer_id: u32,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Reservation, RentalError> {
        if end_date <= start_date {
            return Err(RentalError::InvalidDateRange);
        }

        // BEGIN IMMEDIATE acquires SQLite's write reservation before checking
        // for conflicts. All subsequent queries use this SAME transaction.
        let mut tx = self.pool.begin_with("BEGIN IMMEDIATE").await?;

        let vehicle: Option<(i64, String)> = sqlx::query_as(
            "SELECT daily_rate_cents, status FROM vehicles WHERE id = ?",
        )
        .bind(i64::from(vehicle_id))
        .fetch_optional(&mut *tx)
        .await?;
        let (rate, status) = vehicle.ok_or(RentalError::VehicleNotFound)?;
        if status != "ready" {
            return Err(RentalError::VehicleUnavailable);
        }

        let customer: Option<(i64,)> =
            sqlx::query_as("SELECT id FROM customers WHERE id = ?")
                .bind(i64::from(customer_id))
                .fetch_optional(&mut *tx)
                .await?;
        if customer.is_none() {
            return Err(RentalError::CustomerNotFound);
        }

        let end = end_date.format("%Y-%m-%d").to_string();
        let start = start_date.format("%Y-%m-%d").to_string();
        let overlap: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM reservations \
             WHERE vehicle_id = ? AND status = 'confirmed' \
             AND start_date < ? AND end_date > ?",
        )
        .bind(i64::from(vehicle_id))
        .bind(&end)
        .bind(&start)
        .fetch_one(&mut *tx)
        .await?;
        if overlap.0 > 0 {
            return Err(RentalError::OverlappingReservation);
        }

        // Domain constructor checks rate, dates and overflow.
        let reservation =
            Reservation::new(0, vehicle_id, customer_id, start_date, end_date, rate)?;
        let inserted = sqlx::query(
            "INSERT INTO reservations \
             (vehicle_id, customer_id, start_date, end_date, total_cents, status) \
             VALUES (?, ?, ?, ?, ?, 'confirmed')",
        )
        .bind(i64::from(vehicle_id))
        .bind(i64::from(customer_id))
        .bind(&start)
        .bind(&end)
        .bind(reservation.total_cents())
        .execute(&mut *tx)
        .await?;
        let id = u32::try_from(inserted.last_insert_rowid())
            .map_err(|_| RentalError::InvalidStoredData)?;

        tx.commit().await?;
        Reservation::new(id, vehicle_id, customer_id, start_date, end_date, rate)
            .map_err(RentalError::from)
    }

    pub async fn cancel(&self, id: u32) -> Result<Reservation, RentalError> {
        // Idempotent behavior: a cancelled reservation stays cancelled.
        let result = sqlx::query(
            "UPDATE reservations SET status = 'cancelled' WHERE id = ?",
        )
        .bind(i64::from(id))
        .execute(&self.pool)
        .await?;
        if result.rows_affected() == 0 {
            return Err(RentalError::ReservationNotFound);
        }
        self.find(id)
            .await?
            .ok_or(RentalError::ReservationNotFound)
    }
}
```

### Why is `?` legal for SQLx and domain errors?

This service returns `Result<_, RentalError>`. `impl From<sqlx::Error> for RentalError` tells Rust how to turn SQLx failures into this error type. `impl From<ReservationError>` does the same for validated domain construction. Thus `?` can propagate either kind without a manual `match` at each call.

`decode` is intentionally a *fallible* conversion of an untrusted storage row into a domain object. Its `row.map(decode).transpose()` means: no row → `Ok(None)`, valid row → `Ok(Some(...))`, invalid row → `Err(...)`.

## 4. Why the transaction must include both check and insert

A naïve algorithm checks availability on one connection, then inserts later. Two concurrent requests might both see the vehicle as free. SQLite permits only one writer at a time. Beginning a `BEGIN IMMEDIATE` transaction *before reading availability* serializes competing booking writers for this **local single-database** application. Both the overlap query and insert use `&mut *tx`; they must not independently use `&self.pool`.

A SQLx `Transaction` rolls back when dropped without `commit`, including ordinary early `?` returns. This does **not** guarantee every cancellation, disk-full or driver edge case is handled perfectly. Keep transactions short; do not perform unrelated network operations while holding one. Binding variables with `.bind` avoids SQL injection and aligns with SQLx 0.9's literal SQL-string requirements. See [SQLx Pool](https://docs.rs/sqlx/0.9.0/sqlx/struct.Pool.html), [SQLx Transaction](https://docs.rs/sqlx/0.9.0/sqlx/struct.Transaction.html), and [SQLite transaction documentation](https://www.sqlite.org/lang_transaction.html).

**Deliberate limitation:** The `cancel` method performs a single UPDATE and a subsequent SELECT rather than returning a row within one transaction. This is acceptable as a beginner demonstration of cancellation, but a stronger version would put both operations under the same transaction if a consistent snapshot is required. It also allows cancellation after the scheduled return date; real systems may need a richer lifecycle.

## 5. Connect the service to the existing Axum API

We reuse Day 6's `AppState { pool }`; **do not add another field or pool**. Day 6's `main.rs` and `database.rs` remain unchanged. Add the following to `src/api.rs` and update the router as directed.

**ADD** these imports:

```rust
use chrono::NaiveDate;
use crate::reservation::{Reservation, ReservationStatus};
use crate::reservation_service::{RentalError, ReservationService};
```

**ADD** response/request representations and a boundary conversion (outside existing functions):

```rust
#[derive(Debug, Deserialize)]
struct CreateReservation {
    vehicle_id: u32,
    customer_id: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

#[derive(Debug, Serialize)]
struct ReservationResponse {
    id: u32,
    vehicle_id: u32,
    customer_id: u32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    total_cents: i64,
    status: &'static str,
}

impl From<&Reservation> for ReservationResponse {
    fn from(reservation: &Reservation) -> Self {
        Self {
            id: reservation.id(),
            vehicle_id: reservation.vehicle_id(),
            customer_id: reservation.customer_id(),
            start_date: reservation.start_date(),
            end_date: reservation.end_date(),
            total_cents: reservation.total_cents(),
            status: match reservation.status() {
                ReservationStatus::Confirmed => "confirmed",
                ReservationStatus::Cancelled => "cancelled",
            },
        }
    }
}

fn rental_error_status(error: RentalError) -> StatusCode {
    match error {
        RentalError::VehicleNotFound
        | RentalError::CustomerNotFound
        | RentalError::ReservationNotFound => StatusCode::NOT_FOUND,
        RentalError::InvalidDateRange | RentalError::InvalidRate
        | RentalError::PriceOverflow => StatusCode::BAD_REQUEST,
        RentalError::VehicleUnavailable
        | RentalError::OverlappingReservation => StatusCode::CONFLICT,
        RentalError::Database(error) => {
            eprintln!("Reservation database error: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        RentalError::InvalidStoredData => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
```

Notice: domain errors **do not contain HTTP status codes**. This mapping belongs at the HTTP boundary. In this introductory version the error response is a bare status; structured error JSON is a useful follow-on exercise.

**ADD** handlers to `src/api.rs`:

```rust
async fn create_reservation(
    State(state): State<AppState>,
    Json(input): Json<CreateReservation>,
) -> Result<(StatusCode, Json<ReservationResponse>), StatusCode> {
    let service = ReservationService::new(state.pool.clone());
    let reservation = service
        .create(input.vehicle_id, input.customer_id, input.start_date, input.end_date)
        .await
        .map_err(rental_error_status)?;
    Ok((StatusCode::CREATED, Json(ReservationResponse::from(&reservation))))
}

async fn get_reservation(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ReservationResponse>, StatusCode> {
    let service = ReservationService::new(state.pool.clone());
    let reservation = service
        .find(id)
        .await
        .map_err(rental_error_status)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(ReservationResponse::from(&reservation)))
}

async fn cancel_reservation(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ReservationResponse>, StatusCode> {
    let service = ReservationService::new(state.pool.clone());
    let reservation = service.cancel(id).await.map_err(rental_error_status)?;
    Ok(Json(ReservationResponse::from(&reservation)))
}
```

**ADD** these routes in `router` before `.with_state(state)`:

```rust
.route("/reservations", axum::routing::post(create_reservation))
.route("/reservations/{id}", get(get_reservation))
.route("/reservations/{id}/cancel", axum::routing::post(cancel_reservation))
```

**Checkpoint:** `cargo fmt && cargo check --all-targets && cargo test`. Do not add reservation SQL to `Vehicle`, and do not modify the Day 6 pool startup: you have reused the existing application state.

## 6. Test the full journey over HTTP

Start the server: `cargo run --bin rust-rent`. Use a **new** `DATABASE_URL` or a fresh development database to avoid collisions with prior test records.

Create vehicle and customer (Day 6 requests), noting their returned IDs:

```bash
curl -i -X POST http://127.0.0.1:3000/vehicles \
  -H 'Content-Type: application/json' \
  -d '{"make":"Toyota","model":"Camry","year":2025,"daily_rate_cents":6500}'

curl -i -X POST http://127.0.0.1:3000/customers \
  -H 'Content-Type: application/json' \
  -d '{"name":"Avery","email":"avery@example.com"}'
```

For the following samples, substitute the IDs returned by those two calls if they are not `1`. **Create a booking**:

```bash
curl -i -X POST http://127.0.0.1:3000/reservations \
  -H 'Content-Type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-01","end_date":"2026-11-04"}'
```

Expected: `201 Created`, `total_cents: 19500`, `status: "confirmed"`.

Attempt overlap (expect `409 Conflict`), then adjacent booking (expect `201 Created`):

```bash
curl -i -X POST http://127.0.0.1:3000/reservations \
  -H 'Content-Type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-02","end_date":"2026-11-05"}'

curl -i -X POST http://127.0.0.1:3000/reservations \
  -H 'Content-Type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-11-04","end_date":"2026-11-07"}'
```

Cancel the original booking (substitute original reservation ID). Repeat the original dates: they should now be available.

```bash
curl -i -X POST http://127.0.0.1:3000/reservations/1/cancel
curl -i http://127.0.0.1:3000/reservations/1
```

Stop and restart the server. GET the reservation again; status should remain `cancelled`. Invalid range (same start/end) returns explicit `400`; an unknown vehicle returns `404`. Malformed JSON or wrong content type may receive Axum extractor rejections instead of our domain status body.

## 7. Automated service tests with an isolated SQLite database

Tests must not delete or share your development database. Day 6 already added `tempfile` as a dev dependency and `database::connect(database_url)`.

Create `tests/reservations.rs` (**FULL FILE**):

```rust
use chrono::NaiveDate;
use rust_rent::{
    database,
    repository::{CustomerRepository, VehicleRepository},
    reservation_service::{RentalError, ReservationService},
};
use tempfile::tempdir;

fn date(day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, day).expect("valid date")
}

#[tokio::test]
async fn create_overlap_adjacent_cancel_and_persist() {
    let directory = tempdir().unwrap();
    let url = format!("sqlite://{}", directory.path().join("test.db").display());
    let pool = database::connect(&url).await.unwrap();
    let vehicle = VehicleRepository::new(pool.clone())
        .create("Toyota".into(), "Camry".into(), 2025, 6500)
        .await.unwrap();
    let customer = CustomerRepository::new(pool.clone())
        .create("Avery".into(), "avery@example.com".into())
        .await.unwrap();
    let service = ReservationService::new(pool.clone());

    let first = service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await.unwrap();
    assert_eq!(first.total_cents(), 19500);

    let overlap = service
        .create(vehicle.id(), customer.id(), date(2), date(5))
        .await;
    assert!(matches!(overlap, Err(RentalError::OverlappingReservation)));

    let adjacent = service
        .create(vehicle.id(), customer.id(), date(4), date(7))
        .await.unwrap();
    assert_ne!(first.id(), adjacent.id());

    let cancelled = service.cancel(first.id()).await.unwrap();
    assert_eq!(cancelled.status(), rust_rent::reservation::ReservationStatus::Cancelled);
    let replacement = service
        .create(vehicle.id(), customer.id(), date(1), date(4))
        .await.unwrap();
    assert_ne!(replacement.id(), first.id());

    pool.close().await;
    let reopened = database::connect(&url).await.unwrap();
    let persisted = ReservationService::new(reopened)
        .find(replacement.id()).await.unwrap().expect("persisted booking");
    assert_eq!(persisted.total_cents(), 19500);
}

#[tokio::test]
async fn simultaneous_conflicting_requests_cannot_both_commit() {
    let directory = tempdir().unwrap();
    let url = format!("sqlite://{}", directory.path().join("concurrent.db").display());
    let pool = database::connect(&url).await.unwrap();
    let vehicle = VehicleRepository::new(pool.clone())
        .create("Honda".into(), "Civic".into(), 2025, 5000).await.unwrap();
    let customer = CustomerRepository::new(pool.clone())
        .create("Taylor".into(), "taylor@example.com".into()).await.unwrap();

    let a = ReservationService::new(pool.clone());
    let b = ReservationService::new(pool.clone());
    let (left, right) = tokio::join!(
        a.create(vehicle.id(), customer.id(), date(1), date(4)),
        b.create(vehicle.id(), customer.id(), date(1), date(4))
    );
    let results = [left, right];
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(results.iter().filter(|result| matches!(result, Err(RentalError::OverlappingReservation))).count(), 1, "the loser must be a domain overlap, not a database failure");
}
```

**Important distinction:** These are **service integration tests** that exercise real SQLite transactions, but not Axum HTTP routing. The `curl` checks above test the routes manually. A follow-on automated HTTP test using Axum's `Router` and `tower::ServiceExt` would be valuable and is not silently claimed here.

## 8. Discovery exercises — no unspecified future services

**Exercise A (required, given starting point):** In `tests/reservations.rs`, add a test that attempts a rental where `start_date == end_date`. Starting point: create a temporary pool, vehicle, customer and service as above. Expected: `Err(RentalError::InvalidDateRange)` and no inserted row. **Hint:** the validation precedes `begin_with`. Use `sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM reservations").fetch_one(&pool).await` and assert the count is zero.

**Exercise B (required):** Add a test in which `Vehicle::put_into_maintenance()` conceptually makes a car unavailable, but in SQLite you must update the vehicle row to `status = 'maintenance'` through a parameter-bound `sqlx::query` before creating the reservation. Expected: `Err(RentalError::VehicleUnavailable)`. Explain the actual tradeoff: this reservation service compares the raw persisted status string, rather than calling `Vehicle::is_rentable()`. How could reconstructing a domain `Vehicle` remove this duplication?

**Exercise C (optional):** Add structured JSON errors to Axum by implementing `IntoResponse` on a transport `ApiError`. Map `RentalError` variants to 400/404/409/500 without moving HTTP status codes into the domain. Keep Axum's JSON-extraction errors separate unless you deliberately customize their rejection handling.

**Exercise D (optional):** Add an HTTP integration test and verify response status codes using `tower::ServiceExt::oneshot`. This extends, rather than replaces, the supplied SQLite transaction test.

**Done means:** You can explain why reservation availability is date-dependent, why `From<sqlx::Error>` enables `?`, why the transaction must acquire the write lock before checking overlap, how `AppState.pool` is reused, and why stored totals cannot be blindly recalculated. **First run `cargo fmt` after creating `tests/reservations.rs`.** Then run `cargo fmt --check`, `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`. Complete these commands locally before marking the lesson finished.

**Evidence and limitations:** [Rust error conversions](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html), [SQLx Pool::begin_with](https://docs.rs/sqlx/0.9.0/sqlx/struct.Pool.html), [SQLite transactions](https://www.sqlite.org/lang_transaction.html), [Chrono NaiveDate](https://docs.rs/chrono/0.4/chrono/struct.NaiveDate.html), [Axum state](https://docs.rs/axum/0.8/axum/extract/struct.State.html). This is a local learning implementation, not a production booking system; its concurrency guarantees are limited to the SQLite transaction design described above.

---

## Independent final challenge

Add `GET /customers/{id}/reservations`, listing confirmed and cancelled bookings for a customer. **Start:** Day 7's `AppState.pool`, row decoder, reservation service and response type. **Goal:** add a parameter-bound query, service method, handler, route and a test. **Verify:** a customer with two bookings gets two records; a known customer with none gets an empty array; an unknown customer gets 404. **Hint:** `fetch_all`, `map(decode)` and `collect::<Result<Vec<_>, _>>()`. This is an optional transfer-of-learning challenge, not a prerequisite.

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


## Sources and design decisions

| Claim or decision | Evidence | Course interpretation |
|---|---|---|
| Modules are private by default; `pub` exposes items | [Rust Book: modules and paths](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html) | Explain separate visibility of module, type, methods and fields |
| Ownership, borrowing, and references | [Rust Book: ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html), [references](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) | Teach by predicting compiler errors before moving values |
| `Option`, `Result` and `?` | [Rust Book: recoverable errors](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html) | Convert absence to an error only when the operation requires a record |
| Integration tests live in `tests/` | [Rust Book: test organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html) | Exercise exact `lib.rs` exports, not just unconnected snippets |
| Axum `Json` must be last extractor | [Axum extractor documentation](https://docs.rs/axum/0.8/axum/extract/index.html#the-order-of-extractors) | Body-consuming extractor comes after `State` or `Path` |
| SQLx supports pool, migrations, prepared bound queries | [SQLx 0.9 reference](https://docs.rs/sqlx/0.9.0/sqlx/) | Use SQL literals and bind input values |
| SQLite transaction / SQLx `begin_with` | [SQLx Pool API](https://docs.rs/sqlx/0.9.0/sqlx/struct.Pool.html) | Atomic check-and-insert for this local SQLite service; do not promise perfect cancellation safety |
| Domain/service/repository layering | **Teaching design choice**, not a rule imposed by Rust | Avoid claiming it is the single idiomatic architecture |
| Half-open booking ranges | **Course business rule** | Allows a booking to start on the prior booking's end day; not a language guarantee |

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

# Appendix D — Project scope and next steps

RustRent is a local teaching application. It deliberately leaves out authentication, payments, deployments, observability and distributed reservation coordination. Its introductory email validation is not production-grade, and database errors are simplified to keep the focus on Rust, SQLx and HTTP.

You can strengthen the project after finishing Day 7: automate HTTP route tests, replace boxed repository errors with a typed error enum, introduce configuration for environments, and improve the database model for real rental operations. Add features incrementally, running `cargo fmt`, `cargo check --all-targets`, `cargo clippy --all-targets -- -D warnings`, and `cargo test` after each change.

To revisit a difficult lesson, retain your end-of-day source snapshots or commits. That lets you compare your code before and after the introduction of ownership, HTTP state, or SQLite without risking your completed project.
