use crate::{LoggerBuilder, LoggerOptions, TimeFormat, Verbosity};

#[test]
fn new_creates_builder_with_defaults() {
    // Arrange & Act
    let logger = LoggerBuilder::new().create();

    // Assert
    assert_eq!(logger.options.verbosity, None);
    assert_eq!(logger.options.log_time_format, None);
    assert_eq!(logger.options.log_include_filters, None);
    assert_eq!(logger.options.log_exclude_filters, None);
}

#[test]
fn with_options_sets_all_options() {
    // Arrange
    let options = LoggerOptions {
        verbosity: Some(Verbosity::Debug),
        log_time_format: Some(TimeFormat::Utc),
        log_include_filters: Some(vec!["foo".to_owned()]),
        log_exclude_filters: Some(vec!["bar".to_owned()]),
    };

    // Act
    let logger = LoggerBuilder::new().with_options(options).create();

    // Assert
    assert_eq!(logger.options.verbosity, Some(Verbosity::Debug));
    assert_eq!(logger.options.log_time_format, Some(TimeFormat::Utc));
    assert_eq!(
        logger.options.log_include_filters,
        Some(vec!["foo".to_owned()])
    );
    assert_eq!(
        logger.options.log_exclude_filters,
        Some(vec!["bar".to_owned()])
    );
}

#[test]
fn with_verbosity_sets_verbosity() {
    // Arrange & Act
    let logger = LoggerBuilder::new()
        .with_verbosity(Verbosity::Trace)
        .create();

    // Assert
    assert_eq!(logger.options.verbosity, Some(Verbosity::Trace));
}

#[test]
fn with_time_format_sets_time_format() {
    // Arrange & Act
    let logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Elapsed)
        .create();

    // Assert
    assert_eq!(logger.options.log_time_format, Some(TimeFormat::Elapsed));
}

#[test]
fn with_include_filter_adds_filter() {
    // Arrange & Act
    let logger = LoggerBuilder::new()
        .with_include_filter("my_crate".to_owned())
        .create();

    // Assert
    assert_eq!(
        logger.options.log_include_filters,
        Some(vec!["my_crate".to_owned()])
    );
}

#[test]
fn with_include_filter_accumulates() {
    // Arrange & Act
    let logger = LoggerBuilder::new()
        .with_include_filter("crate_a".to_owned())
        .with_include_filter("crate_b".to_owned())
        .create();

    // Assert
    assert_eq!(
        logger.options.log_include_filters,
        Some(vec!["crate_a".to_owned(), "crate_b".to_owned()])
    );
}

#[test]
fn with_exclude_filter_adds_filter() {
    // Arrange & Act
    let logger = LoggerBuilder::new()
        .with_exclude_filter("noisy_crate".to_owned())
        .create();

    // Assert
    assert_eq!(
        logger.options.log_exclude_filters,
        Some(vec!["noisy_crate".to_owned()])
    );
}
