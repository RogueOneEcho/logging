//! Extension trait for rendering diagnostics with miette's graphical output.

use miette::{Diagnostic, GraphicalReportHandler, GraphicalTheme};
use std::env;

/// Extension trait for rendering [`Diagnostic`] types with fancy output.
pub trait DiagnosticExt {
    /// Render the diagnostic using miette's graphical handler.
    fn render(&self) -> String;
}

impl<T: Diagnostic> DiagnosticExt for T {
    fn render(&self) -> String {
        let theme = if env::var("NO_COLOR").is_ok() {
            GraphicalTheme::unicode_nocolor()
        } else {
            GraphicalTheme::unicode()
        };
        let mut output = String::new();
        GraphicalReportHandler::new_themed(theme)
            .render_report(&mut output, self)
            .expect("diagnostic should render");
        output
    }
}
