use crate::Error;
use chrono::DateTime;
use std::convert::Infallible;
use std::fmt::Write;
use std::io;
use std::str;
use std::time::SystemTime;

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

#[test]
fn test_from_io_error() {
    // Arrange
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");

    // Act
    let error: Error = io_error.into();

    // Assert
    assert_eq!(error.action, "perform I/O operation");
    assert!(error.message.contains("file not found"));
}

#[test]
fn test_from_utf8_error() {
    // Arrange
    let invalid_vec = vec![0xFF, 0xFF];
    let utf8_error = String::from_utf8(invalid_vec).unwrap_err();

    // Act
    let error: Error = utf8_error.into();

    // Assert
    assert_eq!(error.action, "convert bytes to UTF-8 string");
    assert!(error.message.contains("invalid utf-8"));
}

#[test]
fn test_from_parse_int_error() {
    // Arrange
    let parse_error = "not_a_number".parse::<i32>().unwrap_err();

    // Act
    let error: Error = parse_error.into();

    // Assert
    assert_eq!(error.action, "parse integer");
    assert!(error.message.contains("invalid digit"));
}

#[test]
fn test_from_parse_float_error() {
    // Arrange
    let parse_error = "not_a_float".parse::<f64>().unwrap_err();

    // Act
    let error: Error = parse_error.into();

    // Assert
    assert_eq!(error.action, "parse float");
    assert!(error.message.contains("invalid float"));
}

#[test]
fn test_from_system_time_error() {
    // Arrange
    let future_time = SystemTime::now();
    let past_time = SystemTime::now();
    let time_error = future_time.duration_since(past_time).unwrap_err();

    // Act
    let error: Error = time_error.into();

    // Assert
    assert_eq!(error.action, "get system time");
    assert!(error.message.contains("second time provided was later"));
}

#[test]
fn test_from_string() {
    // Arrange
    let string_error = String::from("custom error message");

    // Act
    let error: Error = string_error.into();

    // Assert
    assert_eq!(error.action, "perform operation");
    assert_eq!(error.message, "custom error message");
}

#[test]
fn test_from_str() {
    // Arrange
    let str_error = "custom error message";

    // Act
    let error: Error = str_error.into();

    // Assert
    assert_eq!(error.action, "perform operation");
    assert_eq!(error.message, "custom error message");
}

#[test]
fn test_from_boxed_error() {
    // Arrange
    struct CustomError(String);

    impl std::fmt::Display for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::fmt::Debug for CustomError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for CustomError {}

    let custom_error = CustomError("boxed error message".to_string());
    let boxed_error = Box::new(custom_error);

    // Act
    let error: Error = boxed_error.into();

    // Assert
    assert_eq!(error.action, "perform operation");
    assert!(error.message.contains("boxed error message"));
}

#[test]
fn test_from_str_utf8_error() {
    // Arrange
    // Use a multi-byte character (é) and get its bytes
    let s = "é".as_bytes();
    // Take only the first byte to create an incomplete UTF-8 sequence
    let utf8_error = str::from_utf8(&s[..1]).unwrap_err();

    // Act
    let error: Error = utf8_error.into();

    // Assert
    assert_eq!(error.action, "parse UTF-8 string");
    assert!(error.message.contains("incomplete utf-8"));
}

#[test]
fn test_from_fmt_error() {
    // Arrange
    struct FailingWriter;

    impl Write for FailingWriter {
        fn write_str(&mut self, _s: &str) -> std::fmt::Result {
            Err(std::fmt::Error)
        }
    }

    let mut writer = FailingWriter;
    let fmt_error = write!(writer, "test").unwrap_err();

    // Act
    let error: Error = fmt_error.into();

    // Assert
    assert_eq!(error.action, "format string");
    assert_eq!(
        error.message,
        "an error occurred when formatting an argument"
    );
}

#[test]
fn test_from_yaml_error() {
    // Arrange
    let invalid_yaml = "invalid: : yaml";
    let yaml_error = serde_yaml::from_str::<serde_yaml::Value>(invalid_yaml).unwrap_err();

    // Act
    let error: Error = yaml_error.into();

    // Assert
    assert_eq!(error.action, "parse YAML");
    assert!(!error.message.is_empty());
}

#[test]
fn test_from_chrono_parse_error() {
    // Arrange
    let invalid_date = "invalid_date";
    let parse_error = invalid_date.parse::<DateTime<chrono::Utc>>().unwrap_err();

    // Act
    let error: Error = parse_error.into();

    // Assert
    assert_eq!(error.action, "parse time");
    assert!(!error.message.is_empty());
}

#[test]
fn test_string_error_conversion() {
    // Test direct string conversion
    let err_str = "test error message";
    let error: Error = err_str.into();
    assert_eq!(error.message, err_str);
    assert_eq!(error.action, "perform operation");

    // Test String conversion
    let err_string = String::from("test error message");
    let error: Error = err_string.clone().into();
    assert_eq!(error.message, err_string);
    assert_eq!(error.action, "perform operation");

    // Test Result<Infallible, String> conversion
    let result: Result<Infallible, String> = Err("test error message".to_string());
    let error: Error = result.into();
    assert_eq!(error.message, "test error message");
    assert_eq!(error.action, "perform operation");
}

#[test]
fn test_error_propagation() {
    fn returns_string_error() -> Result<Infallible, String> {
        Err("Operation failed".to_string())
    }

    // Test that the error is properly propagated
    let result: Result<(), Error> = (|| {
        let _result = returns_string_error()?;
        Ok(())
    })();

    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.action, "perform operation");
    assert_eq!(error.message, "Operation failed");
}
