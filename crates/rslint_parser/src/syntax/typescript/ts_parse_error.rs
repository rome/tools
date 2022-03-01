use crate::{
    parser::{expected_any, ToDiagnostic},
    Parser,
};
use rome_rowan::TextRange;
use rslint_errors::{Diagnostic, Span};
use std::ops::Range;

pub(crate) fn expected_ts_enum_member(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "string literal", "computed name"], range).to_diagnostic(p)
}

pub(crate) fn unexpected_abstract_member_with_body(p: &Parser, range: Range<usize>) -> Diagnostic {
    p.err_builder("abstract members should not have a body")
        .primary(range, "")
}

pub(crate) fn abstract_member_cannot_be_async(p: &Parser, range: &TextRange) -> Diagnostic {
    p.err_builder("async members cannot be abstract")
        .primary(range, "")
}

pub(crate) fn ts_member_cannot_be(
    p: &Parser,
    range: impl Span,
    member_type_name: &str,
    modifier_name: &str,
) -> Diagnostic {
    let msg = format!("{} members cannot be {}", member_type_name, modifier_name);
    p.err_builder(&msg).primary(range, "")
}

pub(crate) fn ts_modifier_cannot_appear_on_a_constructor_declaration(
    p: &Parser,
    modifier_range: TextRange,
) -> Diagnostic {
    let modifier = p.span_text(modifier_range);
    p.err_builder(&format!(
        "'{modifier} cannot appear on a constructor declaration."
    ))
    .primary(modifier_range, "")
}

pub(crate) fn ts_modifier_cannot_appear_on_a_parameter(
    p: &Parser,
    modifier_range: TextRange,
) -> Diagnostic {
    let modifier = p.span_text(modifier_range);
    p.err_builder(&format!("'{modifier} cannot appear on a parameter"))
        .primary(modifier_range, "")
}
