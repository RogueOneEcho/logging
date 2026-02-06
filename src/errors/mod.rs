//! Error types for structured error handling and reporting.

#[cfg(feature = "miette-fancy")]
mod diagnostic_ext;
mod error;
#[cfg(feature = "miette")]
mod failure;
#[cfg(test)]
mod tests;

#[cfg(feature = "miette-fancy")]
pub use diagnostic_ext::*;
pub use error::*;
#[cfg(feature = "miette")]
pub use failure::*;
#[cfg(feature = "miette")]
pub use miette::Severity;
