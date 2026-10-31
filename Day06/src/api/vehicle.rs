use crate::{
    api::router::{AppState, delete_error_status, internal_error},
    dto::vehicle_dto::*,
    models::vehicle::Vehicle,
    repository::vehicle::*,
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub(crate) async fn list_vehicles(
    State(state): State<AppState>,
) -> Result<Json<Vec<VehicleResponse>>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let vehicles = repository.list().await.map_err(internal_error)?;
    Ok(Json(vehicles.iter().map(VehicleResponse::from).collect()))
}

pub(crate) async fn count_vehicles(
    State(state): State<AppState>,
) -> Result<Json<VehicleCount>, StatusCode> {
    let count = VehicleRepository::new(state.pool.clone())
        .count()
        .await
        .map_err(internal_error)?;
    Ok(Json(VehicleCount { count }))
}

pub(crate) async fn get_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let vehicle = repository
        .find(id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(VehicleResponse::from(&vehicle)))
}

pub(crate) async fn create_vehicle(
    State(state): State<AppState>,
    Json(input): Json<CreateVehicle>,
) -> Result<(StatusCode, Json<VehicleResponse>), StatusCode> {
    Vehicle::new(
        0,
        input.make.clone(),
        input.model.clone(),
        input.year,
        input.daily_rate_cents,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    let repository = VehicleRepository::new(state.pool.clone());
    let vehicle = repository
        .create(input.make, input.model, input.year, input.daily_rate_cents)
        .await
        .map_err(internal_error)?;
    Ok((StatusCode::CREATED, Json(VehicleResponse::from(&vehicle))))
}

pub(crate) async fn change_rate(
    State(state): State<AppState>,
    Path(id): Path<u32>,
    Json(input): Json<VehicleRateChange>,
) -> Result<Json<VehicleResponse>, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let mut vehicle = repository
        .find(id)
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    vehicle
        .change_rate(input.daily_rate_cents)
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let updated = repository
        .change_rate(id, vehicle.daily_rate_cents())
        .await
        .map_err(internal_error)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(VehicleResponse::from(&updated)))
}

pub(crate) async fn delete_vehicle(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<StatusCode, StatusCode> {
    let repository = VehicleRepository::new(state.pool.clone());
    let deleted = repository.delete(id).await.map_err(delete_error_status)?;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
