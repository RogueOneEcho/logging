use crate::{Logger, LoggerOptions, TimeFormat, Verbosity};
use colored::control::SHOULD_COLORIZE;
use colored::Colorize;
use log::{set_boxed_logger, set_max_level, trace};
use std::sync::Arc;

pub struct LoggerBuilder {
    options: LoggerOptions,
    init: bool,
}

impl LoggerBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            options: LoggerOptions::default(),
            init: true,
        }
    }

    #[must_use]
    pub fn with_options(mut self, options: LoggerOptions) -> Self {
        self.options = options;
        self
    }

    #[must_use]
    pub fn with_verbosity(mut self, verbosity: Verbosity) -> Self {
        self.options.verbosity = Some(verbosity);
        self
    }

    #[must_use]
    pub fn with_time_format(mut self, time_format: TimeFormat) -> Self {
        self.options.log_time_format = Some(time_format);
        self
    }

    #[must_use]
    pub fn with_include_filter(mut self, include_filter: String) -> Self {
        let mut filters = self.options.log_include_filters.unwrap_or_default();
        filters.push(include_filter);
        self.options.log_include_filters = Some(filters);
        self
    }

    #[must_use]
    pub fn with_exclude_filter(mut self, exclude_filter: String) -> Self {
        let mut filters = self.options.log_exclude_filters.unwrap_or_default();
        filters.push(exclude_filter);
        self.options.log_exclude_filters = Some(filters);
        self
    }

    #[must_use]
    pub fn with_init(mut self) -> Self {
        self.init = true;
        self
    }

    #[must_use]
    pub fn without_init(mut self) -> Self {
        self.init = false;
        self
    }

    #[must_use]
    pub fn create(self) -> Arc<Logger> {
        let logger = Arc::from(Logger::from(self.options));
        if self.init {
            init_logger(logger.clone());
        }
        logger
    }
}

//noinspection RsExperimentalTraitObligations
fn init_logger(logger: Arc<Logger>) {
    SHOULD_COLORIZE.set_override(true);
    let filter = logger
        .options
        .verbosity
        .unwrap_or_default()
        .to_level_filter();
    let logger = Box::new(logger);
    if let Err(error) = set_boxed_logger(logger).map(|()| set_max_level(filter)) {
        trace!("{} to initialize the logger: {}", "Failed".bold(), error);
    }
}
