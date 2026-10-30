use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    fn from(v: &Vehicle) -> Self {
        Self {
            id: v.id(),
            make: v.make().to_owned(),
            model: v.model().to_owned(),
            year: v.year(),
            daily_rate_cents: v.daily_rate_cents(),
            rentable: v.is_rentable(),
        }
    }
}

#[derive(Clone, Default)]
pub struct AppState {
    pub fleet: Arc<Mutex<Fleet>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "Ok" }))
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
    let items = fleet.all().iter().map(VehicleResponse::from).collect();
    Ok(Json(items))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(input): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<VehicleResponse>), StatusCode> {
    let mut fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let next_id = fleet
        .all()
        .iter()
        .map(|v| v.id())
        .max()
        .unwrap_or(0)
        .checked_add(1)
        .ok_or(StatusCode::BAD_REQUEST)?;
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
