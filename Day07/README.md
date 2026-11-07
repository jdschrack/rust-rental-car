# RustRent

A car rental REST API built with Rust, Axum, and SQLite. It manages vehicles, customers, and reservations, and prevents double-booking of a vehicle. Logs are structured JSON on stdout.

## Getting started

You need a recent stable Rust toolchain (the crate uses edition 2024). SQLite is bundled, so there is nothing else to install.

```sh
cargo run
```

The service listens on `0.0.0.0:3000`, and the admin server (`/metrics`, `/alive`, `/ready`, `/started`) on `0.0.0.0:8081`. Both ports are fixed in `src/main.rs`. On first run it creates `rust_rent.db` in the working directory and applies the migrations. They are embedded at compile time, so rebuild after changing anything in `migrations/`.

The API has no authentication, and the service binds to all interfaces, so don't expose it to an untrusted network.

```sh
curl localhost:3000/health
```

## Configuration

Nothing is required: every setting has a working default. To change one, set a system environment variable or put it in a `.env` file, which the service loads at startup. System environment variables take precedence over `.env`.

```sh
cp .env.example .env
cargo run
```

The service exits at startup if `.env` is malformed. `src/internal/config.rs` reads these settings and passes them to the rest of the application.

| Variable | Default | Purpose |
| --- | --- | --- |
| `DATABASE_URL` | `sqlite://rust_rent.db` | SQLite database location |
| `RUST_LOG` | `info` | Log filter for stdout |
| `DEPLOYMENT_ENV` | `local` | `deployment.environment.name` on traces |
| `OTEL_SERVICE_NAME` | `reservation-api` | `service.name` on traces |
| `OTEL_EXPORTER_OTLP_ENDPOINT` | `http://localhost:4318` | OTLP/HTTP collector base URL |

Other standard OpenTelemetry variables, such as `OTEL_EXPORTER_OTLP_HEADERS` and `OTEL_TRACES_SAMPLER`, are read by the SDK. The OTLP exporter only supports `http/protobuf`, because gRPC isn't compiled in.

## API

Request and response bodies are JSON. Error responses have a status code and an empty body. Dates are `YYYY-MM-DD`, and money is in integer cents.

| Method | Path | Description | Success | Errors |
| --- | --- | --- | --- | --- |
| `GET` | `/health` | Liveness check, returns `OK` | 200 | |
| `GET` | `/vehicles` | List vehicles | 200 | |
| `POST` | `/vehicles` | Create a vehicle | 201 | 400 |
| `GET` | `/vehicles/count` | Count vehicles | 200 | |
| `GET` | `/vehicles/{id}` | Get a vehicle | 200 | 404 |
| `PATCH` | `/vehicles/{id}` | Change the daily rate | 200 | 400, 404 |
| `DELETE` | `/vehicles/{id}` | Delete a vehicle | 204 | 404, 409 if it has reservations |
| `GET` | `/customers` | List customers | 200 | |
| `POST` | `/customers` | Create a customer | 201 | 400, 409 if the email exists |
| `POST` | `/reservations` | Create a reservation | 201 | 500, see [Known limitations](#known-limitations) |
| `GET` | `/reservations/{id}` | Get a reservation | 200 | 404 |
| `POST` | `/reservations/{id}/cancel` | Cancel a reservation; safe to repeat | 200 | 404 |

### Example

```sh
curl -X POST localhost:3000/vehicles \
  -H 'content-type: application/json' \
  -d '{"make":"Toyota","model":"Corolla","year":2024,"daily_rate_cents":5000}'

curl -X POST localhost:3000/customers \
  -H 'content-type: application/json' \
  -d '{"name":"Ada Lovelace","email":"ada@example.com"}'

curl -X POST localhost:3000/reservations \
  -H 'content-type: application/json' \
  -d '{"vehicle_id":1,"customer_id":1,"start_date":"2026-12-01","end_date":"2026-12-04"}'
```

The last request returns the reservation, priced at three days of the daily rate:

```json
{"id":1,"vehicle_id":1,"customer_id":1,"start_date":"2026-12-01","end_date":"2026-12-04","total_cents":15000,"status":"confirmed"}
```

## Development

```sh
cargo test
```

Tests don't need any environment variables.

| Path | Contents |
| --- | --- |
| `src/api/` | Router and request handlers |
| `src/dto/` | Request and response types |
| `src/models/` | Domain types |
| `src/repository/` | SQLite queries and reservation transactions |
| `src/internal/` | Telemetry, metrics, and the admin server |
| `migrations/` | Database schema |
| `tests/` | Integration tests |

`OPENTELEMETRY_PLAN.md` describes the planned observability work.

## Known limitations

- **Every failed reservation returns 500.** This includes overlapping dates, unknown vehicles or customers, and invalid date ranges, because `create_reservation` doesn't map the error to a status.
- **There is no graceful shutdown.** The service exits on `SIGTERM` or `Ctrl-C` without running telemetry shutdown, so spans still in the batch can be lost.
