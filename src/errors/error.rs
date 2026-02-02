//! Serializable error type with logging support.

#[cfg(feature = "log")]
use colored::Colorize;
#[cfg(feature = "log")]
use log::{error, trace};
use serde::{Deserialize, Serialize};
use std::backtrace::{Backtrace, BacktraceStatus};
use std::error::Error as StdError;
use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display, Formatter};

/// A serializable error with logging support.
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
        #[cfg(feature = "log")]
        lines.push(format!("{} to {}", "Failed".bold(), self.action));
        #[cfg(not(feature = "log"))]
        lines.push(format!("Failed to {}", self.action));
        if let Some(domain) = &self.domain {
            lines.push(format!("A {domain} error occurred"));
        }
        if let Some(status_code) = &self.status_code {
            lines.push(format!("A {status_code} error occurred"));
        }
        lines.push(self.message.clone());
        lines
    }

    /// Log the error at the error level with backtrace at trace level.
    #[cfg(feature = "log")]
    pub fn log(&self) {
        for line in self.lines() {
            error!("{line}");
        }
        if let Some(backtrace) = &self.backtrace {
            trace!("Backtrace:\n{backtrace}");
        }
    }

    /// Multiline string representation of the error.
    pub fn display(&self) -> String {
        self.lines().join("\n")
    }
}

impl Debug for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.lines().join("\n"))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FmtResult {
        write!(formatter, "{}", self.lines().join("\n"))
    }
}

impl StdError for Error {}

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
