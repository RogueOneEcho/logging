use crate::errors::diagnostic_ext::DiagnosticExt;
use crate::errors::tests::test_helpers::*;
use crate::Failure;

#[test]
fn render_produces_output() {
    use_colors(false);
    let failure = Failure::new(TestAction::ReadConfig, io_error());
    let rendered = failure.render();
    assert!(!rendered.is_empty());
    assert!(rendered.contains("read config"));
}

#[test]
fn render_includes_nested_cause() {
    use_colors(false);
    let failure = http_error();
    let rendered = failure.render();
    assert!(rendered.contains("cache users"));
    assert!(rendered.contains("parse response"));
}
