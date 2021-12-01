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
