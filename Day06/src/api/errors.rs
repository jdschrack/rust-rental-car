use axum::http::StatusCode;

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
