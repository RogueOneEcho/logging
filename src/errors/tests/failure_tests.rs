use crate::errors::tests::test_helpers::*;
use crate::{Failure, Severity};
use insta::assert_snapshot;
use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme, NarratableReportHandler};
use std::error::Error as StdError;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

#[test]
fn display_shows_action() {
    use_colors(false);
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    assert_snapshot!(failure.to_string());
}

#[test]
fn display_with_additional_context() {
    use_colors(false);
    let failure = Failure::new(TestAction::ReadConfig, io_error())
        .with("path", "/etc/config.yaml")
        .with("attempt", "3");
    assert_snapshot!(failure.to_string());
}

#[test]
fn with_path_adds_path_context() {
    let failure = Failure::new(TestAction::WriteFile, io_error()).with_path("/tmp/output.txt");
    assert_eq!(failure.get("path"), Some("/tmp/output.txt".to_owned()));
}

#[test]
fn wrap_with_path_adds_path_context() {
    let result: Result<(), _> = Err(io_error());
    let failure = result
        .map_err(Failure::wrap_with_path(
            TestAction::WriteFile,
            "/tmp/output.txt",
        ))
        .expect_err("should be Err");
    assert_eq!(failure.get("path"), Some("/tmp/output.txt".to_owned()));
}

#[test]
fn get_returns_none_for_missing_key() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    assert!(failure.get("nonexistent").is_none());
}

#[test]
fn get_returns_value_for_existing_key() {
    let failure = Failure::new(TestAction::ReadConfig, io_error()).with("key", "value");
    assert_eq!(failure.get("key"), Some("value".to_owned()));
}

#[test]
fn set_updates_existing_key() {
    let failure = Failure::new(TestAction::ReadConfig, io_error())
        .with("key", "original")
        .set("key", "updated");
    assert_eq!(failure.get("key"), Some("updated".to_owned()));
}

#[test]
fn set_adds_new_key_if_missing() {
    let failure = Failure::new(TestAction::ReadConfig, io_error()).set("new_key", "new_value");
    assert_eq!(failure.get("new_key"), Some("new_value".to_owned()));
}

#[test]
fn source_returns_underlying_error() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    let source = failure.source().expect("should have source");
    assert_eq!(source.to_string(), "file not found");
}

#[test]
fn to_error_converts_correctly() {
    let failure = Failure::new(TestAction::LoadConfig, io_error()).with("domain", "configuration");
    let error = failure.to_error();
    assert_eq!(error.action, "load config");
    assert_eq!(error.message, "file not found");
    assert_eq!(error.domain, Some("configuration".to_owned()));
}

#[test]
fn to_error_uses_type_name_when_no_domain() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    let error = failure.to_error();
    let domain = error.domain.expect("domain should be set");
    assert!(domain.contains("TestAction"));
}

#[test]
fn diagnostic_code_returns_type_path() {
    let failure = Failure::new(TestAction::ParseJson, io_error());
    let code = failure.code().expect("should have code");
    assert!(code.to_string().ends_with("TestAction::ParseJson"));
}

#[test]
fn diagnostic_code_returns_custom_code() {
    let failure = Failure::new(TestAction::ParseJson, io_error()).with_code("custom::code");
    let code = failure.code().expect("should have code");
    assert_eq!(code.to_string(), "custom::code");
}

#[test]
fn diagnostic_help_returns_help_text() {
    let failure =
        Failure::new(TestAction::Connect, io_error()).with_help("Check your network connection");
    let help = failure.help().expect("should have help");
    assert_eq!(help.to_string(), "Check your network connection");
}

#[test]
fn diagnostic_url_returns_url() {
    let failure = Failure::new(TestAction::Authenticate, io_error())
        .with_url("https://docs.example.com/auth");
    let url = failure.url().expect("should have url");
    assert_eq!(url.to_string(), "https://docs.example.com/auth");
}

#[test]
fn diagnostic_related_returns_none_when_empty() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    assert!(failure.related().is_none());
}

#[test]
fn diagnostic_severity_returns_severity() {
    let failure = Failure::new(TestAction::ReadConfig, io_error()).with_severity(Severity::Warning);
    assert_eq!(failure.severity(), Some(Severity::Warning));
}

#[test]
fn diagnostic_severity_defaults_to_none() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    assert!(failure.severity().is_none());
}

#[test]
fn debug_impl_works() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    let debug = format!("{failure:?}");
    assert!(debug.contains("Failure"));
    assert!(debug.contains("ReadConfig"));
}

#[test]
fn chained_builder_methods() {
    use_colors(false);
    let failure = Failure::new(TestAction::UploadFile, io_error())
        .with_path("/data/file.bin")
        .with("size", "1024")
        .with("retry", "true");
    assert_snapshot!(failure.to_string());
}

#[test]
fn to_error_snapshot() {
    use_colors(false);
    let failure = Failure::new(TestAction::FetchData, io_error())
        .with("domain", "network")
        .with("endpoint", "/api/v1/data");
    let error = failure.to_error();
    assert_snapshot!(error.display());
}

fn render_diagnostic(diagnostic: &dyn Diagnostic) -> String {
    let mut output = String::new();
    let handler =
        GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor()).with_links(false);
    handler
        .render_report(&mut output, diagnostic)
        .expect("should render diagnostic");
    output
}

#[test]
fn miette_render_basic() {
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_render_with_help() {
    let failure =
        Failure::new(TestAction::Connect, io_error()).with_help("Check that the server is running");
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_render_with_url() {
    let failure = Failure::new(TestAction::Authenticate, io_error())
        .with_url("https://docs.example.com/auth");
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_render_with_context() {
    use_colors(false);
    let inner = Failure::new(TestAction::ParseConfigFile, io_error())
        .with_path("/etc/myapp/config.yaml")
        .with("line", "42");
    let failure = Failure::new(TestAction::LoadConfig, inner)
        .with("format", "yaml")
        .with("env", "production");
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_render_with_warning_severity() {
    let failure = Failure::new(TestAction::LoadConfig, io_error())
        .with_severity(Severity::Warning)
        .with_help("Using default configuration");
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_render_with_related() {
    use_colors(false);
    let fallback_api = {
        let conn_err = IoError::new(IoErrorKind::ConnectionRefused, "connection refused");
        Failure::new(HttpAction::CacheUsers, conn_err)
            .with("url", "https://fallback.example.com/users")
            .with_help("Fallback API is unavailable")
    };
    let local_cache = {
        let not_found = IoError::new(IoErrorKind::NotFound, "file not found");
        Failure::new(HttpAction::CacheUsers, not_found)
            .with_path("/var/cache/users.json")
            .with_severity(Severity::Warning)
            .with_help("Local cache is missing")
    };
    let failure = Failure::new(TestAction::GetAllUsers, http_error())
        .with_code("users::fetch::failed")
        .with_help("All sources failed")
        .with_url("https://docs.example.com/users")
        .with_related(fallback_api)
        .with_related(local_cache);
    assert_snapshot!(render_diagnostic(&failure));
}

#[test]
fn miette_graphical_vs_narratable() {
    use_colors(false);

    // Arrange
    let failure = http_error();

    // Act
    let graphical = render_diagnostic(&failure);
    let mut narratable = String::new();
    NarratableReportHandler::new()
        .render_report(&mut narratable, &failure)
        .expect("should render");

    // Assert
    let snapshot = format!("{graphical}\n{}\n\n{narratable}", "-".repeat(80));
    assert_snapshot!(snapshot);
}

#[test]
#[ignore = "modifies global color state, run manually with --ignored"]
#[cfg(feature = "miette-fancy")]
fn miette_graphical_with_colors() {
    // Arrange
    use_colors(true);
    let failure = http_error();

    // Act
    let mut graphical = String::new();
    GraphicalReportHandler::new_themed(GraphicalTheme::unicode())
        .with_links(false)
        .render_report(&mut graphical, &failure)
        .expect("should render diagnostic");
    use_colors(false);

    // Assert
    println!("\n{graphical}\n");
    let dimmed_context = "\x1b[0m\x1b[2m▷ url: https://api.example.com/users\x1b[0m";
    let count = graphical.matches(dimmed_context).count();
    assert_eq!(count, 1);
}
