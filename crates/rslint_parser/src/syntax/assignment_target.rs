use crate::parser::{ParsedSyntax, RecoveryError};
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{
	conditional_expr, expr, expr_or_assignment, is_at_reference_identifier_member,
	parse_reference_identifier_member, unary_expr, EXPR_RECOVERY_SET,
};
use crate::syntax::js_parse_error::{
	expected_array_assignment_target_element, expected_assignment_target, expected_identifier,
	expected_property_assignment_target, expected_simple_assignment_target,
};
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
		T!['['] => parse_array_assignment_target(p),
		T!['{'] if p.state.allow_object_expr => parse_object_assignment_target(p),
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

fn parse_assignment_target_with_optional_default(p: &mut Parser) -> ParsedSyntax {
	let target = parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional);

	if p.at(T![=]) {
		let with_default = target.precede_or_missing_with_error(p, expected_assignment_target);
		p.bump_any(); // eat the = token
		expr_or_assignment(p);
		Present(with_default.complete(p, JS_ASSIGNMENT_TARGET_WITH_DEFAULT))
	} else {
		target
	}
}

// test array_assignment_target
// [foo, bar] = baz;
// [,,,b,,c,] = baz;
// [a = "test", a.b, call().b] = baz;
// [((a))] = baz;
//
// test_err array_assignment_target_err
// [a a, ++b, ] = test;
// [a, c, ...rest,] = test;
// [a = , = "test"] = test;
// [[a b] [c]]= test;
// [a: b] = c
fn parse_array_assignment_target(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['[']) {
		return Absent;
	}

	let m = p.start();

	p.bump(T!['[']);
	let elements = p.start();

	while !p.at(EOF) && !p.at(T![']']) {
		if p.at(T![,]) {
			p.start().complete(p, SyntaxKind::JS_ARRAY_HOLE);
			p.bump_any();
			continue;
		}

		let recovery = ParseRecovery::new(
			JS_UNKNOWN_ASSIGNMENT_TARGET,
			token_set!(EOF, T![,], T![']'], T![=], T![;], T![...]),
		)
		.enable_recovery_on_line_break();

		if let Present(rest) = parse_array_assignment_target_rest_element(p) {
			if validate_rest_pattern(p, rest, T![']'], &recovery, JS_UNKNOWN_ASSIGNMENT_TARGET) {
				break;
			}
		} else {
			let element = {
				let mut guard = p.with_state(ParserState {
					expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...], T![=]]),
					..p.state.clone()
				});

				parse_assignment_target_with_optional_default(&mut *guard).or_recover(
					&mut *guard,
					&recovery,
					expected_array_assignment_target_element,
				)
			};

			if element.is_err() {
				// Failed to recover
				break;
			}
		}

		if !p.at(T![']']) {
			p.expect_required(T![,]);
		}
	}

	elements.complete(p, LIST);
	p.expect(T![']']);

	Present(m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET))
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
fn parse_array_assignment_target_rest_element(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![...]) {
		return Absent;
	}

	let m = p.start();
	p.bump(T![...]);

	parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
		.or_missing_with_error(p, expected_assignment_target);

	Present(m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET_REST_ELEMENT))
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

pub(crate) trait ObjectPattern {
	fn parse_object_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T!['{']) {
			return Absent;
		}

		let m = p.start();

		p.bump(T!['{']);
		let elements = p.start();

		let recovery_set = ParseRecovery::new(
			self.unknown_pattern_kind(),
			token_set!(EOF, T![,], T!['}'], T![...], T![;]),
		)
		.enable_recovery_on_line_break();

		while !p.at(T!['}']) {
			if p.at(T![,]) {
				// missing element
				p.missing();
				p.error(self.expected_property_pattern_error(p, p.cur_tok().range));
				p.bump_any(); // bump ,
				continue;
			}

			if let Present(rest) = self.parse_rest_property_pattern(p) {
				if validate_rest_pattern(
					p,
					rest,
					T!['}'],
					&recovery_set,
					self.unknown_pattern_kind(),
				) {
					break;
				}
			} else {
				let recover_result =
					self.parse_any_property_pattern(p)
						.or_recover(p, &recovery_set, |p, range| {
							self.expected_property_pattern_error(p, range)
						});

				if recover_result.is_err() {
					break;
				}

				match recover_result {
					Err(RecoveryError::Eof) => break,
					Err(RecoveryError::AlreadyRecovered) => {
						p.error(expected_assignment_target(p, p.cur_tok().range));
						break;
					}
					_ => {}
				}
			}

			if !p.at(T!['}']) {
				p.expect_required(T![,]);
			}
		}

		elements.complete(p, LIST);
		p.expect(T!['}']);

		Present(m.complete(p, self.object_pattern_kind()))
	}

	fn unknown_pattern_kind(&self) -> SyntaxKind;
	fn object_pattern_kind(&self) -> SyntaxKind;

	fn expected_property_pattern_error(&self, p: &Parser, range: Range<usize>) -> Diagnostic;

	fn parse_any_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if p.at(T![:]) || p.nth_at(1, T![:]) {
			self.parse_property_pattern(p)
		} else {
			self.parse_shorthand_property_pattern(p)
		}
	}

	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax;

	fn parse_shorthand_property_pattern(&self, p: &mut Parser) -> ParsedSyntax;

	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax;
}

fn parse_object_assignment_target(p: &mut Parser) -> ParsedSyntax {
	ObjectAssignmentTarget.parse_object_pattern(p)
}

struct ObjectAssignmentTarget;

// test object_assignment_target
// ({} = {});
// ({ bar, baz } = {});
// ({ bar: [baz = "baz"], foo = "foo", ...rest } = {});
impl ObjectPattern for ObjectAssignmentTarget {
	fn unknown_pattern_kind(&self) -> SyntaxKind {
		JS_UNKNOWN_ASSIGNMENT_TARGET
	}

	fn object_pattern_kind(&self) -> SyntaxKind {
		JS_OBJECT_ASSIGNMENT_TARGET
	}

	fn expected_property_pattern_error(&self, p: &Parser, range: Range<usize>) -> Diagnostic {
		expected_property_assignment_target(p, range)
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
		if !is_at_reference_identifier_member(p) && !p.at_ts(token_set![T![:], T![=]]) {
			return Absent;
		}

		let m = p.start();
		parse_reference_identifier_member(p).or_missing_with_error(p, expected_identifier);

		p.expect_required(T![:]);

		parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
			.or_missing_with_error(p, expected_assignment_target);

		{
			// TODO remove after migrating expression to `ParsedSyntax`
			let mut guard = p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
				..p.state.clone()
			});
			parse_equal_value_clause(&mut *guard).or_missing(&mut *guard);
		}

		Present(m.complete(p, JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET))
	}

	fn parse_shorthand_property_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		let identifier = parse_identifier_assignment_target(p);

		if p.at(T![=]) || identifier.is_present() {
			let shorthand_prop = identifier.precede_or_missing_with_error(p, expected_identifier);

			{
				// TODO remove after migrating expression to `ParsedSyntax`
				let mut guard = p.with_state(ParserState {
					expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
					..p.state.clone()
				});
				parse_equal_value_clause(&mut *guard).or_missing(&mut *guard);
			}

			Present(shorthand_prop.complete(p, JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET))
		} else {
			Absent
		}
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

		let target = parse_assignment_target(p, SimpleAssignmentTargetExprKind::Conditional)
			.or_missing_with_error(p, expected_assignment_target);
		if let Some(mut target) = target {
			if matches!(
				target.kind(),
				JS_OBJECT_ASSIGNMENT_TARGET | JS_ARRAY_ASSIGNMENT_TARGET
			) {
				target.change_kind(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
				p.error(
					p.err_builder(
						"object and array assignment targets are not allowed in rest patterns",
					)
					.primary(target.range(p), ""),
				);
			}
		}

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

/// Validates if the parsed completed rest marker is a valid rest element inside of a
/// array or object assignment target and converts it to an unknown assignment target if not.
/// A rest element must be:
///
/// * the last element
/// * not followed by a trailing comma
/// * not have a default value
#[must_use]
fn validate_rest_pattern(
	p: &mut Parser,
	mut rest: CompletedMarker,
	end_token: SyntaxKind,
	recovery: &ParseRecovery,
	unknown_kind: SyntaxKind,
) -> bool {
	if p.at(end_token) {
		return true;
	}

	if p.at(T![=]) {
		let rest_range = rest.range(p);
		let rest_marker = rest.undo_completion(p);
		let default_start = p.cur_tok().range.start;
		p.bump(T![=]);

		if let Ok(recovered) = recovery.recover(p) {
			recovered.undo_completion(p).abandon(p); // append recovered content to parent
		}
		p.error(
			p.err_builder("rest element cannot have a default")
				.primary(
					default_start..p.cur_tok().range.start,
					"Remove the default value here",
				)
				.secondary(rest_range, "Rest element"),
		);

		rest_marker.complete(p, unknown_kind);
		return false;
	} else if p.at(T![,]) && p.nth_at(1, end_token) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here")
				.secondary(rest.range(p), "Rest element"),
		);
	} else {
		p.error(
			p.err_builder("rest element must be the last element")
				.primary(rest.range(p), "Move the rest element to the end"),
		);
	}

	rest.change_kind(p, unknown_kind);

	false
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
