use crate::Error;
use insta::{assert_snapshot, assert_yaml_snapshot};

#[test]
fn serialize_error() {
    // Arrange
    let error = Error {
        action: "perform action".to_owned(),
        message: "Something went wrong".to_owned(),
        ..Error::default()
    };

    // Act & Assert
    assert_yaml_snapshot!(error);
}

#[test]
fn serialize_error_with_domain() {
    // Arrange
    let error = Error {
        action: "perform action".to_owned(),
        message: "Something went wrong".to_owned(),
        domain: Some("test".to_owned()),
        ..Error::default()
    };

    // Act & Assert
    assert_yaml_snapshot!(error);
}

#[test]
fn display_returns_multiline_string() {
    // Arrange
    let error = Error {
        action: "load config".to_owned(),
        message: "File not found".to_owned(),
        domain: Some("io".to_owned()),
        ..Error::default()
    };

    // Act
    let display = error.display();

    // Assert
    assert_snapshot!(display);
}

#[test]
fn clone_loses_backtrace() {
    // Arrange
    let error = Error {
        action: "test".to_owned(),
        message: "test".to_owned(),
        ..Error::default()
    };

    // Act
    let cloned = error.clone();

    // Assert
    assert!(cloned.backtrace.is_none());
}

#[test]
fn debug_matches_display() {
    // Arrange
    let error = Error {
        action: "test action".to_owned(),
        message: "test message".to_owned(),
        ..Error::default()
    };

    // Act
    let debug_output = format!("{error:?}");
    let display_output = format!("{error}");

    // Assert
    assert_eq!(debug_output, display_output);
}

#[test]
fn default_has_empty_strings() {
    // Arrange & Act
    let error = Error::default();

    // Assert
    assert!(error.action.is_empty());
    assert!(error.message.is_empty());
    assert!(error.domain.is_none());
    assert!(error.status_code.is_none());
}
