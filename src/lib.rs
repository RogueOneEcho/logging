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
mod logger;
mod options;
#[cfg(test)]
mod tests;
mod time_format;
mod verbosity;
