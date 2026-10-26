use rental_car::models::vehicle::{Vehicle, VehicleError};

fn main() {
    let mut car = Vehicle::new(1, "Toyota".to_string(), "Camry".to_string(), 2025, 6500).unwrap();

    car.display();

    match car.rent() {
        Ok(()) => println!("Vehicle rented successfully"),
        Err(VehicleError::NotAvailable) => println!("Vehicle is unavailable"),
        Err(VehicleError::NotRented) => (),
    }

    car.display();

    let mut car_list: Vec<Vehicle> = Vec::new();

    match Vehicle::new(7, "Toyota".to_string(), "Corolla".to_string(), 2027, -5000) {
        Ok(car2) => {
            car2.display();
            car_list.push(car2);
        }
        Err(e) => println!("Could not create vehicle: {e}"),
    }

    car_list.push(car);

    car_list.push(Vehicle::new(2, "Ram".to_string(), "1500".to_string(), 2026, 12000).unwrap());
    car_list
        .push(Vehicle::new(3, "Volkswagen".to_string(), "Atlas".to_string(), 2026, 9500).unwrap());
    car_list
        .push(Vehicle::new(4, "Jeep".to_string(), "Wrangler".to_string(), 2027, 14000).unwrap());

    list_available_cars(&car_list);
}

fn print_vehicle_name(vehicle: &Vehicle) {
    println!("{} {}", vehicle.make, vehicle.model)
}

fn list_available_cars(vehicle_list: &[Vehicle]) {
    vehicle_list
        .iter()
        .filter(|car| car.is_available())
        .for_each(print_vehicle_name);
}
