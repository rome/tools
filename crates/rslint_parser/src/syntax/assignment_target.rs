use crate::parser::ParsedSyntax;
use crate::syntax::class::parse_equal_value_clause;
use crate::syntax::expr::{
	conditional_expr, expr_or_assignment_target, identifier_name, EXPR_RECOVERY_SET,
};
use crate::syntax::js_parse_error::{
	expected_array_assignment_target_element, expected_assignment_target,
	expected_property_assignment_target,
};
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Parser};
use crate::{SyntaxKind::*, *};

/// Converts the passed in target (expression) to an assignment target
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_target(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> CompletedMarker {
	if let Ok(assignment_target) = try_expression_to_simple_assignment_target(p, target) {
		return assignment_target;
	}

	let expression_end = p.token_pos();
	p.rewind(checkpoint);

	match parse_assignment_target(p) {
		Present(target) => target,
		Absent => {
			let unknown = p.start();
			// Eat all tokens until we reached the end of the original expression. This is better than
			// any other error recovery because it's already know where the expression ends.
			while p.token_pos() <= expression_end {
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
	}
}

// * members
// * Re-add parenthesized assignment target, allowed in some places

pub(crate) fn parse_assignment_target(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T!['['] => parse_array_assignment_target(p),
		T!['{'] if p.state.allow_object_expr => parse_object_assignment_target(p),
		_ => parse_simple_assignment_target(p),
	}
}

pub(crate) fn parse_simple_assignment_target(p: &mut Parser) -> ParsedSyntax {
	let checkpoint = p.checkpoint();

	if let Some(expr) = conditional_expr(p) {
		let assignment_target = try_expression_to_simple_assignment_target(p, expr);
		match assignment_target {
			Ok(target) => Present(target),
			Err(_) => {
				// Ideally, rewind wouldn't be needed here because there's a try_expr function that tries to parse
				// the expression and otherwise returns an Error but doesn't add any diagnostics
				p.rewind(checkpoint);
				Absent
			}
		}
	} else {
		Absent
	}
}

fn parse_assignment_target_with_optional_default(p: &mut Parser) -> ParsedSyntax {
	let target = parse_assignment_target(p);

	if p.at(T![=]) {
		let with_default = target.precede_or_missing_with_error(p, expected_assignment_target);
		p.bump_any(); // eat the = token
		expr_or_assignment_target(p);
		Present(with_default.complete(p, JS_ASSIGNMENT_TARGET_WITH_DEFAULT))
	} else {
		target
	}
}

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

		if parse_array_assignment_target_rest_element(p).is_present() {
			break;
		}

		let element = {
			let mut guard = p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...], T![=]]),
				..p.state.clone()
			});

			let recovery = ParseRecovery::new(
				JS_UNKNOWN_ASSIGNMENT_TARGET,
				token_set!(EOF, T![,], T![']'], T![=], T![;], T![...]),
			)
			.enable_recovery_on_line_break();
			parse_assignment_target_with_optional_default(&mut *guard).or_recover(
				&mut *guard,
				recovery,
				expected_array_assignment_target_element,
			)
		};

		if element.is_err() {
			// Failed to recover
			break;
		}

		if !p.at(T![']']) {
			p.expect(T![,]);
		}
	}

	elements.complete(p, LIST);
	p.expect(T![']']);

	Present(m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET))
}

fn parse_array_assignment_target_rest_element(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![...]) {
		return Absent;
	}

	let m = p.start();
	p.bump(T![...]);

	parse_assignment_target_with_optional_default(p)
		.or_missing_with_error(p, expected_assignment_target);

	if p.eat(T![,]) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here"),
		);
		Present(m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET))
	} else {
		Present(m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET_REST_ELEMENT))
	}
}

fn parse_object_assignment_target(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['{']) {
		return Absent;
	}

	let m = p.start();

	p.bump(T!['{']);
	let elements = p.start();

	while !matches!(p.cur(), EOF | T!['}']) {
		if parse_object_rest_property_assignment_target(p).is_present() {
			break;
		}

		let element = parse_property_assignment_target(p).or_recover(
			p,
			ParseRecovery::new(
				JS_UNKNOWN_ASSIGNMENT_TARGET,
				token_set!(EOF, T![,], T![']'], T![...], T![;]),
			)
			.enable_recovery_on_line_break(),
			expected_property_assignment_target,
		);

		if element.is_err() {
			break;
		}

		if !p.at(T!['}']) {
			p.expect(T![,]);
		}
	}

	elements.complete(p, LIST);
	p.expect(T!['}']);

	Present(m.complete(p, JS_OBJECT_ASSIGNMENT_TARGET))
}

const PROPERTY_ASSIGNMENT_TARGET_START_TOKENS: TokenSet =
	token_set![T![ident], T![yield], T![await], T![:], T![=]];

fn parse_property_assignment_target(p: &mut Parser) -> ParsedSyntax {
	if !p.at_ts(PROPERTY_ASSIGNMENT_TARGET_START_TOKENS) {
		return Absent;
	}

	let m = p.start();
	let mut property_name = identifier_name(p)
		.expect("The parser is currently at an identifier, calling identifier_name should succeed");
	let is_shorthand_property = !p.eat(T![:]);

	if is_shorthand_property {
		property_name.change_kind(p, JS_IDENTIFIER_ASSIGNMENT_TARGET);
	} else {
		property_name.change_kind(p, JS_REFERENCE_IDENTIFIER_MEMBER);

		parse_assignment_target(p).or_missing_with_error(p, expected_assignment_target);
	}

	{
		let mut guard = p.with_state(ParserState {
			expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
			..p.state.clone()
		});
		parse_equal_value_clause(&mut *guard).or_missing(&mut *guard);
	}

	Present(m.complete(
		p,
		if is_shorthand_property {
			JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET
		} else {
			JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET
		},
	))
}

fn parse_object_rest_property_assignment_target(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![...]) {
		return Absent;
	}

	let m = p.start();
	p.bump(T![...]);

	parse_simple_assignment_target(p).or_missing_with_error(p, expected_assignment_target);

	if p.eat(T![,]) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here"),
		);
		Present(m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET))
	} else {
		Present(m.complete(p, JS_OBJECT_REST_PROPERTY_ASSIGNMENT_TARGET))
	}
}

fn try_expression_to_simple_assignment_target(
	p: &mut Parser,
	mut target: CompletedMarker,
) -> Result<CompletedMarker, ()> {
	let mapped_kind = match target.kind() {
		JS_STATIC_MEMBER_EXPRESSION => JS_STATIC_MEMBER_ASSIGNMENT_TARGET,
		JS_COMPUTED_MEMBER_EXPRESSION => JS_COMPUTED_MEMBER_ASSIGNMENT_TARGET,
		JS_REFERENCE_IDENTIFIER_EXPRESSION => JS_IDENTIFIER_ASSIGNMENT_TARGET,
		JS_UNKNOWN_EXPRESSION | ERROR => JS_UNKNOWN_ASSIGNMENT_TARGET,
		_ => {
			target.undo_completion(p).abandon(p);
			return Err(());
		}
	};

	target.change_kind(p, mapped_kind);
	Ok(target)
}
