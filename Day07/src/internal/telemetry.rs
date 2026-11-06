use opentelemetry::{KeyValue, global, trace::TracerProvider as _};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::{
    Resource, error::OTelSdkError, metrics::SdkMeterProvider, propagation::TraceContextPropagator,
    trace::SdkTracerProvider,
};
use prometheus::Registry;
use tracing_subscriber::{EnvFilter, Layer, layer::SubscriberExt, util::SubscriberInitExt};

pub struct TelemetryGuard {
    provider: SdkTracerProvider,
}

impl TelemetryGuard {
    pub fn shutdown(self) {
        if let Err(err) = self.provider.shutdown() {
            tracing::warn!(error = %err, "telemetry shutdown failed");
        }
    }
}

pub fn init() -> Result<TelemetryGuard, Box<dyn std::error::Error + Send + Sync>> {
    global::set_text_map_propagator(TraceContextPropagator::new());

    let exporter = SpanExporter::builder().with_http().build()?;

    let mut resource = Resource::builder().with_attributes([
        KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        KeyValue::new(
            "deployment.environment.name",
            std::env::var("DEPLOYMENT_ENV").unwrap_or_else(|_| "local".into()),
        ),
    ]);

    if std::env::var_os("OTEL_SERVICE_NAME").is_none() {
        resource = resource.with_service_name("reservation-api");
    }

    let provider = SdkTracerProvider::builder()
        .with_batch_exporter(exporter)
        .with_resource(resource.build())
        .build();

    let tracer = provider.tracer("reservation-api");
    global::set_tracer_provider(provider.clone());

    let log_layer = tracing_subscriber::fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(false)
        .with_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")));

    let otel_layer = tracing_opentelemetry::layer()
        .with_tracer(tracer)
        .with_filter(EnvFilter::new("info.otel.tracing=trace"));

    tracing_subscriber::registry()
        .with(log_layer)
        .with(otel_layer)
        .init();

    Ok(TelemetryGuard { provider })
}

pub fn init_meter_provider(registry: Registry) -> Result<SdkMeterProvider, OTelSdkError> {
    let exporter = opentelemetry_prometheus::exporter()
        .with_registry(registry)
        .build()?;

    let provider = SdkMeterProvider::builder().with_reader(exporter).build();

    opentelemetry::global::set_meter_provider(provider.clone());
    Ok(provider)
}
