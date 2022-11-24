use crate::{
    parser::{expected_any, expected_node, ToDiagnostic},
    JsParser, ParseDiagnostic,
};
use rome_js_syntax::TextRange;

pub(crate) fn jsx_expected_attribute(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("JSX attribute", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_attribute_value(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_node("JSX attribute value", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_children(p: &JsParser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["JSX Expression", "Element", "text"], range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_closing_tag(
    p: &JsParser,
    opening_name: &str,
    opening_range: TextRange,
    closing_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        format!("Expected corresponding JSX closing tag for '{opening_name}'."),
        opening_range,
    )
    .detail(opening_range, "Opening tag")
    .detail(closing_range, "closing tag")
}
