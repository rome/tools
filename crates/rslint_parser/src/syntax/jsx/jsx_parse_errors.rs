use rome_js_syntax::TextRange;
use rslint_errors::Diagnostic;

use crate::{
    parser::{expected_any, expected_node, ToDiagnostic},
    parser::{expected_any, ToDiagnostic},
    parser::{expected_node, ToDiagnostic},
    Parser,
};

pub(crate) fn jsx_only_syntax_error(p: &Parser, syntax: &str, range: TextRange) -> Diagnostic {
    p.err_builder(&format!(
        "{} are a JSX only feature. Convert your file to a JSX file or remove the syntax.",
        syntax
    ))
    .primary(range, "JSX only syntax")
}

pub(crate) fn jsx_expected_attribute(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("JSX attribute", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_attribute_value(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("JSX attribute value", range).to_diagnostic(p)
}

pub(crate) fn jsx_expected_children(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("JSX child", range).to_diagnostic(p)
}
