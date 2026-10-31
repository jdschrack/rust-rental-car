use crate::api::{customer::*, vehicle::*};
use axum::{Router, http::StatusCode, routing::get};

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/vehicles", get(list_vehicles))
        .route("/vehicles/count", get(count_vehicles))
        .route(
            "/vehicles/{id}",
            get(get_vehicle)
                .patch(change_rate)
                .post(create_vehicle)
                .delete(delete_vehicle),
        )
        .route("/customers", get(list_customers).post(create_customer))
        .with_state(state)
}

pub(crate) fn internal_error(error: Box<dyn std::error::Error + Send + Sync>) -> StatusCode {
    eprintln!("Database or row conversion error: {error}");
    StatusCode::INTERNAL_SERVER_ERROR
}

pub(crate) fn delete_error_status(error: Box<dyn std::error::Error + Send + Sync>) -> StatusCode {
    if let Some(sqlx::Error::Database(db)) = error.downcast_ref::<sqlx::Error>()
        && db.is_foreign_key_violation()
    {
        return StatusCode::CONFLICT;
    }
    internal_error(error)
}
