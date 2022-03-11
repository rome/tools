use rome_js_syntax::TextRange;
use rslint_errors::Diagnostic;

use crate::{
    parser::{expected_any, ToDiagnostic},
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
    expected_any(&["JSX attribute"], range).to_diagnostic(p)
}
