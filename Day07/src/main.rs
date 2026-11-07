use prometheus::Registry;
use rust_rent::{
    api::router::{AppState, router},
    database,
    internal::{
        admin::{ReadinessProbe, StartupProbe, admin_app},
        config::Config,
        http_metrics::track_http,
        metrics::Metrics,
        telemetry,
    },
};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Loading .env modifies the process environment, so it has to happen
    // before the runtime starts its worker threads.
    let config = Config::load()?;

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()?
        .block_on(run(config))
}

async fn run(config: Config) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let otlp = telemetry::init(&config)?;

    tracing::info!(operation = "startup", "service starting");
    let registry = Registry::new();
    let metrics = Metrics::register(&registry)?;
    let pool = database::connect(&config.database_url)
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

    let startup = StartupProbe::default();
    let readiness = ReadinessProbe::default();
    let admin = admin_app(registry, Some(readiness.clone()), Some(startup.clone()));

    // Bound before spawning so a port conflict fails startup instead of
    // leaving the service running without metrics.
    let admin_address: String = "0.0.0.0:8081".into();
    let admin_listener = tokio::net::TcpListener::bind(admin_address.clone()).await?;
    tokio::spawn(async move {
        if let Err(error) = axum::serve(admin_listener, admin).await {
            tracing::error!(error = %error, "admin server stopped");
        }
    });

    tracing::info!(
        operation = "startup",
        address = admin_address,
        "Admin server listening"
    );

    startup.set_started();
    readiness.set_ready();

    axum::serve(listener, api).await?;

    otlp.shutdown();

    Ok(())
}
