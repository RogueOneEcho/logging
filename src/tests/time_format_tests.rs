use crate::TimeFormat;

#[test]
fn default_is_local() {
    // Arrange & Act
    let default = TimeFormat::default();

    // Assert
    assert!(matches!(default, TimeFormat::Local));
}
