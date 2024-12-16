use chrono::{Local, Utc};
use colored::{ColoredString, Colorize};
use log::*;
use std::borrow::ToOwned;
use std::time::SystemTime;

use crate::*;

const PACKAGE_NAME: &str = "rogue_logging";

pub struct Logger {
    pub(crate) options: LoggerOptions,
    start: SystemTime,
}

impl From<LoggerOptions> for Logger {
    fn from(options: LoggerOptions) -> Self {
        Self {
            options,
            start: SystemTime::now(),
        }
    }
}

impl Logger {
    fn format_log(&self, verbosity: Verbosity, message: String) -> String {
        let prefix = self.format_prefix(verbosity);
        let message = format_message(verbosity, message);
        format!("{prefix} {message}")
    }

    #[must_use]
    pub fn format_prefix(&self, verbosity: Verbosity) -> String {
        let time = self.format_time();
        let verbosity_id = verbosity.get_id();
        let icon = verbosity.get_icon();
        format!("{time}{verbosity_id} {icon}")
    }

    fn format_time(&self) -> ColoredString {
        let value = match self.options.log_time_format.unwrap_or_default() {
            TimeFormat::Local => Local::now().format("%Y-%m-%d %H:%M:%S%.3f ").to_string(),
            TimeFormat::Utc => Utc::now().format("%Y-%m-%d %H:%M:%S%.3fZ ").to_string(),
            TimeFormat::Elapsed => format!(
                "{:>8.3} ",
                self.start.elapsed().unwrap_or_default().as_secs_f64()
            ),
            TimeFormat::None => String::new(),
        };
        value.dark_gray()
    }

    fn exclude_by_target(&self, target: &str) -> bool {
        if let Some(exclude_filters) = self.options.log_exclude_filters.clone() {
            for filter in exclude_filters {
                if target.starts_with(&filter) {
                    return true;
                }
            }
        }
        if let Some(mut include_filters) = self.options.log_include_filters.clone() {
            include_filters.push(PACKAGE_NAME.to_owned());
            for filter in include_filters {
                if !target.starts_with(&filter) {
                    return true;
                }
            }
        }
        false
    }

    fn exclude_by_verbosity(&self, verbosity: Verbosity) -> bool {
        verbosity.as_num() > self.options.verbosity.unwrap_or_default().as_num()
    }
}

impl Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        let target = metadata.target();
        let verbosity = Verbosity::from_level(metadata.level());
        !self.exclude_by_target(target) && !self.exclude_by_verbosity(verbosity)
    }

    #[allow(clippy::print_stderr)]
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let verbosity = Verbosity::from_level(record.level());
            let message = format!("{}", record.args());
            let log = self.format_log(verbosity, message);
            eprintln!("{log}");
        }
    }

    fn flush(&self) {}
}

fn format_message(verbosity: Verbosity, message: String) -> String {
    if verbosity.as_num() >= Verbosity::Debug.as_num() {
        format!("{}", message.dimmed())
    } else {
        message
    }
}
