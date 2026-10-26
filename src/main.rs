use validator::{Validate, ValidationErrors};
mod utilities;

fn main() {
    let mut car = Vehicle::new(1, "Toyota".to_string(), "Camry".to_string(), 2025, 6500).unwrap();

    car.display();

    match car.rent() {
        Ok(()) => println!("Veicle rented successfully"),
        Err(VehicleError) => println!("Vehicle is unavilable"),
        Err(VehicleError::NotRented) => (),
    }

    car.display();


    let mut car_list: Vec<Vehicle> = Vec::new();

    match Vehicle::new(7, "Toyota".to_string(), "Corolla".to_string(), 2027, -5000) {
        Ok(car2) => {
            car2.display(); 
            car_list.push(car2);
        },
        Err(e) => println!("Could not create vehicle: {e}"),
    }


    car_list.push(car);

    car_list.push(Vehicle::new(
        2,
        "Ram".to_string(),
        "1500".to_string(),
        2026,
        12000,
    ).unwrap());
    car_list.push(Vehicle::new(
        3,
        "Volkswagen".to_string(),
        "Atlas".to_string(),
        2026,
        9500,
    ).unwrap());
    car_list.push(Vehicle::new(
        4,
        "Jeep".to_string(),
        "Wrangler".to_string(),
        2027,
        14000,
    ).unwrap());

    list_available_cars(&car_list);
}

#[derive(Debug, Clone, PartialEq)]
enum VehicleStatus {
    Available,
    Rented,
    Maintenance,
}

#[derive(Debug, PartialEq)]
enum VehicleError {
    NotAvailable,
    NotRented,
}

#[derive(Debug, Validate, Clone)]
struct Vehicle {
    id: u32,
    make: String,
    model: String,
    year: u16,
    #[validate(custom(function = "utilities::positive_int::validate_positive_number"))]
    daily_rate_cents: i64,
    status: VehicleStatus,
}

impl Vehicle {
    fn new(id: u32, make: String, model: String, year: u16, daily_rate_cents: i64) -> Result<Self, ValidationErrors> {
        let vehicle = Self {
            id,
            make,
            model,
            year,
            daily_rate_cents,
            status: VehicleStatus::Available,
        };

        vehicle.validate();
        Ok(vehicle)
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
        )
    }

    fn rent(&mut self) -> Result<(), VehicleError> {
        if !self.is_available() {
            return Err(VehicleError::NotAvailable);
        }

        self.status = VehicleStatus::Rented;
        Ok(())
    }

    fn return_vehicle(&mut self) -> Result<(), VehicleError> {
        if self.is_available() {
            return Err(VehicleError::NotRented);
        }

        self.status = VehicleStatus::Available;
        Ok(())
    }

    fn send_to_maintenance(&mut self) -> Result<(), VehicleError> {
        if !self.is_available() {
            return Err(VehicleError::NotAvailable);
        }

        self.status = VehicleStatus::Maintenance;
        Ok(())
    }
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
