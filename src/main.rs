use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VehicleStatus {
    Available,
    Rented,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct Vehicle {
    pub id: u32,
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
    pub status: VehicleStatus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateVehicle {
    pub make: String,
    pub model: String,
    pub year: u16,
    pub daily_rate_cents: i64,
}

pub type VehicleStore = Arc<Mutex<Vec<Vehicle>>>;

#[derive(Clone)]
pub struct AppState {
    pub vehicles: VehicleStore,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        vehicles: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind server");

    axum::serve(listener, app).await.expect("server failed");
}

async fn health() -> &'static str {
    "OK"
}

async fn list_vehicles(State(state): State<AppState>) -> Result<Json<Vec<Vehicle>>, StatusCode> {
    let vehicles = state
        .vehicles
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(vehicles.clone()))
}

async fn create_vehicle(
    State(state): State<AppState>,
    Json(payload): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<Vehicle>), StatusCode> {
    if payload.make.trim().is_empty()
        || payload.model.trim().is_empty()
        || payload.daily_rate_cents <= 0
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut vehicles = state
        .vehicles
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let next_id = vehicles.iter().map(|v| v.id).max().unwrap_or(0) + 1;

    let vehicle = Vehicle {
        id: next_id,
        make: payload.make,
        model: payload.model,
        year: payload.year,
        daily_rate_cents: payload.daily_rate_cents,
        status: VehicleStatus::Available,
    };

    vehicles.push(vehicle.clone());

    Ok((StatusCode::CREATED, Json(vehicle)))
}
