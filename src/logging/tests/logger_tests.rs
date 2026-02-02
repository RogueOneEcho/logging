use super::test_helpers::force_truecolor;
use crate::{LoggerBuilder, TimeFormat, Verbosity};
use insta::assert_snapshot;
use log::{Level, Metadata};

#[test]
fn format_log_with_time_format_local() {
    // Arrange
    force_truecolor();
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Local)
        .without_init()
        .create();

    // Act
    let output = logger.format_log(Verbosity::Info, "This is an info message".to_owned());

    // Assert
    eprintln!("{output}");
    assert!(output.contains("INFO"));
    assert!(output.contains("This is an info message"));
}

#[test]
fn format_log_with_time_format_utc() {
    // Arrange
    force_truecolor();
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Utc)
        .without_init()
        .create();

    // Act
    let output = logger.format_log(Verbosity::Warn, "This is a warning message".to_owned());

    // Assert
    eprintln!("{output}");
    assert!(output.contains("WARN"));
    assert!(output.contains("This is a warning message"));
}

#[test]
fn format_log_with_time_format_elapsed() {
    // Arrange
    force_truecolor();
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Elapsed)
        .without_init()
        .create();

    // Act
    let output = logger.format_log(Verbosity::Error, "This is an error message".to_owned());

    // Assert
    eprintln!("{output}");
    assert!(output.contains("ERRO"));
    assert!(output.contains("This is an error message"));
}

#[test]
fn format_log_with_time_format_none() {
    // Arrange
    force_truecolor();
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::None)
        .without_init()
        .create();

    // Act
    let error = logger.format_log(Verbosity::Error, "This is an error message".to_owned());
    let warn = logger.format_log(Verbosity::Warn, "This is a warning message".to_owned());
    let info = logger.format_log(Verbosity::Info, "This is an info message".to_owned());
    let debug = logger.format_log(Verbosity::Debug, "This is a debug message".to_owned());
    let trace = logger.format_log(Verbosity::Trace, "This is a trace message".to_owned());

    // Assert
    eprintln!("{error}");
    eprintln!("{warn}");
    eprintln!("{info}");
    eprintln!("{debug}");
    eprintln!("{trace}");
    assert_snapshot!(error);
    assert_snapshot!(warn);
    assert_snapshot!(info);
    assert_snapshot!(debug);
    assert_snapshot!(trace);
}

#[test]
fn enabled_excludes_target_with_exclude_filter() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_exclude_filter("noisy_crate".to_owned())
        .without_init()
        .create();
    let metadata = Metadata::builder()
        .level(Level::Info)
        .target("noisy_crate::module")
        .build();

    // Act
    let enabled = log::Log::enabled(logger.as_ref(), &metadata);

    // Assert
    assert!(!enabled);
}

#[test]
fn enabled_includes_target_without_matching_filter() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_exclude_filter("noisy_crate".to_owned())
        .without_init()
        .create();
    let metadata = Metadata::builder()
        .level(Level::Info)
        .target("my_crate::module")
        .build();

    // Act
    let enabled = log::Log::enabled(logger.as_ref(), &metadata);

    // Assert
    assert!(enabled);
}

#[test]
fn enabled_excludes_target_not_in_include_filter() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_include_filter("allowed_crate".to_owned())
        .without_init()
        .create();
    let metadata = Metadata::builder()
        .level(Level::Info)
        .target("other_crate::module")
        .build();

    // Act
    let enabled = log::Log::enabled(logger.as_ref(), &metadata);

    // Assert
    assert!(!enabled);
}

#[test]
fn enabled_excludes_verbosity_above_threshold() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_verbosity(Verbosity::Info)
        .without_init()
        .create();
    let metadata = Metadata::builder()
        .level(Level::Debug)
        .target("test")
        .build();

    // Act
    let enabled = log::Log::enabled(logger.as_ref(), &metadata);

    // Assert
    assert!(!enabled);
}

#[test]
fn enabled_includes_verbosity_at_threshold() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_verbosity(Verbosity::Info)
        .without_init()
        .create();
    let metadata = Metadata::builder()
        .level(Level::Info)
        .target("test")
        .build();

    // Act
    let enabled = log::Log::enabled(logger.as_ref(), &metadata);

    // Assert
    assert!(enabled);
}

#[test]
fn format_prefix_contains_verbosity_id() {
    // Arrange
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::None)
        .without_init()
        .create();

    // Act
    let prefix = logger.format_prefix(Verbosity::Info);

    // Assert
    assert!(prefix.contains("INFO"));
}
