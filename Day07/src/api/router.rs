use crate::api::{customer::*, vehicle::*};
use axum::{Router, routing::get};

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
        .with_state(state)
}
