use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::models::vehicle;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VehicleStatus {
    Available,
    Rented,
    Maintenance,
}

#[derive(Debug, PartialEq)]
pub enum VehicleError {
    NotAvailable,
    NotRented,
}

#[derive(Debug, Validate, Clone)]
pub struct Vehicle {
    pub id: u32,
    pub make: String,
    pub model: String,
    pub year: u16,
    #[validate(custom(function = "crate::utilities::positive_int::validate_positive_number"))]
    pub daily_rate_cents: i64,
    pub status: VehicleStatus,
}

impl Vehicle {
    pub fn new(
        id: u32,
        make: String,
        model: String,
        year: u16,
        daily_rate_cents: i64,
    ) -> Result<Self, ValidationErrors> {
        let vehicle = Self {
            id,
            make,
            model,
            year,
            daily_rate_cents,
            status: VehicleStatus::Available,
        };

        match vehicle.validate() {
            Ok(()) => Ok(vehicle),
            Err(err) => Err(err),
        }
    }

    pub fn is_available(&self) -> bool {
        self.status == vehicle::VehicleStatus::Available
    }

    pub fn display(&self) {
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

    pub fn rent(&mut self) -> Result<(), VehicleError> {
        if !self.is_available() {
            return Err(VehicleError::NotAvailable);
        }

        self.status = VehicleStatus::Rented;
        Ok(())
    }

    pub fn return_vehicle(&mut self) -> Result<(), VehicleError> {
        if self.is_available() {
            return Err(VehicleError::NotRented);
        }

        self.status = VehicleStatus::Available;
        Ok(())
    }

    pub fn maintenance(&mut self) -> Result<(), VehicleError> {
        if !self.is_available() {
            return Err(VehicleError::NotAvailable);
        }

        self.status = VehicleStatus::Maintenance;
        Ok(())
    }
}
