//! Error wrapper implementing `miette::Diagnostic` for rich error reporting.

use super::Error;
use miette::{Diagnostic, Severity};
use std::any::type_name;
use std::error::Error as StdError;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::path::Path;

/// Marker trait for action types that can be used with [`Failure`].
pub trait Action: Debug + Display {}

impl<T: Debug + Display> Action for T {}

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
    /// ```text
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
    /// ```text
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

    /// Returns a closure for use with `map_err`, adding path context.
    ///
    /// # Example
    ///
    /// ```text
    /// read_to_string(path).map_err(Failure::wrap_with_path(Action::ReadFile, path))?;
    /// ```
    pub fn wrap_with_path<E>(action: T, path: impl AsRef<Path>) -> impl FnOnce(E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        let path = path.as_ref().display().to_string();
        move |e| Self::new(action, e).with("path", path)
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

impl<T: Action> Failure<T> {
    fn display_additional(&self) -> String {
        self.additional
            .iter()
            .fold(String::new(), |mut acc, (k, v)| {
                use std::fmt::Write;
                let line = format!("▷ {k}: {v}");
                #[cfg(feature = "miette-fancy")]
                let line = {
                    use owo_colors::{OwoColorize, Stream};
                    line.if_supports_color(Stream::Stdout, |text| text.dimmed())
                        .to_string()
                };
                let _ = write!(acc, "\n{line}");
                acc
            })
    }
}

impl<T: Action> Display for Failure<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "Failed to {}", self.action)?;
        if !self.additional.is_empty() {
            write!(f, "{}", self.display_additional())?;
        }
        Ok(())
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

impl<T: Action> AsRef<T> for Failure<T> {
    fn as_ref(&self) -> &T {
        &self.action
    }
}

impl<T: Action> Diagnostic for Failure<T> {
    fn code<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        Some(Box::new(
            self.code
                .clone()
                .unwrap_or_else(|| short_code::<T>(&self.action)),
        ))
    }

    fn severity(&self) -> Option<Severity> {
        self.severity
    }

    #[expect(
        clippy::as_conversions,
        reason = "cast from boxed struct to trait object"
    )]
    fn help<'a>(&'a self) -> Option<Box<dyn Display + 'a>> {
        self.help.as_ref().map(|h| Box::new(Displayable(h)) as _)
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

/// Build a short diagnostic code from `type_name::<T>()` and the action's `Debug` output.
///
/// - Enum actions: `crate::EnumName::Variant`
/// - Struct actions at crate root (`crate::Struct`): `crate::Struct`
/// - Struct actions with a parent module (`crate::module::Struct`): `crate::module::Struct`
fn short_code<T: Action>(action: &T) -> String {
    let full = type_name::<T>();
    let segments: Vec<&str> = full.split("::").collect();
    let crate_name = segments.first().unwrap_or(&full);
    let type_name_segment = segments.last().unwrap_or(&full);
    let debug = format!("{action:?}");
    let first_word = debug.split([' ', '(', '{']).next().unwrap_or(&debug);
    let is_struct = first_word == *type_name_segment;
    if is_struct {
        if segments.len() > 2 {
            #[expect(clippy::indexing_slicing, reason = "len > 2 is checked")]
            let parent = segments[segments.len() - 2];
            format!("{crate_name}::{parent}::{type_name_segment}")
        } else {
            format!("{crate_name}::{type_name_segment}")
        }
    } else {
        format!("{crate_name}::{type_name_segment}::{first_word}")
    }
}

type BoxedError = Box<dyn StdError + Send + Sync>;

type BoxedDiagnostic = Box<dyn Diagnostic + Send + Sync>;

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;
    use std::fmt::Write;
    use std::io;

    #[derive(Debug)]
    enum SimpleEnum {
        Read,
        Write,
    }
    impl Display for SimpleEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Self::Read => write!(f, "read"),
                Self::Write => write!(f, "write"),
            }
        }
    }

    #[derive(Debug)]
    enum TupleEnum {
        Download(String),
    }
    impl Display for TupleEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Self::Download(url) => write!(f, "download {url}"),
            }
        }
    }

    #[derive(Debug)]
    enum StructEnum {
        Connect { host: String },
    }
    impl Display for StructEnum {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            match self {
                Self::Connect { host } => write!(f, "connect to {host}"),
            }
        }
    }

    #[derive(Debug)]
    enum SingleVariant {
        Only,
    }
    impl Display for SingleVariant {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "only")
        }
    }

    #[derive(Debug)]
    struct UnitStruct;
    impl Display for UnitStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "unit action")
        }
    }

    #[derive(Debug)]
    struct FieldStruct {
        _msg: String,
    }
    impl Display for FieldStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "field action")
        }
    }

    #[derive(Debug)]
    struct TupleStruct(#[expect(dead_code)] String);
    impl Display for TupleStruct {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            write!(f, "tuple action")
        }
    }

    #[expect(clippy::unwrap_used)]
    #[test]
    fn short_code_snapshot() {
        let mut out = String::new();
        let mut line = |label: &str, code: &str| writeln!(out, "{label:<40} => {code}").unwrap();
        // Enum — unit variants
        line(
            "SimpleEnum::Read",
            &short_code::<SimpleEnum>(&SimpleEnum::Read),
        );
        line(
            "SimpleEnum::Write",
            &short_code::<SimpleEnum>(&SimpleEnum::Write),
        );
        // Enum — tuple variant (payload must not leak)
        line(
            "TupleEnum::Download(url)",
            &short_code::<TupleEnum>(&TupleEnum::Download("https://example.com".into())),
        );
        // Enum — struct variant (fields must not leak)
        line(
            "StructEnum::Connect { host }",
            &short_code::<StructEnum>(&StructEnum::Connect {
                host: "localhost".into(),
            }),
        );
        // Enum — single variant
        line(
            "SingleVariant::Only",
            &short_code::<SingleVariant>(&SingleVariant::Only),
        );
        // Struct — unit
        line("UnitStruct", &short_code::<UnitStruct>(&UnitStruct));
        // Struct — with fields (values must not leak)
        line(
            "FieldStruct { _msg }",
            &short_code::<FieldStruct>(&FieldStruct {
                _msg: "secret".into(),
            }),
        );
        // Struct — tuple (values must not leak)
        line(
            "TupleStruct(data)",
            &short_code::<TupleStruct>(&TupleStruct("secret".into())),
        );
        // String action (edge case — alloc::string::String)
        line(
            "String(\"do something\")",
            &short_code::<String>(&String::from("do something")),
        );
        // Custom code override
        line(
            "with_code override",
            &Failure::new(SimpleEnum::Read, io::Error::other("e"))
                .with_code("my::custom::code")
                .code()
                .unwrap()
                .to_string(),
        );
        // from_action (no source)
        line(
            "from_action enum",
            &Failure::from_action(SimpleEnum::Write)
                .code()
                .unwrap()
                .to_string(),
        );
        line(
            "from_action struct",
            &Failure::from_action(UnitStruct).code().unwrap().to_string(),
        );
        assert_snapshot!(out);
    }
}
