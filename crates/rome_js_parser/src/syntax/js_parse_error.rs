use crate::parser::{expected_any, expected_node, ToDiagnostic};
use crate::{ParseDiagnostic, Parser};
use rome_diagnostics::Span;
use rome_js_syntax::TextRange;

///! Provides factory function to create common diagnostics for the JavaScript syntax

pub(crate) fn expected_function_body(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("function body", range).to_diagnostic(p)
}

pub(crate) fn expected_class_member_name(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "identifier",
            "string literal",
            "number literal",
            "private field name",
            "computed name",
        ],
        range,
    )
    .to_diagnostic(p)
}

pub(crate) fn expected_arrow_body(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["function body", "expression"], range).to_diagnostic(p)
}

pub(crate) fn expected_object_member(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "property",
            "shorthand property",
            "getter",
            "setter",
            "method",
        ],
        range,
    )
    .to_diagnostic(p)
}
pub(crate) fn expected_array_element(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "expression", "method"], range).to_diagnostic(p)
}

pub(crate) fn expected_object_member_name(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "identifier",
            "string literal",
            "number literal",
            "computed property",
        ],
        range,
    )
    .to_diagnostic(p)
}

pub(crate) fn expected_block_statement(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("block statement", range).to_diagnostic(p)
}

pub(crate) fn expected_catch_clause(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("catch clause", range).to_diagnostic(p)
}

pub(crate) fn expected_parameter(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("parameter", range).to_diagnostic(p)
}

pub(crate) fn expected_parameters(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("parenthesis '('", range).to_diagnostic(p)
}

pub(crate) fn expected_case_or_default(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["default", "case"], range).to_diagnostic(p)
}

pub(crate) fn expected_case(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("case", range).to_diagnostic(p)
}

pub(crate) fn expected_assignment_target(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "assignment target"], range).to_diagnostic(p)
}

pub(crate) fn expected_simple_assignment_target(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "member expression"], range).to_diagnostic(p)
}

pub(crate) fn expected_identifier(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range).to_diagnostic(p)
}

pub(crate) fn expected_statement(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("statement", range).to_diagnostic(p)
}

pub(crate) fn expected_binding(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["identifier", "array pattern", "object pattern"], range).to_diagnostic(p)
}

pub(crate) fn expected_class_member(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range).to_diagnostic(p)
}

pub(crate) fn expected_class_parameters(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("class parameters", range).to_diagnostic(p)
}

pub(crate) fn expected_constructor_parameters(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("constructor parameters", range).to_diagnostic(p)
}

pub(crate) fn expected_class_method_body(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("class method body", range).to_diagnostic(p)
}

pub(crate) fn expected_module_source(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("string literal", range).to_diagnostic(p)
}

pub(crate) fn expected_named_import(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["namespace import", "named imports"], range).to_diagnostic(p)
}

pub(crate) fn expected_literal_export_name(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["string literal", "identifier"], range).to_diagnostic(p)
}

pub(crate) fn expected_export_clause(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["class", "function", "variable declaration"], range).to_diagnostic(p)
}

pub(crate) fn expected_export_name_specifier(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("export name", range).to_diagnostic(p)
}

pub(crate) fn expected_named_import_specifier(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("identifier", range).to_diagnostic(p)
}

pub(crate) fn duplicate_assertion_keys_error(
    p: &Parser,
    key: &str,
    first_use: TextRange,
    duplicate_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder("Duplicate assertion keys are not allowed", &first_use)
        .detail(&first_use, &format!("First use of the key `{}`", key))
        .detail(duplicate_range, "second use here")
}

pub(crate) fn expected_expression(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("expression", range).to_diagnostic(p)
}

pub(crate) fn expected_expression_assignment(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["expression", "assignment"], range).to_diagnostic(p)
}

pub(crate) fn expected_unary_expression(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_node("unary expression", range).to_diagnostic(p)
}

pub(crate) fn expected_property_or_signature(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(&["property", "signature"], range).to_diagnostic(p)
}

pub(crate) fn expected_declaration(p: &Parser, range: TextRange) -> ParseDiagnostic {
    expected_any(
        &[
            "function",
            "class",
            "variable declaration",
            "interface",
            "enum",
            "type alias",
        ],
        range,
    )
    .to_diagnostic(p)
}

pub(crate) fn unexpected_body_inside_ambient_context(
    p: &Parser,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "members inside ambient contexts should not have a body",
        range,
    )
}

pub(crate) fn private_names_only_allowed_on_left_side_of_in_expression(
    p: &Parser,
    private_name_range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        "Private names are only allowed on the left side of a 'in' expression",
        private_name_range,
    )
}

pub(crate) fn invalid_assignment_error(p: &Parser, range: TextRange) -> ParseDiagnostic {
    p.err_builder(
        &format!("Invalid assignment to `{}`", p.source(range.as_range()),),
        range,
    )
    .hint("This expression cannot be assigned to")
}

pub(crate) fn modifier_already_seen(
    p: &Parser,
    second_range: TextRange,
    first_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.source(second_range);
    p.err_builder(&format!("'{modifier}' already seen"), second_range)
        .detail(second_range, "duplicate modifier")
        .detail(first_range, "first seen here")
}

pub(crate) fn modifier_cannot_be_used_with_modifier(
    p: &Parser,
    range: TextRange,
    other_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier = p.source(range);
    let other_modifier = p.source(other_modifier_range);

    p.err_builder(
        &format!("'{modifier}' cannot be used with '{other_modifier}' modifier."),
        range,
    )
    .detail(range, &format!("'{modifier}' modifier"))
    .detail(
        other_modifier_range,
        &format!("'{other_modifier}' modifier"),
    )
}

pub(crate) fn modifier_must_precede_modifier(
    p: &Parser,
    range: TextRange,
    to_precede_modifier_range: TextRange,
) -> ParseDiagnostic {
    let modifier_name = p.source(range);
    let to_precede_name = p.source(to_precede_modifier_range);

    p.err_builder(
        &format!("'{modifier_name}' must precede '{to_precede_name}'",),
        range,
    )
    .detail(range, "move this modifier")
    .detail(to_precede_modifier_range, "before this modifier")
}
