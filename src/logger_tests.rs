use crate::*;
use log::*;

fn example_logs() {
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");
}

#[test]
#[ignore]
fn logger_with_time_format_local() {
    // Arrange
    let _logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Local)
        .create();

    // Act
    example_logs();
}

#[test]
#[ignore]
fn logger_with_time_format_utc() {
    // Arrange
    let _logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Utc)
        .create();

    // Act
    example_logs();
}

#[test]
#[ignore]
fn logger_with_time_format_elapsed() {
    // Arrange
    let _logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::Elapsed)
        .create();

    // Act
    example_logs();
}

#[test]
#[ignore]
fn logger_with_time_format_none() {
    // Arrange
    let _logger = LoggerBuilder::new()
        .with_time_format(TimeFormat::None)
        .create();

    // Act
    example_logs();
}

#[test]
#[ignore]
fn logger_with_exclude_filter() {
    // Arrange
    let _logger = LoggerBuilder::new()
        .with_exclude_filter("rogue_logging".to_owned())
        .create();

    // Act
    example_logs();
}
