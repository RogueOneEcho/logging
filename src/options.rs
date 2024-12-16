use crate::{TimeFormat, Verbosity};
use serde::{Deserialize, Serialize};

/// Options for [`Logger`]
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct LoggerOptions {
    /// Level of logs to display.
    ///
    /// Default: `info`
    pub verbosity: Option<Verbosity>,

    /// Time format to use in logs.
    ///
    /// Default: `utc`
    pub log_time_format: Option<TimeFormat>,

    /// Include only logs from specific packages
    pub log_include_filters: Option<Vec<String>>,

    /// Exclude logs from specific packages
    pub log_exclude_filters: Option<Vec<String>>,
}
