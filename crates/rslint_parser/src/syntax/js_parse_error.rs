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
