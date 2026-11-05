use crate::{models::reservation::Reservation, repository::reservation::RentalError};
use axum::http::StatusCode;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateReservation {
    pub vehicle_id: u32,
    pub customer_id: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Serialize)]
pub struct ReservationResponse {
    pub id: u32,
    pub vehicle_id: u32,
    pub customer_id: u32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_cents: i64,
    pub status: &'static str,
}

impl From<&Reservation> for ReservationResponse {
    fn from(reservation: &Reservation) -> Self {
        Self {
            id: reservation.id(),
            vehicle_id: reservation.vehicle_id(),
            customer_id: reservation.customer_id(),
            start_date: reservation.start_date(),
            end_date: reservation.end_date(),
            total_cents: reservation.total_cents(),
            status: reservation.status().as_str(),
        }
    }
}

pub fn rental_error_status(error: RentalError) -> StatusCode {
    match error {
        RentalError::VehicleNotFound
        | RentalError::CustomerNotFound
        | RentalError::ReservationNotFound => StatusCode::NOT_FOUND,
        RentalError::InvalidDateRange | RentalError::InvalidRate | RentalError::PriceOverflow => {
            StatusCode::BAD_REQUEST
        }
        RentalError::VehicleUnavailable | RentalError::OverlappingReservation => {
            StatusCode::CONFLICT
        }
        RentalError::Database(error) => {
            tracing::error!(
                operation = "reservation_dto",
                error.category = "database",
                "database operation failed"
            );
            eprintln!("Reservation database error: {error}");
            StatusCode::INTERNAL_SERVER_ERROR
        }
        RentalError::InvalidStoredData => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
