use axum::Router;
use axum::routing::get;
use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};
#[cfg(feature = "metrics")]
use prometheus::{Registry, TextEncoder};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Clone, Default)]
pub struct ReadinessProbe {
    ready: Arc<AtomicBool>,
}

impl ReadinessProbe {
    pub fn set_ready(&self) {
        self.ready.store(true, Ordering::Relaxed)
    }

    pub fn set_unready(&self) {
        self.ready.store(false, Ordering::Relaxed);
    }

    fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Relaxed)
    }
}

#[derive(Clone, Default)]
pub struct StartupProbe {
    started: Arc<AtomicBool>,
}

impl StartupProbe {
    pub fn set_started(&self) {
        self.started.store(true, Ordering::Relaxed);
    }

    fn is_started(&self) -> bool {
        self.started.load(Ordering::Relaxed)
    }
}

#[derive(Clone)]
struct AdminState {
    #[cfg(feature = "metrics")]
    registry: Arc<Registry>,
    readiness: Option<ReadinessProbe>,
    startup: Option<StartupProbe>,
}

async fn alive_handler() -> &'static str {
    "alive"
}

async fn ready_handler(State(state): State<AdminState>) -> Response {
    match &state.readiness {
        None => (StatusCode::NOT_FOUND, "not configured").into_response(),
        Some(probe) if probe.is_ready() => (StatusCode::OK, "ready").into_response(),
        Some(_) => (StatusCode::SERVICE_UNAVAILABLE, "not ready").into_response(),
    }
}

async fn started_handler(State(state): State<AdminState>) -> Response {
    match &state.startup {
        None => (StatusCode::NOT_FOUND, "not configured").into_response(),
        Some(probe) if probe.is_started() => (StatusCode::OK, "started").into_response(),
        Some(_) => (StatusCode::SERVICE_UNAVAILABLE, "not started").into_response(),
    }
}

async fn root_handler() -> Html<&'static str> {
    Html(
        r#"<!DOCTYPE HTML>
<html>
  <head>
	<title>Administrative Service</title>
  </head>
  <body>
	<h1>Administrative Operations</h1>
	<ul>
	  <li><a href="/metrics">/metrics</a></li>
	  <li><a href="/alive">/alive</a></li>
	  <li><a href="/ready">/ready</a></li>
	  <li><a href="/started">/started</a></li>
	</ul>
  </body>
</html>"#,
    )
}

pub(crate) fn admin_app(
    #[cfg(feature = "metrics")] registry: Registry,
    readiness: Option<ReadinessProbe>,
    startup: Option<StartupProbe>,
) -> Router {
    let state = AdminState {
        #[cfg(feature = "metrics")]
        registry: Arc::new(registry),
        readiness,
        startup,
    };
    let router = Router::new()
        .route("/", get(root_handler))
        .route("/alive", get(alive_handler))
        .route("/ready", get(ready_handler))
        .route("/started", get(started_handler));

    #[cfg(feature = "metrics")]
    let router = router.route("/metrics", get(metrics_handler));

    router.with_state(state)
}

#[cfg(feature = "metrics")]
async fn metrics_handler(State(state): State<AdminState>) -> Result<String, AdminServerError> {
    let encoder = TextEncoder::new();
    let mut buffer = String::new();
    encoder.encode_utf8(&state.registry.gather(), &mut buffer)?;
    Ok(buffer)
}

#[derive(thiserror::Error, Debug)]
#[cfg(feature = "metrics")]
enum AdminServerError {
    #[error("Prometheus error: {0}")]
    Prometheus(#[from] prometheus::Error),
}

#[cfg(feature = "metrics")]
impl IntoResponse for AdminServerError {
    fn into_response(self) -> Response {
        tracing::error!("Internal error in admin server: {}", self);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
