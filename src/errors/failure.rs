//! Error wrapper implementing `miette::Diagnostic` for rich error reporting.

use super::Error;
use miette::{Diagnostic, Severity};
use std::any::type_name;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::path::Path;

type BoxedError = Box<dyn StdError + Send + Sync>;

/// Marker trait for action types that can be used with [`Failure`].
pub trait Action: Debug + Display {}

impl<T: Debug + Display> Action for T {}

type BoxedDiagnostic = Box<dyn Diagnostic + Send + Sync>;

/// A wrapper that implements [`miette::Diagnostic`] for rich error reporting.
///
/// Each `Failure` wraps an action type `T` (which describes what operation failed)
/// along with the underlying error source and optional contextual information.
#[derive(Debug)]
pub struct Failure<T: Action> {
    action: T,
    code: Option<String>,
    help: Option<String>,
    url: Option<String>,
    severity: Option<Severity>,
    related: Vec<BoxedDiagnostic>,
    additional: Vec<(String, String)>,
    source: Option<BoxedError>,
}

impl<T: Action> Failure<T> {
    /// Create a new `Failure` with the given action and source error.
    pub fn new(action: T, source: impl StdError + Send + Sync + 'static) -> Self {
        Self {
            action,
            code: None,
            help: None,
            url: None,
            severity: None,
            related: Vec::new(),
            additional: Vec::new(),
            source: Some(Box::new(source)),
        }
    }

    /// Create a new `Failure` with only an action, no source error.
    pub fn from_action(action: T) -> Self {
        Self {
            action,
            code: None,
            help: None,
            url: None,
            severity: None,
            related: Vec::new(),
            additional: Vec::new(),
            source: None,
        }
    }

    /// Returns a closure for use with `map_err`.
    ///
    /// # Example
    ///
    /// ```ignore
    /// read_to_string("/etc/config.yaml").map_err(Failure::wrap(Action::ReadFile))?;
    /// ```
    pub fn wrap<E>(action: T) -> impl FnOnce(E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        move |e| Self::new(action, e)
    }

    /// Returns a closure for use with `map_err`, with builder configuration.
    ///
    /// # Example
    ///
    /// ```ignore
    /// read_to_string(path).map_err(Failure::wrap_with(Action::ReadFile, |f| {
    ///     f.with_path(path).with_help("Check file permissions")
    /// }))?;
    /// ```
    pub fn wrap_with<E, F>(action: T, configure: F) -> impl FnOnce(E) -> Self
    where
        E: StdError + Send + Sync + 'static,
        F: FnOnce(Self) -> Self,
    {
        move |e| configure(Self::new(action, e))
    }

    /// Get the action.
    #[must_use]
    pub fn action(&self) -> &T {
        &self.action
    }

    /// Get a value by key from additional context.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<String> {
        self.additional
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    }

    /// Set a key-value pair, updating if the key already exists.
    #[must_use]
    pub fn set(mut self, key: &str, value: impl Into<String>) -> Self {
        if let Some((_, v)) = self.additional.iter_mut().find(|(k, _)| k == key) {
            *v = value.into();
        } else {
            self.additional.push((key.to_owned(), value.into()));
        }
        self
    }

    /// Add a key-value pair of additional context.
    #[must_use]
    pub fn with(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.additional.push((key.into(), value.into()));
        self
    }

    /// Add a path to the additional context.
    #[must_use]
    pub fn with_path(self, path: impl AsRef<Path>) -> Self {
        self.with("path", path.as_ref().display().to_string())
    }

    /// Set the diagnostic code.
    ///
    /// Default: `module::path::Action::Variant`
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Set the help text.
    ///
    /// Default: `None`
    #[must_use]
    pub fn with_help(mut self, help: impl Into<String>) -> Self {
        self.help = Some(help.into());
        self
    }

    /// Set the URL for more information.
    ///
    /// Default: `None`
    #[must_use]
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// Set the severity level.
    ///
    /// Default: `Error`
    #[must_use]
    pub fn with_severity(mut self, severity: Severity) -> Self {
        self.severity = Some(severity);
        self
    }

    /// Add a related diagnostic.
    #[must_use]
    pub fn with_related(mut self, diagnostic: impl Diagnostic + Send + Sync + 'static) -> Self {
        self.related.push(Box::new(diagnostic));
        self
    }

    /// Convert to a serializable [`Error`].
    #[must_use]
    pub fn to_error(&self) -> Error {
        Error {
            action: self.action.to_string(),
            message: self
                .source
                .as_ref()
                .map_or_else(String::new, ToString::to_string),
            domain: self
                .get("domain")
                .or_else(|| Some(type_name::<T>().to_owned())),
            status_code: None,
            backtrace: None,
        }
    }
}

impl<T: Action> Display for Failure<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to {}", self.action)
    }
}

impl<T: Action> StdError for Failure<T> {
    #[expect(
        clippy::as_conversions,
        reason = "cast from boxed trait object to trait reference"
    )]
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_ref()
            .map(|s| s.as_ref() as &(dyn StdError + 'static))
    }
}

impl<T: Action> Diagnostic for Failure<T> {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(self.code.clone().unwrap_or_else(|| {
            format!("{}::{:?}", type_name::<T>(), self.action)
        })))
    }

    fn severity(&self) -> Option<Severity> {
        self.severity
    }

    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        if self.help.is_none() && self.additional.is_empty() {
            return None;
        }
        let mut result = String::new();
        if let Some(h) = &self.help {
            result.push_str(h);
        }
        for (key, value) in &self.additional {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str("▷ ");
            result.push_str(key);
            result.push_str(": ");
            result.push_str(value);
        }
        Some(Box::new(result))
    }

    #[expect(
        clippy::as_conversions,
        reason = "cast from boxed struct to trait object"
    )]
    fn url<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.url.as_ref().map(|u| Box::new(Displayable(u)) as _)
    }

    #[expect(
        clippy::as_conversions,
        reason = "cast from boxed trait object to trait reference"
    )]
    fn related<'a>(&'a self) -> Option<Box<dyn Iterator<Item = &'a dyn Diagnostic> + 'a>> {
        if self.related.is_empty() {
            None
        } else {
            Some(Box::new(self.related.iter().map(|d| d.as_ref() as _)))
        }
    }
}

struct Displayable<'a, T: Display>(&'a T);

impl<T: Display> Display for Displayable<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(self.0, f)
    }
}
