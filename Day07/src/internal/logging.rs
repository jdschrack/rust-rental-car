use chrono::{DateTime, Utc};
use serde_json::{Map, Value as JsonValue};
use std::{collections::HashMap, fmt::Display, thread};

#[derive(Default, Clone)]
pub struct LogstashJsonFormat {
    config: LogstashConfig,
}

impl LogstashJsonFormat {
    pub fn builder() -> LogstashJsonFormatBuilder {
        LogstashJsonFormatBuilder::default()
    }
}

#[derive(Default)]
pub struct LogstashJsonFormatBuilder {
    config: LogstashConfig,
}

impl LogstashJsonFormatBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_default_field(mut self, key: impl Info<String>, value: impl Info<String>) -> Self {
        self.config.default.fields.insert(key.into(), value.into());
        self
    }
}
