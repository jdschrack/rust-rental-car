use crate::{
    api::router::AppState,
    dto::reservation_dto::{CreateReservation, ReservationResponse, rental_error_status},
    internal::metrics::{Operation, Outcome},
    repository::reservation::ReservationRepository,
};

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

pub(crate) async fn create_reservation(
    State(state): State<AppState>,
    Json(input): Json<CreateReservation>,
) -> Result<(StatusCode, Json<ReservationResponse>), StatusCode> {
    let service = ReservationRepository::new(state.pool.clone());
    let reservation = service
        .create(
            input.vehicle_id,
            input.customer_id,
            input.start_date,
            input.end_date,
        )
        .await;

    match reservation {
        Ok(reservation) => {
            state
                .metrics
                .record_reservation(Operation::Create, Outcome::Ok);
            Ok((
                StatusCode::CREATED,
                Json(ReservationResponse::from(&reservation)),
            ))
        }
        Err(err) => {
            state
                .metrics
                .record_reservation(Operation::Create, Outcome::Rejected);
            tracing::error!("unable to create reservation");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub(crate) async fn get_reservation(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ReservationResponse>, StatusCode> {
    let service = ReservationRepository::new(state.pool.clone());
    let reservation = service
        .find(id)
        .await
        .map_err(rental_error_status)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ReservationResponse::from(&reservation)))
}

pub(crate) async fn cancel_reservation(
    State(state): State<AppState>,
    Path(id): Path<u32>,
) -> Result<Json<ReservationResponse>, StatusCode> {
    let service = ReservationRepository::new(state.pool.clone());
    let reservation = service.cancel(id).await.map_err(rental_error_status)?;
    Ok(Json(ReservationResponse::from(&reservation)))
}
