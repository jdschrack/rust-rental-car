#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VehicleStatus {
    Ready,
    Maintenance,
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

#[derive(Debug, PartialEq, Eq)]
pub enum VehicleError {
    EmptyMake,
    EmptyModel,
    InvalidRate,
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

    pub fn display(&self) -> String {
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

#[cfg(test)]
mod VehicleTests {
    use std::assert_matches;

    use super::{Vehicle, VehicleError};

    #[test]
    fn add_car() {
        if let Ok(car) = Vehicle::new(1, String::from("Toyota"), String::from("Camry"), 2025, 6500)
        {
            assert_eq!(car.display(), "#1 Toyota Camry (2025), $65, Ready");
        }
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
