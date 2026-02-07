//! Fluent builder for creating [`Logger`] instances.

use crate::{Logger, LoggerOptions, TimeFormat, Verbosity};

/// Fluent builder for creating and configuring a [`Logger`].
pub struct LoggerBuilder {
    options: LoggerOptions,
}

impl LoggerBuilder {
    /// Create a new builder with default options.
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: LoggerOptions::default(),
        }
    }

    /// Set all options from a [`LoggerOptions`] struct.
    #[must_use]
    pub fn with_options(mut self, options: LoggerOptions) -> Self {
        self.options = options;
        self
    }

    /// Set the verbosity level.
    #[must_use]
    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.options.verbosity = Some(verbosity);
        self
    }

    /// Set the time format.
    #[must_use]
    pub fn with_time_format(mut self, time_format: TimeFormat) -> Self {
        self.options.log_time_format = Some(time_format);
        self
    }

    /// Add a package name filter to include.
    #[must_use]
    pub fn with_include_filter(mut self, include_filter: String) -> Self {
        let mut filters = self.options.log_include_filters.unwrap_or_default();
        filters.push(include_filter);
        self.options.log_include_filters = Some(filters);
        self
    }

    /// Add a package name filter to exclude.
    #[must_use]
    pub fn with_exclude_filter(mut self, exclude_filter: String) -> Self {
        let mut filters = self.options.log_exclude_filters.unwrap_or_default();
        filters.push(exclude_filter);
        self.options.log_exclude_filters = Some(filters);
        self
    }

    /// Build and return the configured [`Logger`].
    #[must_use]
    pub fn create(self) -> Logger {
        Logger::from(self.options)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}
