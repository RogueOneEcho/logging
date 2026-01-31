//! A logging library that provides logs you'll actually want to read.
//!
//! - Colorized output with configurable verbosity levels
//! - Flexible time formats (local, UTC, elapsed, or none)
//! - Target-based filtering by package name

pub use builder::*;
pub use colors::*;
pub use error::*;
pub use logger::*;
pub use options::*;
pub use time_format::*;
pub use verbosity::*;

mod builder;
mod colors;
mod error;
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod error_tests;
mod logger;
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod logger_tests;
mod options;
mod time_format;
mod verbosity;
