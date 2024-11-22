use std::backtrace::{Backtrace, BacktraceStatus};
use std::convert::Infallible;
use std::fmt::{Debug, Display, Formatter};
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::string::FromUtf8Error;
use std::time::SystemTimeError;

use colored::Colorize;
use log::{error, trace, SetLoggerError};
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
    /// Create a new Error with a specific action and message
    #[must_use]
    pub fn new(action: String, message: String) -> Self {
        Self {
            action,
            message,
            domain: None,
            status_code: None,
            backtrace: get_backtrace(),
        }
    }

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

/// Converts a standard I/O error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::io::Error`
/// to the custom `Error` type, enabling seamless error handling and propagation
/// in I/O-related operations.
///
/// # Arguments
///
/// * `err` - The source `io::Error` to be converted
///
/// # Returns
///
/// A new `Error` instance with a generic I/O operation description and
/// the specific error message from the original `io::Error`.
///
/// # Examples
///
/// ```
/// let io_error = std::fs::File::open("nonexistent_file.txt").unwrap_err();
/// let custom_error: Error = io_error.into(); // Automatic conversion
/// ```
///
/// # Notes
///
/// - This implementation enables the use of the `?` operator for error conversion
/// - The context message is a generic "perform I/O operation" description
/// - The specific error details are extracted from the original `io::Error`
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::new("perform I/O operation".to_owned(), err.to_string())
    }
}

/// Converts a `FromUtf8Error` into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::string::FromUtf8Error`
/// to the custom `Error` type, facilitating error handling during UTF-8 string conversion
/// attempts from byte vectors.
///
/// # Arguments
///
/// * `err` - The source `FromUtf8Error` encountered during UTF-8 conversion
///
/// # Returns
///
/// A new `Error` instance with a UTF-8 conversion context and the specific
/// error message from the original `FromUtf8Error`.
///
/// # Examples
///
/// ```
/// let invalid_utf8_bytes = vec![0xFF, 0xFE, 0xFD];
/// let conversion_result = String::from_utf8(invalid_utf8_bytes);
///
/// // Automatic conversion to custom Error type
/// let custom_error: Error = conversion_result.unwrap_err().into();
/// ```
///
/// # Errors
///
/// Typically occurs when:
/// - Byte vector contains invalid UTF-8 sequences
/// - Incomplete multi-byte character encodings
/// - Non-UTF-8 encoded byte sequences
///
/// # Notes
///
/// - Enables seamless error propagation using the `?` operator
/// - Provides a consistent error representation for UTF-8 conversion failures
/// - Preserves the original error message for detailed diagnostics
impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Error::new("convert bytes to UTF-8 string".to_owned(), err.to_string())
    }
}

/// Converts a formatting error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::fmt::Error`
/// to the custom `Error` type, handling errors that occur during string
/// formatting operations.
///
/// # Arguments
///
/// * `err` - The source `std::fmt::Error` encountered during formatting
///
/// # Returns
///
/// A new `Error` instance with a formatting context and the specific
/// error message from the original formatting error.
///
/// # Examples
///
/// ```
/// use std::fmt::Write;
/// let mut output = String::new();
/// let result = write!(&mut output, "{:>5}", "test");
/// if let Err(fmt_error) = result {
///     let custom_error: Error = fmt_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables the use of the `?` operator for error propagation in formatting operations
/// - Provides consistent error handling for formatting failures
#[allow(clippy::absolute_paths)]
impl From<std::fmt::Error> for Error {
    fn from(err: std::fmt::Error) -> Self {
        Error::new("format string".to_owned(), err.to_string())
    }
}

/// Converts a UTF-8 validation error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::str::Utf8Error`
/// to the custom `Error` type, handling errors that occur during UTF-8
/// validation of byte slices.
///
/// # Arguments
///
/// * `err` - The source `Utf8Error` encountered during UTF-8 validation
///
/// # Returns
///
/// A new `Error` instance with a UTF-8 parsing context and the specific
/// error message from the original UTF-8 error.
///
/// # Examples
///
/// ```
/// let bytes = [0xFF, 0xFE, 0xFD];
/// let result = std::str::from_utf8(&bytes);
/// if let Err(utf8_error) = result {
///     let custom_error: Error = utf8_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables seamless error propagation using the `?` operator
/// - Useful for operations that work directly with byte slices and require UTF-8 validation
#[allow(clippy::absolute_paths)]
impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::new("parse UTF-8 string".to_owned(), err.to_string())
    }
}

/// Converts an integer parsing error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::num::ParseIntError`
/// to the custom `Error` type, handling errors that occur when parsing strings
/// into integer types.
///
/// # Arguments
///
/// * `err` - The source `ParseIntError` encountered during integer parsing
///
/// # Returns
///
/// A new `Error` instance with an integer parsing context and the specific
/// error message from the original parsing error.
///
/// # Examples
///
/// ```
/// let result = "not_a_number".parse::<i32>();
/// if let Err(parse_error) = result {
///     let custom_error: Error = parse_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables the use of the `?` operator for error propagation in parsing operations
/// - Handles parsing errors for all integer types (i32, u64, etc.)
impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::new("parse integer".to_owned(), err.to_string())
    }
}

/// Converts a floating-point parsing error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::num::ParseFloatError`
/// to the custom `Error` type, handling errors that occur when parsing strings
/// into floating-point types.
///
/// # Arguments
///
/// * `err` - The source `ParseFloatError` encountered during float parsing
///
/// # Returns
///
/// A new `Error` instance with a float parsing context and the specific
/// error message from the original parsing error.
///
/// # Examples
///
/// ```
/// let result = "not_a_number".parse::<f64>();
/// if let Err(parse_error) = result {
///     let custom_error: Error = parse_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables the use of the `?` operator for error propagation in parsing operations
/// - Handles parsing errors for all floating-point types (f32, f64)
impl From<ParseFloatError> for Error {
    fn from(err: ParseFloatError) -> Self {
        Error::new("parse float".to_owned(), err.to_string())
    }
}

/// Converts a YAML parsing error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `serde_yaml::Error`
/// to the custom `Error` type, handling errors that occur during YAML
/// serialization and deserialization operations.
///
/// # Arguments
///
/// * `err` - The source `serde_yaml::Error` encountered during YAML processing
///
/// # Returns
///
/// A new `Error` instance with a YAML parsing context and the specific
/// error message from the original YAML error.
///
/// # Examples
///
/// ```
/// let yaml_str = "invalid: : yaml";
/// let result: Result<Value, _> = serde_yaml::from_str(yaml_str);
/// if let Err(yaml_error) = result {
///     let custom_error: Error = yaml_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables seamless error propagation using the `?` operator
/// - Handles both serialization and deserialization errors
impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Self {
        Error::new("parse YAML".to_owned(), err.to_string())
    }
}

/// Converts a logger initialization error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `log::SetLoggerError`
/// to the custom `Error` type, handling errors that occur when setting up
/// or configuring the logging system.
///
/// # Arguments
///
/// * `err` - The source `SetLoggerError` encountered during logger initialization
///
/// # Returns
///
/// A new `Error` instance with a logger setup context and the specific
/// error message from the original logger error.
///
/// # Examples
///
/// ```
/// use log::{SetLoggerError, Logger};
/// let result = log::set_logger(&MY_LOGGER);
/// if let Err(logger_error) = result {
///     let custom_error: Error = logger_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables error handling during logging system initialization
/// - Typically occurs when attempting to set a logger more than once
impl From<SetLoggerError> for Error {
    fn from(err: SetLoggerError) -> Self {
        Error::new("set logger".to_owned(), err.to_string())
    }
}

/// Converts a system time error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `std::time::SystemTimeError`
/// to the custom `Error` type, handling errors that occur when performing
/// operations with system time.
///
/// # Arguments
///
/// * `err` - The source `SystemTimeError` encountered during time operations
///
/// # Returns
///
/// A new `Error` instance with a system time context and the specific
/// error message from the original time error.
///
/// # Examples
///
/// ```
/// use std::time::SystemTime;
/// let now = SystemTime::now();
/// let result = now.duration_since(SystemTime::now());
/// if let Err(time_error) = result {
///     let custom_error: Error = time_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables error handling for system time operations
/// - Typically occurs when computing duration between time points
impl From<SystemTimeError> for Error {
    fn from(err: SystemTimeError) -> Self {
        Error::new("get system time".to_owned(), err.to_string())
    }
}

/// Converts a time parsing error into a custom `Error` type.
///
/// This implementation allows automatic conversion from `chrono::ParseError`
/// to the custom `Error` type, handling errors that occur when parsing
/// strings into datetime values.
///
/// # Arguments
///
/// * `err` - The source `ParseError` encountered during time parsing
///
/// # Returns
///
/// A new `Error` instance with a time parsing context and the specific
/// error message from the original parsing error.
///
/// # Examples
///
/// ```
/// use chrono::DateTime;
/// let result = "invalid_date".parse::<DateTime<Utc>>();
/// if let Err(parse_error) = result {
///     let custom_error: Error = parse_error.into();
/// }
/// ```
///
/// # Notes
///
/// - Enables seamless error propagation using the `?` operator
/// - Handles parsing errors for various datetime formats
impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::new("parse time".to_owned(), err.to_string())
    }
}

/// Converts a String into a custom `Error` type.
///
/// This implementation allows automatic conversion from `String`
/// to the custom `Error` type, enabling the use of string messages
/// as errors directly.
///
/// # Arguments
///
/// * `err` - The source `String` to be used as an error message
///
/// # Returns
///
/// A new `Error` instance with a generic operation context and the
/// provided string as the error message.
///
/// # Examples
///
/// ```
/// let custom_error: Error = "Something went wrong".to_string().into();
/// ```
///
/// # Notes
///
/// - Useful for creating errors from dynamic string messages
/// - Provides a generic "perform operation" context
impl From<String> for Error {
    fn from(err: String) -> Self {
        Error::new("perform operation".to_owned(), err)
    }
}

/// Converts a string slice into a custom `Error` type.
///
/// This implementation allows automatic conversion from `&str`
/// to the custom `Error` type, enabling the use of string literals
/// as errors directly.
///
/// # Arguments
///
/// * `err` - The source string slice to be used as an error message
///
/// # Returns
///
/// A new `Error` instance with a generic operation context and the
/// provided string slice as the error message.
///
/// # Examples
///
/// ```
/// let custom_error: Error = "Something went wrong".into();
/// ```
///
/// # Notes
///
/// - Convenient for using string literals as errors
/// - Provides a generic "perform operation" context
impl From<&str> for Error {
    fn from(err: &str) -> Self {
        Error::new("perform operation".to_owned(), err.to_owned())
    }
}

/// Converts an Infallible error into a custom `Error` type.
///
/// This implementation exists for completeness but will never actually be called,
/// as `Infallible` represents computations that cannot fail and has no instances.
///
/// # Arguments
///
/// * `err` - The source `Infallible` error (which can never exist)
///
/// # Returns
///
/// This function never returns as it's impossible to construct an `Infallible`.
///
/// # Examples
///
/// ```
/// // This code can never actually create an Infallible error
/// let result: Result<(), Infallible> = Ok(());
/// if let Err(infallible) = result {
///     let custom_error: Error = infallible.into(); // Unreachable
/// }
/// ```
///
/// # Notes
///
/// - Implementation required for completeness in the type system
/// - Never actually used at runtime
impl From<Infallible> for Error {
    fn from(err: Infallible) -> Self {
        match err {}
    }
}

/// Converts any boxed error into a custom `Error` type.
///
/// This implementation allows automatic conversion from any boxed error type
/// that implements `std::error::Error` into the custom `Error` type, providing
/// a catch-all conversion for error types not explicitly handled.
///
/// # Arguments
///
/// * `err` - The source boxed error implementing `std::error::Error`
///
/// # Returns
///
/// A new `Error` instance with a generic operation context and the
/// string representation of the boxed error as the message.
///
/// # Examples
///
/// ```
/// let custom_error = Box::new(std::io::Error::new(
///     std::io::ErrorKind::Other,
///     "Custom error"
/// ));
/// let converted: Error = custom_error.into();
/// ```
///
/// # Notes
///
/// - Provides a fallback for error types not explicitly supported
/// - Enables working with trait objects and dynamic error types
#[allow(clippy::absolute_paths)]
impl<E> From<Box<E>> for Error
where
    E: std::error::Error + 'static,
{
    fn from(err: Box<E>) -> Self {
        Error::new("perform operation".to_owned(), err.to_string())
    }
}

/// Converts a Result<Infallible, String> into a custom `Error` type.
///
/// This implementation allows automatic conversion from `Result<Infallible, String>`
/// to the custom `Error` type, enabling seamless error propagation with the `?` operator
/// when working with string-based errors.
///
/// # Arguments
///
/// * `err` - The source `Result<Infallible, String>` to be converted
///
/// # Returns
///
/// A new `Error` instance with a generic operation context and the
/// string error message from the Result.
///
/// # Examples
///
/// ```
/// let result: Result<Infallible, String> = Err("Operation failed".to_string());
/// let custom_error: Error = result.into();
/// ```
///
/// # Notes
///
/// - Enables the use of the `?` operator with string-based errors
/// - Provides a generic "perform operation" context
impl From<Result<Infallible, String>> for Error {
    fn from(err: Result<Infallible, String>) -> Self {
        match err {
            Ok(infallible) => match infallible {},
            Err(msg) => Error::new("perform operation".to_owned(), msg),
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
