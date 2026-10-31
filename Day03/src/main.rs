mod customer;
mod fleet;
mod vehicle;

use customer::*;
use fleet::{Fleet, FleetError};
use vehicle::Vehicle;

fn main() {
    let mut fleet = Fleet::new();

    fleet.add(
        Vehicle::new(1, "Toyota".into(), "Camry".into(), 2025, 6500)
            .expect("seed vehicle is valid"),
    );
    fleet.add(
        Vehicle::new(2, "Jeep".into(), "Wrangler".into(), 2026, 14000)
            .expect("seed vehicle is valid"),
    );

    for car in fleet.available() {
        car.display();
    }

    match fleet.find_by_id(2) {
        Some(car) => println!("Found {} {}", car.make(), car.model()),
        None => println!("Not found"),
    }

    for (id, name, email) in [
        (1, "Avery", "avery@example.com"),
        (2, "Jordan", "jordan@example.com"),
        (3, "Invalid", "no-at-sign"),
    ] {
        match Customer::new(id, name.into(), email.into()) {
            Ok(c) => println!("Customer #{}: {} ({})", c.id(), c.name(), c.email()),
            Err(e) => println!("Customer rejected: {e:?}"),
        }
    }
}
