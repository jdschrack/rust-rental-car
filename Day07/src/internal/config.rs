//! Application configuration.
//!
//! Values come from the system environment, then a `.env` file, then the
//! defaults below. `.env` is loaded into the process environment so that
//! variables read directly by the OpenTelemetry SDK, such as
//! `OTEL_EXPORTER_OTLP_ENDPOINT`, work from there too.

const DEFAULT_DATABASE_URL: &str = "sqlite://rust_rent.db";
const DEFAULT_LOG_FILTER: &str = "info";
const DEFAULT_DEPLOYMENT_ENV: &str = "local";
const DEFAULT_SERVICE_NAME: &str = "reservation-api";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub database_url: String,
    pub log_filter: String,
    pub deployment_env: String,
    pub service_name: String,
}

impl Config {
    /// Loads `.env`, if there is one, and reads the configuration.
    ///
    /// Call this before starting any threads: loading `.env` modifies the
    /// process environment. A missing `.env` is fine; a malformed one is an
    /// error.
    pub fn load() -> Result<Self, dotenvy::Error> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(error) if error.not_found() => {}
            Err(error) => return Err(error),
        }
        Ok(Self::from_lookup(|key| std::env::var(key).ok()))
    }

    fn from_lookup(lookup: impl Fn(&str) -> Option<String>) -> Self {
        // An empty value counts as unset, as it does for OTEL_* variables.
        let value = |key: &str, default: &str| {
            lookup(key)
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| default.to_owned())
        };

        Self {
            database_url: value("DATABASE_URL", DEFAULT_DATABASE_URL),
            log_filter: value("RUST_LOG", DEFAULT_LOG_FILTER),
            deployment_env: value("DEPLOYMENT_ENV", DEFAULT_DEPLOYMENT_ENV),
            service_name: value("OTEL_SERVICE_NAME", DEFAULT_SERVICE_NAME),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_from(vars: &[(&str, &str)]) -> Config {
        Config::from_lookup(|key| {
            vars.iter()
                .find(|(name, _)| *name == key)
                .map(|(_, value)| (*value).to_owned())
        })
    }

    #[test]
    fn uses_defaults_when_unset() {
        assert_eq!(
            config_from(&[]),
            Config {
                database_url: "sqlite://rust_rent.db".to_owned(),
                log_filter: "info".to_owned(),
                deployment_env: "local".to_owned(),
                service_name: "reservation-api".to_owned(),
            }
        );
    }

    #[test]
    fn reads_each_variable() {
        assert_eq!(
            config_from(&[
                ("DATABASE_URL", "sqlite://other.db"),
                ("RUST_LOG", "debug"),
                ("DEPLOYMENT_ENV", "production"),
                ("OTEL_SERVICE_NAME", "rentals"),
            ]),
            Config {
                database_url: "sqlite://other.db".to_owned(),
                log_filter: "debug".to_owned(),
                deployment_env: "production".to_owned(),
                service_name: "rentals".to_owned(),
            }
        );
    }

    #[test]
    fn treats_empty_values_as_unset() {
        assert_eq!(config_from(&[("DATABASE_URL", "")]), config_from(&[]));
    }
}
