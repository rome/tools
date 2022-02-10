use crate::{
    parser::{expected_any, expected_token, ToDiagnostic},
    Parser,
};
use rslint_errors::Diagnostic;
use rslint_lexer::T;
use std::ops::Range;

pub(crate) fn expected_ts_enum_member(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "string literal", "computed name"], range).to_diagnostic(p)
}

pub(crate) fn unexpected_abstract_member_with_body(p: &Parser, range: Range<usize>) -> Diagnostic {
    dbg!(&range);
    p.err_builder("abstract members should not have a body")
        .primary(range, "unexpected")
}
