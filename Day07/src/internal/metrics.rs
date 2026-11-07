use prometheus::{
    Histogram, HistogramOpts, HistogramVec, IntCounterVec, IntGauge, IntGaugeVec, Opts, Registry,
};

/// Bucket bounds (seconds) recommended by the OpenTelemetry HTTP conventions.
const HTTP_BUCKETS: [f64; 14] = [
    0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0,
];
const LOCK_BUCKETS: [f64; 11] = [
    0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
];
const QUERY_BUCKETS: [f64; 11] = [
    0.0005, 0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5,
];

// Closed vocabularies: the only values allowed as labels.
#[derive(Clone, Copy)]
pub enum Operation {
    Create,
    Cancel,
}
#[derive(Clone, Copy)]
pub enum Outcome {
    Ok,
    Conflict,
    Rejected,
}
#[derive(Clone, Copy)]
pub enum ErrorCategory {
    Database,
    DataIntegrity,
}

impl Operation {
    const ALL: [Self; 2] = [Self::Create, Self::Cancel];
    fn as_str(self) -> &'static str {
        match self {
            Self::Create => "create",
            Self::Cancel => "cancel",
        }
    }
}
impl Outcome {
    const ALL: [Self; 3] = [Self::Ok, Self::Conflict, Self::Rejected];
    fn as_str(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Conflict => "conflict",
            Self::Rejected => "rejected",
        }
    }
}
impl ErrorCategory {
    const ALL: [Self; 2] = [Self::Database, Self::DataIntegrity];
    fn as_str(self) -> &'static str {
        match self {
            Self::Database => "database",
            Self::DataIntegrity => "data_integrity",
        }
    }
}

/// Cheap to clone: every field is a handle to shared state.
#[derive(Clone)]
pub struct Metrics {
    pub http_request_duration: HistogramVec,
    pub http_active_requests: IntGauge,
    reservation_operations: IntCounterVec,
    app_errors: IntCounterVec,
    pub db_write_lock_wait: Histogram,
    pub db_query_duration: HistogramVec,
    pub db_pool_connections: IntGaugeVec,
}

impl Metrics {
    pub fn register(registry: &Registry) -> prometheus::Result<Self> {
        let http_request_duration = HistogramVec::new(
            HistogramOpts::new(
                "http_server_request_duration_seconds",
                "Duration of HTTP server requests.",
            )
            .buckets(HTTP_BUCKETS.to_vec()),
            &[
                "http_request_method",
                "http_route",
                "http_response_status_code",
            ],
        )?;
        let http_active_requests = IntGauge::new(
            "http_server_active_requests",
            "Number of HTTP requests currently being served.",
        )?;
        let reservation_operations = IntCounterVec::new(
            Opts::new(
                "reservation_operations_total",
                "Reservation operations by outcome.",
            ),
            &["operation", "outcome"],
        )?;
        let app_errors = IntCounterVec::new(
            Opts::new("app_errors_total", "Unexpected failures by category."),
            &["error_category"],
        )?;
        let db_write_lock_wait = Histogram::with_opts(
            HistogramOpts::new(
                "db_write_lock_wait_seconds",
                "Time spent waiting for the SQLite write lock (BEGIN IMMEDIATE).",
            )
            .buckets(LOCK_BUCKETS.to_vec()),
        )?;
        let db_query_duration = HistogramVec::new(
            HistogramOpts::new(
                "db_query_duration_seconds",
                "Repository operation duration.",
            )
            .buckets(QUERY_BUCKETS.to_vec()),
            &["operation"],
        )?;
        let db_pool_connections = IntGaugeVec::new(
            Opts::new("db_pool_connections", "Database pool connections by state."),
            &["state"],
        )?;
        let build_info = IntGaugeVec::new(
            Opts::new(
                "rust_rent_build_info",
                "Build information; the value is always 1.",
            ),
            &["version"],
        )?;
        build_info
            .with_label_values(&[env!("CARGO_PKG_VERSION")])
            .set(1);

        registry.register(Box::new(http_request_duration.clone()))?;
        registry.register(Box::new(http_active_requests.clone()))?;
        registry.register(Box::new(reservation_operations.clone()))?;
        registry.register(Box::new(app_errors.clone()))?;
        registry.register(Box::new(db_write_lock_wait.clone()))?;
        registry.register(Box::new(db_query_duration.clone()))?;
        registry.register(Box::new(db_pool_connections.clone()))?;
        registry.register(Box::new(build_info))?;

        // CPU, memory and open files. The collector exists on Linux only.
        #[cfg(target_os = "linux")]
        registry.register(Box::new(
            prometheus::process_collector::ProcessCollector::for_self(),
        ))?;

        // Create known series at zero so rate() and absent() behave before the first event.
        for op in Operation::ALL {
            for outcome in Outcome::ALL {
                reservation_operations.with_label_values(&[op.as_str(), outcome.as_str()]);
            }
        }
        for category in ErrorCategory::ALL {
            app_errors.with_label_values(&[category.as_str()]);
        }
        for state in ["idle", "in_use"] {
            db_pool_connections.with_label_values(&[state]);
        }

        Ok(Self {
            http_request_duration,
            http_active_requests,
            reservation_operations,
            app_errors,
            db_write_lock_wait,
            db_query_duration,
            db_pool_connections,
        })
    }

    pub fn record_reservation(&self, op: Operation, outcome: Outcome) {
        self.reservation_operations
            .with_label_values(&[op.as_str(), outcome.as_str()])
            .inc();
    }

    pub fn record_error(&self, category: ErrorCategory) {
        self.app_errors
            .with_label_values(&[category.as_str()])
            .inc();
    }
}
