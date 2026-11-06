use prometheus::Registry;
use rust_rent::{
    api::router::{AppState, router},
    database,
    internal::{http_metrics::track_http, metrics::Metrics, telemetry},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let otlp = telemetry::init()?;

    tracing::info!(operation = "startup", "service starting");
    let registry = Registry::new();
    let metrics = Metrics::register(&registry)?;
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://rust_rent.db".to_owned());
    let pool = database::connect(&database_url)
        .await
        .map_err(|e| e.to_string())?;

    let state = AppState {
        pool: pool.clone(),
        metrics: metrics.clone(),
    };

    let api = router(state.clone()).layer(axum::middleware::from_fn_with_state(
        state.metrics.clone(),
        track_http,
    ));

    let listener_address: String = "0.0.0.0:3000".into();

    let listener = tokio::net::TcpListener::bind(listener_address.clone()).await?;

    tracing::info!(
        operation = "startup",
        address = listener_address,
        "Service Started and listening"
    );
    axum::serve(listener, api).await?;

    otlp.shutdown();

    Ok(())
}
