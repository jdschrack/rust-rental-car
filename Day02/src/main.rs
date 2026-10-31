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
        None => println!("Not found"),
    }

    let in_budget = fleet.affordable(9000);

    for car in in_budget {
        println!("{} {} {} is in range", car.id(), car.make(), car.model());
    }
}
