use crate::parser::{expected_any, ParsedSyntax, ToDiagnostic};
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{
	conditional_expr, expr, is_at_reference_identifier_member, parse_reference_identifier_member,
	unary_expr,
};
use crate::syntax::js_parse_error::{
	expected_assignment_target, expected_identifier, expected_simple_assignment_target,
};
use crate::syntax::pattern::{ParseArrayPattern, ParseObjectPattern, ParseWithDefaultPattern};
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Parser};
use crate::{SyntaxKind::*, *};

// test assignment_target
// foo += bar = b ??= 3;
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count === 3

// test_err invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;

/// Converts the passed in target (expression) to an assignment target
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_target(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
	expr_kind: SimpleAssignmentTargetExprKind,
) -> CompletedMarker {
	if let Present(assignment_target) =
		try_expression_to_simple_assignment_target(p, target, checkpoint)
	{
		return assignment_target;
	}

	let expression_end = p.token_pos();
	p.rewind(checkpoint);

	match parse_assignment_target(p, expr_kind) {
		Present(target) => target,
		Absent => wrap_expression_in_invalid_assignment(p, expression_end),
	}
}

pub(crate) fn parse_assignment_target(
	p: &mut Parser,
	expression_kind: SimpleAssignmentTargetExprKind,
) -> ParsedSyntax {
	match p.cur() {
		T!['['] => ArrayAssignmentTarget.parse_array_pattern(p),
		T!['{'] if p.state.allow_object_expr => ObjectAssignmentTarget.parse_object_pattern(p),
		_ => parse_simple_assignment_target(p, expression_kind),
	}
}

/// Re-parses an expression as a simple assignment target.
pub(crate) fn expression_to_simple_assignment_target(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> CompletedMarker {
	if let Present(assignment_target) =
		try_expression_to_simple_assignment_target(p, target, checkpoint)
	{
		assignment_target
	} else {
		// Doesn't seem to be a valid assignment target. Recover and create an error.
		let expression_end = p.token_pos();
		p.rewind(checkpoint);
		wrap_expression_in_invalid_assignment(p, expression_end)
	}
}

pub(crate) enum SimpleAssignmentTargetExprKind {
	Unary,
	Conditional,
	Any,
}

pub(crate) fn parse_simple_assignment_target(
	p: &mut Parser,
	expr_kind: SimpleAssignmentTargetExprKind,
) -> ParsedSyntax {
	let checkpoint = p.checkpoint();

	// TODO remove the rewind inside of the error handle once the `unary_expr` returns a ParsedSyntax
	let assignment_expression = match expr_kind {
		SimpleAssignmentTargetExprKind::Unary => unary_expr(p),
		SimpleAssignmentTargetExprKind::Conditional => conditional_expr(p),
		SimpleAssignmentTargetExprKind::Any => expr(p),
	};

	if let Some(expr) = assignment_expression {
		Present(expression_to_simple_assignment_target(p, expr, checkpoint))
	} else {
		// Only necessary because `unary_expr` always adds a "expected an expression" error.
		p.rewind(checkpoint);
		Absent
	}
}

struct AssignmentTargetWithDefault;

impl ParseWithDefaultPattern for AssignmentTargetWithDefault {
	#[inline]
	fn pattern_with_default_kind() -> SyntaxKind {
		JS_ASSIGNMENT_TARGET_WITH_DEFAULT
	}

	#[inline]
	fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_assignment_target(p, range)
	}

	#[inline]
	fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
	}
}

struct ArrayAssignmentTarget;

impl ParseArrayPattern<AssignmentTargetWithDefault> for ArrayAssignmentTarget {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_ASSIGNMENT_TARGET
	}

	#[inline]
	fn array_pattern_kind() -> SyntaxKind {
		JS_ARRAY_ASSIGNMENT_TARGET
	}

	// test array_assignment_target_rest
	// ([ ...abcd ] = a);
	// ([ ...(abcd) ] = a);
	// ([ ...m.test ] = c);
	// ([ ...m[call()] ] = c);
	// ([ ...any.expression().b ] = c);
	// ([ ...[x, y] ] = b);
	// ([ ...[ ...a ] ] = c);
	//
	// test_err array_assignment_target_rest_err
	// ([ ... ] = a);
	// ([ ...c = "default" ] = a);
	// ([ ...rest, other_assignment ] = a);
	#[inline]
	fn rest_pattern_kind() -> SyntaxKind {
		JS_ARRAY_ASSIGNMENT_TARGET_REST_ELEMENT
	}

	#[inline]
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(&["assignment target", "rest element", "comma"], range).to_diagnostic(p)
	}

	#[inline]
	fn pattern_with_default(&self) -> AssignmentTargetWithDefault {
		AssignmentTargetWithDefault
	}
}

fn parse_identifier_assignment_target(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T![yield] | T![await] | T![ident] => {
			let m = p.start();
			let name = p.cur_src();

			let mut valid = false;

			if name == "await" && p.state.in_async {
				let err = p
					.err_builder("Illegal use of `await` as an identifier in an async context")
					.primary(p.cur_tok().range, "");
				p.error(err);
			} else if name == "yield" && p.state.in_generator {
				let err = p
					.err_builder("Illegal use of `yield` as an identifier in a generator function")
					.primary(p.cur_tok().range, "");
				p.error(err);
			} else {
				valid = true;
			}

			p.bump_remap(T![ident]);

			if valid {
				Present(m.complete(p, JS_IDENTIFIER_ASSIGNMENT_TARGET))
			} else {
				Present(m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET))
			}
		}
		_ => Absent,
	}
}

struct ObjectAssignmentTarget;

// test object_assignment_target
// ({} = {});
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
impl ParseObjectPattern for ObjectAssignmentTarget {
	#[inline]
	fn unknown_pattern_kind() -> SyntaxKind {
		JS_UNKNOWN_ASSIGNMENT_TARGET
	}

	#[inline]
	fn object_pattern_kind() -> SyntaxKind {
		JS_OBJECT_ASSIGNMENT_TARGET
	}

	#[inline]
	fn expected_property_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_any(&["assignment target", "rest property"], range).to_diagnostic(p)
	}

	// test property_assignment_target
	// ({x}= {});
	// ({x: y}= {});
	// ({x: y.test().z}= {});
	// ({x: ((z))}= {});
	// ({x: z["computed"]}= {});
	// ({x = "default"}= {});
	// ({x: y = "default"}= {});
	//
	// test_err property_assignment_target_err
	// ({:y} = {});
	// ({=y} = {});
	// ({:="test"} = {});
	// ({:=} = {});
	// ({ a b } = {});
	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !is_at_reference_identifier_member(p)
			&& !p.at_ts(token_set![T![:], T![=], T![ident], T![await], T![yield]])
		{
			return Absent;
		}

		let m = p.start();

		let kind = if p.at(T![:]) || p.nth_at(1, T![:]) {
			parse_reference_identifier_member(p).or_missing_with_error(p, expected_identifier);
			p.expect_required(T![:]);
			parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
				.or_missing_with_error(p, expected_assignment_target);
			JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET
		} else {
			parse_identifier_assignment_target(p).or_missing_with_error(p, expected_identifier);
			JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET
		};

		parse_equal_value_clause(p).or_missing(p);

		Present(m.complete(p, kind))
	}

	// test rest_property_assignment_target
	// ({ ...abcd } = a);
	// ({ ...(abcd) } = a);
	// ({ ...m.test } = c);
	// ({ ...m[call()] } = c);
	// ({ ...any.expression().b } = c);
	// ({ b: { ...a } } = c);
	//
	// test_err rest_property_assignment_target_err
	// ({ ... } = a);
	// ({ ...c = "default" } = a);
	// ({ ...{a} } = b);
	// ({ ...rest, other_assignment } = a);
	// ({ ...rest, } = a);
	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T![...]) {
			return Absent;
		}

		let m = p.start();
		p.bump(T![...]);

		let target = parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional);

		if matches!(
			target.kind(),
			Some(JS_OBJECT_ASSIGNMENT_TARGET | JS_ARRAY_ASSIGNMENT_TARGET)
		) {
			target.abandon(p);
			let completed = m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);

			p.error(
				p.err_builder(
					"object and array assignment targets are not allowed in rest patterns",
				)
				.primary(completed.range(p), ""),
			);

			return Present(completed);
		}

		target.or_missing_with_error(p, expected_assignment_target);

		Present(m.complete(p, JS_OBJECT_REST_PROPERTY_ASSIGNMENT_TARGET))
	}
}

fn try_expression_to_simple_assignment_target(
	p: &mut Parser,
	mut target: CompletedMarker,
	checkpoint: Checkpoint,
) -> ParsedSyntax {
	if target.kind() == JS_PARENTHESIZED_EXPRESSION {
		// Special treatment for parenthesized expressions because they can be nested and an error
		// should only cover the sub-expressions that are indeed invalid assignment targets.
		// This code traverses through all descendants of the parenthesized expression and tries to
		// convert them to valid assignment targets. It returns the converted parenthesized expression if
		// everything is valid and otherwise re-parses the parenthesized expression only:
		let events = &mut p.events[target.start_pos as usize..target.finish_pos as usize];
		let mut children_valid = true;

		for event in events {
			match event {
				Event::Start {
					kind: TOMBSTONE, ..
				} => {}
				Event::Start { kind, .. } => {
					if let Some(assignment_target_kind) =
						map_expression_to_simple_assignment_target_kind(*kind)
					{
						*kind = assignment_target_kind
					} else {
						children_valid = false;
						// continue to convert other children
					}
				}
				_ => {}
			}
		}

		if children_valid {
			Present(target)
		} else {
			p.rewind(checkpoint);

			// You're wondering why this is OK? The reason is, that there's a valid outermost parenthesized
			// assignment target. The problem is with one of the inner assignment targets and this is why we
			// reparse it to add the necessariy diagnostics
			Present(re_parse_parenthesized_expression_as_assignment_target(p))
		}
	} else if let Some(assignment_target_kind) =
		map_expression_to_simple_assignment_target_kind(target.kind())
	{
		target.change_kind(p, assignment_target_kind);
		Present(target)
	} else {
		Absent
	}
}

/// Re-parses a parenthesized expression as an assignment target.
/// Only intended to be used if the parser fully rewinds to the position before a valid
/// parenthesized expression.
///
/// # Panics
/// If the parser isn't positioned at a parenthesized expression.
fn re_parse_parenthesized_expression_as_assignment_target(p: &mut Parser) -> CompletedMarker {
	let outer = p.start();
	p.bump(T!['(']);

	// re-parse any nested parenthesized assignment targets
	if p.at(T!['(']) {
		re_parse_parenthesized_expression_as_assignment_target(p);
	} else {
		// if the parenthesized expression contains any other assignment target, re-parse it too
		parse_simple_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
			.or_missing_with_error(p, expected_simple_assignment_target);
	}

	p.expect_required(T![')']);

	outer.complete(p, JS_PARENTHESIZED_ASSIGNMENT_TARGET)
}

fn map_expression_to_simple_assignment_target_kind(kind: SyntaxKind) -> Option<SyntaxKind> {
	match kind {
		JS_STATIC_MEMBER_EXPRESSION => Some(JS_STATIC_MEMBER_ASSIGNMENT_TARGET),
		JS_COMPUTED_MEMBER_EXPRESSION => Some(JS_COMPUTED_MEMBER_ASSIGNMENT_TARGET),
		JS_REFERENCE_IDENTIFIER_EXPRESSION => Some(JS_IDENTIFIER_ASSIGNMENT_TARGET),
		JS_PARENTHESIZED_EXPRESSION => Some(JS_PARENTHESIZED_ASSIGNMENT_TARGET),
		_ => None,
	}
}

fn wrap_expression_in_invalid_assignment(p: &mut Parser, expression_end: usize) -> CompletedMarker {
	let unknown = p.start();
	// Eat all tokens until we reached the end of the original expression. This is better than
	// any other error recovery because it's already know where the expression ends.
	while p.token_pos() < expression_end {
		p.bump_any();
	}

	let completed = unknown.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);

	let expression_range = completed.range(p);
	p.error(
		p.err_builder(&format!(
			"Invalid assignment to `{}`",
			p.source(expression_range)
		))
		.primary(expression_range, "This expression cannot be assigned to"),
	);

	completed
}
