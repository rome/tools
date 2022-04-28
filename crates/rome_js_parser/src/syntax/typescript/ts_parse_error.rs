use crate::parser::expected_node;
use crate::{
    parser::{expected_any, ToDiagnostic},
    CompletedMarker, Parser,
};
use rome_diagnostics::{Diagnostic, Span};
use rome_rowan::TextRange;

pub(crate) fn expected_ts_enum_member(p: &Parser, range: TextRange) -> Diagnostic {
    expected_any(&["identifier", "string literal", "computed name"], range).to_diagnostic(p)
}

pub(crate) fn unexpected_abstract_member_with_body(p: &Parser, range: TextRange) -> Diagnostic {
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
    let modifier = p.source(modifier_range);
    p.err_builder(&format!(
        "'{modifier}' cannot appear on a constructor declaration."
    ))
    .primary(modifier_range, "")
}

pub(crate) fn ts_modifier_cannot_appear_on_a_parameter(
    p: &Parser,
    modifier_range: TextRange,
) -> Diagnostic {
    let modifier = p.source(modifier_range);
    p.err_builder(&format!("'{modifier}' cannot appear on a parameter."))
        .primary(modifier_range, "")
}

pub(crate) fn ts_accessibility_modifier_already_seen(
    p: &Parser,
    second_range: TextRange,
    first_range: TextRange,
) -> Diagnostic {
    p.err_builder("Accessibility modifier already seen.")
        .primary(second_range, "duplicate modifier")
        .secondary(first_range, "first modifier")
}

pub(crate) fn ts_only_syntax_error(p: &Parser, syntax: &str, range: TextRange) -> Diagnostic {
    p.err_builder(&format!("{} are a TypeScript only feature. Convert your file to a TypeScript file or remove the syntax.", syntax))
        .primary(range, "TypeScript only syntax")
}

pub(crate) fn ts_accessor_type_parameters_error(
    p: &Parser,
    type_parameters: &CompletedMarker,
) -> Diagnostic {
    p.err_builder("An accessor cannot have type parameters.")
        .primary(type_parameters.range(p), "")
}

pub(crate) fn ts_constructor_type_parameters_error(
    p: &Parser,
    type_parameters: &CompletedMarker,
) -> Diagnostic {
    p.err_builder("constructors cannot have type parameters.")
        .primary(type_parameters.range(p), "")
}

pub(crate) fn ts_set_accessor_return_type_error(
    p: &Parser,
    type_annotation: &CompletedMarker,
) -> Diagnostic {
    p.err_builder("A 'set' accessor cannot have a return type annotation.")
        .primary(type_annotation.range(p), "")
}

pub(crate) fn ts_type_assertion_on_new_expr(p: &Parser, type_expr: &CompletedMarker) -> Diagnostic {
    p.err_builder("Type assertions cannot be used at 'new' expressions.")
        .primary(type_expr.range(p), "")
}

pub(crate) fn expected_ts_type(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("type", range).to_diagnostic(p)
}

pub(crate) fn expected_ts_type_parameter(p: &Parser, range: TextRange) -> Diagnostic {
    expected_node("type parameter", range).to_diagnostic(p)
}
