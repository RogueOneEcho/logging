use crate::Verbosity;
use log::{Level, LevelFilter};

#[test]
fn as_num_returns_ordered_values() {
    // Arrange & Act & Assert
    assert_eq!(Verbosity::Silent.as_num(), 0);
    assert_eq!(Verbosity::Error.as_num(), 1);
    assert_eq!(Verbosity::Warn.as_num(), 2);
    assert_eq!(Verbosity::Info.as_num(), 3);
    assert_eq!(Verbosity::Debug.as_num(), 4);
    assert_eq!(Verbosity::Trace.as_num(), 5);
}

#[test]
fn from_level_converts_correctly() {
    // Arrange & Act & Assert
    assert_eq!(Verbosity::from_level(Level::Error), Verbosity::Error);
    assert_eq!(Verbosity::from_level(Level::Warn), Verbosity::Warn);
    assert_eq!(Verbosity::from_level(Level::Info), Verbosity::Info);
    assert_eq!(Verbosity::from_level(Level::Debug), Verbosity::Debug);
    assert_eq!(Verbosity::from_level(Level::Trace), Verbosity::Trace);
}

#[test]
fn to_level_filter_converts_correctly() {
    // Arrange & Act & Assert
    assert_eq!(Verbosity::Silent.to_level_filter(), LevelFilter::Off);
    assert_eq!(Verbosity::Error.to_level_filter(), LevelFilter::Error);
    assert_eq!(Verbosity::Warn.to_level_filter(), LevelFilter::Warn);
    assert_eq!(Verbosity::Info.to_level_filter(), LevelFilter::Info);
    assert_eq!(Verbosity::Debug.to_level_filter(), LevelFilter::Debug);
    assert_eq!(Verbosity::Trace.to_level_filter(), LevelFilter::Trace);
}

#[test]
fn default_is_info() {
    // Arrange & Act
    let default = Verbosity::default();

    // Assert
    assert_eq!(default, Verbosity::Info);
}
