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

    fn mark_ready(&self) -> bool {
        self.status == VehicleStatus::Ready
    }

    fn put_into_maintenance(&mut self) {
        self.status = VehicleStatus::Maintenance
    }

    fn display(&self) -> String {
        format!(
            "#{} {} {} ({}), ${:.2}, {:?}",
            self.id,
            self.make,
            self.model,
            self.year,
            (self.daily_rate_cents / 100),
            self.status
        )
    }
}

fn main() {
    let mut car = match Vehicle::new(1, String::from("Toyota"), String::from("Camry"), 2025, 6500) {
        Ok(car) => car,
        Err(error) => {
            eprintln!("Could not create vehicle: {error:?}");
            return;
        }
    };

    println!("{}", car.display());
    println!("Rentable: {}", car.mark_ready());
    car.put_into_maintenance();
    println!("{}", car.display());
    println!("Rentable: {}", car.mark_ready());
    car.mark_ready();
    println!("{}", car.display());
}

#[cfg(test)]
mod VehicleTests {
    use std::assert_matches;

    use super::{Vehicle, VehicleError, VehicleStatus};

    #[test]
    fn add_car() {
        match Vehicle::new(1, String::from("Toyota"), String::from("Camry"), 2025, 6500) {
            Ok(car) => {
                assert_eq!(car.display(), "#1 Toyota Camry (2025), $65, Ready")
            }
            Err(_) => {}
        };
    }

    #[test]
    fn add_car_with_no_make() {
        let result = Vehicle::new(1, String::from(""), String::from("Camry"), 2025, 6500);
        assert_matches!(result, Err(VehicleError::EmptyMake));
    }

    #[test]
    fn add_car_with_no_model() {
        let result = Vehicle::new(1, String::from("Toyota"), String::from(""), 2025, 6500);
        assert_matches!(result, Err(VehicleError::EmptyModel));
    }

    #[test]
    fn add_car_with_negative_price() {
        let result = Vehicle::new(
            1,
            String::from("Toyota"),
            String::from("Camry"),
            2025,
            -6500,
        );
        assert_matches!(result, Err(VehicleError::InvalidRate));
    }
}
