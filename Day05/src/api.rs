use axum::{Json, Router, extract::Path, extract::State, http::StatusCode, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::customer::Customer;
use crate::customers::CustomerStore;
use crate::fleet::Fleet;
use crate::vehicle::Vehicle;

#[derive(Debug, Deserialize)]
struct CreateCustomer {
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct CustomerResponse {
    id: u32,
    name: String,
    email: String,
}

impl From<&Customer> for CustomerResponse {
    fn from(customer: &Customer) -> Self {
        Self {
            id: customer.id(),
            name: customer.name().to_owned(),
            email: customer.email().to_owned(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ChangeRate {
    daily_rate_cents: i64,
}

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
    pub customers: Arc<Mutex<CustomerStore>>,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "Ok" }))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .route(
            "/vehicles/{id}",
            get(get_vehicle).patch(change_rate).delete(delete_vehicle),
        )
        .route("/customers", get(list_customers).post(create_customer))
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

async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let vehicle = fleet.find(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(VehicleResponse::from(vehicle)))
}

async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let mut fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    fleet.remove(id).ok_or(StatusCode::NOT_FOUND)?;
    Ok(StatusCode::NO_CONTENT)
}

async fn change_rate(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(input): Json<ChangeRate>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let mut fleet = state
        .fleet
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let vehicle = fleet.find_mut(id).ok_or(StatusCode::NOT_FOUND)?;

    vehicle
        .change_rate(input.daily_rate_cents)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(Json(VehicleResponse::from(&*vehicle)))
}

async fn list_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerResponse>>, StatusCode> {
    let customers = state
        .customers
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let responses = customers.all().iter().map(CustomerResponse::from).collect();
    Ok(Json(responses))
}

async fn create_customer(
    State(state): State<AppState>,
    Json(input): Json<CreateCustomer>,
) -> Result<(StatusCode, Json<CustomerResponse>), StatusCode> {
    let mut store = state
        .customers
        .lock()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if store.has_email(&input.email) {
        return Err(StatusCode::CONFLICT);
    }

    let id = store.next_id().ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    let customer =
        Customer::new(id, input.name, input.email).map_err(|_| StatusCode::BAD_REQUEST)?;
    let response = CustomerResponse::from(&customer);
    store.add(customer);
    Ok((StatusCode::CREATED, Json(response)))
}
