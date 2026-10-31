use crate::api::{customer_service::*, reservation_service::*, vehicle_service::*};
use axum::{
    Router,
    routing::{get, post},
};

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(list_vehicles).post(create_vehicle))
        .route("/vehicles/count", get(count_vehicles))
        .route(
            "/vehicles/{id}",
            get(get_vehicle).patch(change_rate).delete(delete_vehicle),
        )
        .route("/customers", get(list_customers).post(create_customer))
        .route("/reservations", post(create_reservation))
        .route("/reservations/{id}", get(get_reservation))
        .route("/reservations/{id}/cancel", post(cancel_reservation))
        .with_state(state)
}
