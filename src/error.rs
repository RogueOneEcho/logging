use std::backtrace::{Backtrace, BacktraceStatus};
use std::fmt::{Debug, Display, Formatter};

use colored::Colorize;
use log::{error, trace};
use serde::{Deserialize, Serialize};

/// A serializable and log friendly error
#[derive(Deserialize, Serialize)]
pub struct Error {
    /// A concise description of the action that failed.
    ///
    /// Typically starts with a verb.
    ///
    /// Will be displayed as:
    /// > Failed to {action}
    ///
    /// Example: `deserialize object`
    pub action: String,

    /// A concise message describing the error.
    ///
    /// Displayed after the domain.
    ///
    /// Example: `Object is not valid.`
    pub message: String,

    /// A concise description of the domain in which this occurred:
    ///
    /// Will be displayed as:
    /// > A {domain} error occurred
    ///
    /// Example: `serialization`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// An HTTP status code.
    ///
    /// Will be displayed as:
    /// > A {status_code} error occurred
    ///
    /// Example: `404`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<u16>,

    /// Backtrace
    #[serde(skip)]
    pub backtrace: Option<Backtrace>,
}

impl Default for Error {
    fn default() -> Self {
        Self {
            action: String::new(),
            message: String::new(),
            domain: None,
            status_code: None,
            backtrace: get_backtrace(),
        }
    }
}

impl Error {
    /// Format the error as separate lines.
    fn lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("{} to {}", "Failed".bold(), self.action));
        if let Some(domain) = &self.domain {
            lines.push(format!("A {domain} error occurred"));
        }
        if let Some(status_code) = &self.status_code {
            lines.push(format!("A {status_code} error occurred"));
        }
        lines.push(self.message.clone());
        lines
    }

    /// Log the error from separate lines.
    pub fn log(&self) {
        for line in self.lines() {
            error!("{line}");
        }
        if let Some(backtrace) = &self.backtrace {
            trace!("Backtrace:\n{backtrace}");
        }
    }

    /// Get the error as a multiline string.
    pub fn display(&self) -> String {
        self.lines().join("\n")
    }
}

impl Debug for Error {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.lines().join("\n"))
    }
}

impl Display for Error {
    #[allow(clippy::absolute_paths)]
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.lines().join("\n"))
    }
}

#[allow(clippy::absolute_paths)]
impl std::error::Error for Error {}

impl Clone for Error {
    fn clone(&self) -> Self {
        Self {
            action: self.action.clone(),
            domain: self.domain.clone(),
            message: self.message.clone(),
            status_code: self.status_code,
            backtrace: None,
        }
    }
}

#[allow(clippy::wildcard_enum_match_arm)]
fn get_backtrace() -> Option<Backtrace> {
    let backtrace = Backtrace::capture();
    match backtrace.status() {
        BacktraceStatus::Captured => Some(backtrace),
        _ => None,
    }
}
