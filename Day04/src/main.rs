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

    let car2 = match fleet.require(5) {
        Ok(car2) => car2,
        Err(FleetError::VehicleNotFound) => {
            println!("Vehicle with id {} not found", 5);
            return;
        }
    };

    println!("{}", car2.display());

    let customer = match Customer::new(1, "Jeremey Schrack".into(), "jdschrack@gmail.com".into()) {
        Ok(customer) => customer,
        Err(CustomerError::EmptyName) => {
            println!("Invalid name");
            return;
        }
        Err(CustomerError::InvalidEmail) => {
            println!("Invalid email");
            return;
        }
    };

    println!("{} {}", customer.name(), customer.email());
}
