//! Custom color extensions for terminal output.

use colored::{ColoredString, Colorize, CustomColor};

const GRAY: u8 = 168;
const DARK_GRAY: u8 = 112;

/// Extension trait for applying custom gray colors to strings.
pub trait Colors {
    /// Associated error type (unused, exists for trait consistency).
    type Error;

    /// Apply a medium gray color (RGB 168).
    fn gray(&self) -> ColoredString;

    /// Apply a dark gray color (RGB 112).
    fn dark_gray(&self) -> ColoredString;
}

impl Colors for &str {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}

impl Colors for String {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}

impl Colors for ColoredString {
    type Error = ();

    fn gray(&self) -> ColoredString {
        self.clone()
            .custom_color(CustomColor::new(GRAY, GRAY, GRAY))
    }

    fn dark_gray(&self) -> ColoredString {
        self.clone()
            .custom_color(CustomColor::new(DARK_GRAY, DARK_GRAY, DARK_GRAY))
    }
}
