//! A logging library that provides logs you'll actually want to read.
//!
//! - Colorized output with configurable verbosity levels
//! - Flexible time formats (local, UTC, elapsed, or none)
//! - Target-based filtering by package name

mod errors;
#[cfg(feature = "log")]
mod logging;

pub use errors::*;
#[cfg(feature = "log")]
pub use logging::*;
