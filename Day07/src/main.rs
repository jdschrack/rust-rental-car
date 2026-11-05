use rust_rent::{
    api::router::{AppState, router},
    database,
    internal::telemetry,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let otlp = telemetry::init()?;

    tracing::info!(operation = "startup", "service starting");
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://rust_rent.db".to_owned());
    let pool = database::connect(&database_url)
        .await
        .map_err(|e| e.to_string())?;

    let app = router(AppState { pool });

    let listener_address: String = "0.0.0.0:3000".into();

    let listener = tokio::net::TcpListener::bind(listener_address.clone()).await?;

    tracing::info!(
        operation = "startup",
        address = listener_address,
        "Service Started and listening"
    );

    axum::serve(listener, app).await?;

    otlp.shutdown();

    Ok(())
}
