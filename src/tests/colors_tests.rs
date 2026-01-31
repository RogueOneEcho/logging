use super::test_helpers::force_truecolor;
use crate::Colors;
use colored::{ColoredString, Colorize};
use insta::assert_snapshot;

const TEXT: &str = "Hello, world!";

#[test]
fn gray_applies_to_str() {
    // Arrange
    force_truecolor();

    // Act
    let result = TEXT.gray();

    // Assert
    assert_snapshot!(result.to_string());
}

#[test]
fn gray_applies_to_string() {
    // Arrange
    force_truecolor();

    // Act
    let result = TEXT.to_owned().gray();

    // Assert
    assert_snapshot!(result.to_string());
}

#[test]
fn dark_gray_applies_to_str() {
    // Arrange
    force_truecolor();

    // Act
    let result = TEXT.dark_gray();

    // Assert
    assert_snapshot!(result.to_string());
}

#[test]
fn dark_gray_applies_to_colored_string() {
    // Arrange
    force_truecolor();
    let colored: ColoredString = TEXT.blue();

    // Act
    let result = colored.dark_gray();

    // Assert
    assert_snapshot!(result.to_string());
}
