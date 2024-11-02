use crate::Error;

#[test]
fn serialize_error() {
    // Arrange
    let error = Error {
        action: "perform action".to_owned(),
        message: "Something went wrong".to_owned(),
        ..Error::default()
    };

    // Act
    let yaml = serde_yaml::to_string(&error).unwrap();

    // Assert
    let expected = "action: perform action
message: Something went wrong
";
    assert_eq!(yaml, expected);
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

    // Act
    let yaml_output = serde_yaml::to_string(&error).unwrap();

    // Assert
    let expected_output = "action: perform action
message: Something went wrong
domain: test
";
    assert_eq!(yaml_output, expected_output);
}
