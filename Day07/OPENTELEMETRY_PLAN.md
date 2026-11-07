# OpenTelemetry and tracing implementation plan

Use `tracing` for application spans and structured logs, export traces through OTLP to an OpenTelemetry Collector, and write JSON logs to stdout for the deployment platform to collect.

The repository already declares the main dependencies, but they are not wired up. Requests currently follow `router → API handler → repository → SQLite`; logging consists of a startup `println!` and several `eprintln!` calls.

## 1. Define the initial outcome

Each HTTP request should have a trace showing its repository work, duration, response status, and unexpected failures. Logs generated during that request should contain matching `trace_id` and `span_id` fields. Preserve all existing API responses and reservation transaction behavior.

## 2. Check dependency compatibility

Review `Cargo.toml` and the lockfile to confirm that `tracing-opentelemetry`, the OpenTelemetry SDK/exporter, and Axum instrumentation use compatible versions. Enable the required JSON formatting, environment filtering, and OTLP transport features.

Start with one trace exporter: OTLP. The existing Zipkin, Prometheus, metrics, and file-appender dependencies do not all need to participate.

## 3. Create one telemetry initialization point

Add a small telemetry module, called from `src/main.rs` before database initialization, containing:

- A `tracing_subscriber` registry with an environment filter.
- A JSON logging layer.
- A `tracing-opentelemetry` layer backed by a batch span exporter.
- Resource attributes identifying service name, version, and deployment environment.

Use standard settings such as `RUST_LOG`, `OTEL_SERVICE_NAME`, and the applicable OTLP endpoint/protocol variables. Keep provider ownership available for shutdown.

## 4. Make logs useful and safe

Replace the startup print and error prints in `src/api/errors.rs`, `src/api/customer_service.rs`, and `src/dto/reservation_dto.rs` with structured events. Give events stable messages and fields such as operation and error category.

Explicitly implement trace/span ID enrichment for JSON logs; adding the OpenTelemetry tracing layer alone does not guarantee those fields appear. Keep logs available even when traces are sampled out.

## 5. Instrument HTTP requests at the router boundary

Attach the existing Axum OpenTelemetry integration in `src/api/router.rs`, using its documented middleware ordering. Extract W3C `traceparent`/`tracestate` before creating the server span.

Record method, matched route template, response status, and duration. Use `/reservations/{id}` rather than individual URLs as span names. Verify coverage for extractor rejections and unmatched routes as well as successful handlers. Treat incoming trace context as untrusted metadata.

## 6. Add spans where the work is owned

Instrument repository operations in `src/repository/`, starting with reservation creation, cancellation, and lookup. Use async-safe instrumentation such as `#[instrument(skip_all)]` with explicitly approved fields; avoid holding an entered span guard across `.await`.

For reservation creation, make time waiting for `BEGIN IMMEDIATE` distinguishable from work inside the transaction. This helps diagnose SQLite contention without changing locking, commit, or rollback behavior. Add SQL-level detail only if repository spans prove insufficient.

## 7. Set error and privacy rules

Log unexpected database and stored-data failures once at the boundary that converts them into HTTP errors. Record expected validation failures, missing resources, and booking conflicts as outcomes without treating every 4xx as an operational fault. Mark server failures appropriately on the request span.

Exclude customer names, emails, request bodies, credentials, database URLs, SQL bind values, and unrestricted headers. Review database error text before exporting it. Use resource IDs only where needed, and never as metric labels.

## 8. Deploy the collection pipeline

Run an OpenTelemetry Collector alongside the application or as shared infrastructure. Configure an OTLP receiver, memory limiter, batching, and export to the selected trace backend. Collect stdout logs through the deployment's logging agent and preserve their correlation fields.

Apply authentication, TLS, retention, and access controls at the appropriate deployment boundaries. Avoid exporting the same log through both stdout collection and an OTLP log bridge.

## 9. Handle sampling, failure, and shutdown

Start with full sampling locally and parent-based ratio sampling in production, tuned to traffic and cost. Keep log-level filtering separate from span sampling so lowering log verbosity does not accidentally remove required traces.

Add graceful shutdown: stop accepting requests, allow in-flight work to finish within a deadline, then flush and shut down telemetry providers. Use bounded exporter queues and timeouts so collector outages cannot indefinitely block requests or process termination. Monitor export failures and dropped telemetry.

## 10. Validate before a gradual rollout

Extend the existing HTTP tests with focused telemetry checks using an in-memory exporter and captured logs:

- A successful request produces a server span and child repository span.
- Incoming trace context continues the expected trace.
- An unexpected failure produces one correlated error event.
- Invalid input, missing resources, and conflicts retain their existing responses.
- Logs contain correlation IDs and exclude seeded customer PII.
- Collector unavailability leaves API behavior intact; shutdown flushes buffered spans.

Run the existing reservation concurrency tests to confirm instrumentation preserves booking guarantees. Then exercise a local collector/backend, measure latency and telemetry volume, and roll out gradually. Document how to find a trace from a log and how to reduce telemetry volume during an incident.

## Scope and verification status

This document is a plan only; no application code was changed and no tests were run. Dependency compatibility and collector/backend integration remain unverified; those are the first implementation checks.
