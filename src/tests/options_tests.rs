use crate::LoggerOptions;

#[test]
fn default_has_none_values() {
    // Arrange & Act
    let options = LoggerOptions::default();

    // Assert
    assert!(options.verbosity.is_none());
    assert!(options.log_time_format.is_none());
    assert!(options.log_include_filters.is_none());
    assert!(options.log_exclude_filters.is_none());
}
