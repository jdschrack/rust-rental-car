use axum::{
    extract::{MatchedPath, Request, State},
    middleware::Next,
    response::Response,
};
use prometheus::IntGauge;
use std::time::Instant;

use crate::internal::metrics::{ErrorCategory, Metrics};

const KNOWN_METHODS: [&str; 9] = [
    "GET", "HEAD", "POST", "PUT", "DELETE", "CONNECT", "OPTIONS", "TRACE", "PATCH",
];

fn method_label(method: &str) -> &'static str {
    KNOWN_METHODS
        .iter()
        .copied()
        .find(|m| *m == method)
        .unwrap_or("_OTHER")
}

struct InFlight(IntGauge);

impl InFlight {
    fn start(gauge: IntGauge) -> Self {
        gauge.inc();
        Self(gauge)
    }
}

impl Drop for InFlight {
    fn drop(&mut self) {
        self.0.dec();
    }
}

pub async fn track_http(State(metrics): State<Metrics>, req: Request, next: Next) -> Response {
    let method = method_label(req.method().as_str());
    let route = req
        .extensions()
        .get::<MatchedPath>()
        .map(|path| path.as_str().to_owned())
        .unwrap_or_else(|| "unmatched".to_owned());

    let _in_flight = InFlight::start(metrics.http_active_requests.clone());
    let started = Instant::now();

    let response = next.run(req).await;
    if response.status() == axum::http::StatusCode::INTERNAL_SERVER_ERROR {
        metrics.record_error(ErrorCategory::Database);
    }

    let status = response.status().as_u16().to_string();
    metrics
        .http_request_duration
        .with_label_values(&[method, &route, &status])
        .observe(started.elapsed().as_secs_f64());

    response
}
