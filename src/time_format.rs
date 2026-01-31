//! Time format options for log timestamps.

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Timestamp format for log output.
#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum TimeFormat {
    /// Local date and time in an ISO 8601 like format.
    ///
    /// Example: `2013-02-27 12:34:56`
    #[default]
    Local,
    /// UTC date and time in an ISO 8601 like format.
    ///
    /// Example: `2013-02-27 12:34:56Z`
    Utc,
    /// Elapsed time since program start in seconds with millisecond precision.
    ///
    /// Example: `30020.289`
    Elapsed,
    /// No timestamp.
    None,
}
