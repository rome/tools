use crate::parser::{expected_any, expected_node, ToDiagnostic};
use crate::Parser;
use rslint_errors::Diagnostic;
use std::ops::Range;

///! Provides factory function to create common diagnostics for the JavaScript syntax

pub(crate) fn expected_function_body(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("function body", range).to_diagnostic(p)
}

pub(crate) fn expected_class_member_name(p: &Parser, range: Range<usize>) -> Diagnostic {
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

pub(crate) fn expected_arrow_body(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["function body", "expression"], range).to_diagnostic(p)
}

pub(crate) fn expected_object_member(p: &Parser, range: Range<usize>) -> Diagnostic {
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

pub(crate) fn expected_array_element(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["property", "expression", "method"], range).to_diagnostic(p)
}

pub(crate) fn expected_object_member_name(p: &Parser, range: Range<usize>) -> Diagnostic {
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

pub(crate) fn expected_block_statement(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("block statement", range).to_diagnostic(p)
}

pub(crate) fn expected_catch_clause(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("catch clause", range).to_diagnostic(p)
}

pub(crate) fn expected_parameter(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("parameter", range).to_diagnostic(p)
}

pub(crate) fn expected_parameters(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("parenthesis '('", range).to_diagnostic(p)
}

pub(crate) fn expected_case_or_default(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["default", "case"], range).to_diagnostic(p)
}

pub(crate) fn expected_case(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("case", range).to_diagnostic(p)
}

pub(crate) fn expected_assignment_target(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "assignment target"], range).to_diagnostic(p)
}

pub(crate) fn expected_simple_assignment_target(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "member expression"], range).to_diagnostic(p)
}

pub(crate) fn expected_identifier(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("identifier", range).to_diagnostic(p)
}

pub(crate) fn expected_variable(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["let", "const", "var"], range).to_diagnostic(p)
}

pub(crate) fn expected_statement(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("statement", range).to_diagnostic(p)
}

pub(crate) fn expected_binding(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["identifier", "array pattern", "object pattern"], range).to_diagnostic(p)
}

pub(crate) fn expected_class_member(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["property ", "method", "getter", "setter"], range).to_diagnostic(p)
}

pub(crate) fn expected_class_parameters(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("class parameters", range).to_diagnostic(p)
}

pub(crate) fn expected_constructor_parameters(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("constructor parameters", range).to_diagnostic(p)
}

pub(crate) fn expected_class_method_body(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("class method body", range).to_diagnostic(p)
}

pub(crate) fn expected_module_source(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("string literal", range).to_diagnostic(p)
}

pub(crate) fn expected_named_import(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["namespace import", "named imports"], range).to_diagnostic(p)
}

pub(crate) fn expected_literal_export_name(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["string literal", "identifier"], range).to_diagnostic(p)
}

pub(crate) fn expected_export_clause(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["class", "function", "variable declaration"], range).to_diagnostic(p)
}

pub(crate) fn expected_export_name_specifier(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("export name", range).to_diagnostic(p)
}

pub(crate) fn expected_named_import_specifier(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("identifier", range).to_diagnostic(p)
}

pub(crate) fn expected_local_name_for_default_import(
    p: &Parser,
    range: Range<usize>,
) -> Diagnostic {
    p.err_builder("`default` imports must be aliased")
        .primary(p.cur_tok().range(), "`default` used here")
        .secondary(
            range.end..range.end,
            "add the `as` keyword followed by an identifier name here",
        )
}

pub(crate) fn duplicate_assertion_keys_error(
    p: &Parser,
    key: &str,
    first_use: Range<usize>,
    duplicate_range: Range<usize>,
) -> Diagnostic {
    p.err_builder("Duplicate assertion keys are not allowed")
        .primary(&first_use, &format!("First use of the key `{}`", key))
        .secondary(duplicate_range, "second use here")
}

pub(crate) fn expected_expression(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("expression", range).to_diagnostic(p)
}

pub(crate) fn expected_expression_assignment(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_any(&["expression", "assignment"], range).to_diagnostic(p)
}

pub(crate) fn expected_unary_expression(p: &Parser, range: Range<usize>) -> Diagnostic {
    expected_node("unary expression", range).to_diagnostic(p)
}
