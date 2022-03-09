use rome_js_syntax::TextRange;
use rslint_errors::Diagnostic;

use crate::Parser;

pub(crate) fn jsx_only_syntax_error(p: &Parser, syntax: &str, range: TextRange) -> Diagnostic {
    p.err_builder(&format!(
        "{} are a Jsx only feature. Convert your file to a Jsx file or remove the syntax.",
        syntax
    ))
    .primary(range, "Jsx only syntax")
}
