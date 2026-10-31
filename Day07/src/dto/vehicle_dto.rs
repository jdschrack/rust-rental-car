use crate::models::vehicle::Vehicle;
use serde::{Deserialize, Serialize};

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
pub struct VehicleCount {
    pub count: usize,
}

#[derive(Debug, Deserialize)]
pub struct VehicleRateChange {
    pub daily_rate_cents: i64,
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
