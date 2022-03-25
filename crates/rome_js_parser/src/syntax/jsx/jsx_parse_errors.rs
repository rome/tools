use crate::{
    parser::{expected_any, expected_node, ToDiagnostic},
    Parser,
};
use rome_diagnostics::Diagnostic;
use rome_js_syntax::TextRange;

pub(crate) fn jsx_expected_attribute(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("JSX attribute", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_attribute_value(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("JSX attribute value", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_children(p: &Parser, range: TextRange) -> Diagnostic {
    expected_any(&["JSX Expression", "Element", "text"], range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_closing_tag(
    p: &Parser,
    opening_name: &str,
    opening_range: TextRange,
    closing_range: TextRange,
) -> Diagnostic {
    p.err_builder(&format!(
        "Expected corresponding JSX closing tag for '{opening_name}'."
    ))
    .primary(opening_range, "Opening tag")
    .secondary(closing_range, "closing tag")
}
