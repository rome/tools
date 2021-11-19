use crate::syntax::class::optional_equals_value_clause;
use crate::syntax::expr::{
	conditional_expr, expr, expr_or_assignment_target, identifier_name, primary_expr,
	EXPR_RECOVERY_SET,
};
use crate::{CompletedMarker, Parser};
use crate::{SyntaxKind::*, *};
use rslint_errors::Diagnostic;

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

	match try_assignment_target(p) {
		Ok(target) => target,
		Err(_) => {
			let unknown = p.start();
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

/// Parses an assignment target and inserts and creates an unknown assignment target if
/// the current token isn't a valid start for an assignment target
pub(crate) fn assignment_target(p: &mut Parser) -> Option<CompletedMarker> {
	let result = try_assignment_target(p).ok();

	if result == None {
		p.error(missing_assignment_target_error(p));
	}

	result
}

fn missing_assignment_target_error(p: &Parser) -> Diagnostic {
	p.err_builder("Missing assignment target").primary(
		p.cur_tok().range,
		"Expected an identifier, member expression, or an array or object pattern here",
	)
}

/// Tries to parse an assignment target and returns the node's completed marker if successful.
/// Returns an [Err] if the parser isn't positioned at an assignment target without adding an error.
fn try_assignment_target(p: &mut Parser) -> Result<CompletedMarker, ()> {
	match p.cur() {
		T!['['] => Ok(array_assignment_target(p)),
		T!['{'] if p.state.allow_object_expr => Ok(object_assignment_target(p)),
		_ => try_simple_assignment_target(p),
	}
}

fn simple_assignment_target(p: &mut Parser) -> Option<CompletedMarker> {
	let result = try_simple_assignment_target(p).ok();

	if result == None {
		p.error(
			p.err_builder("Expected an identifier or a member expression, but found none")
				.primary(p.cur_tok().range, ""),
		);
	}

	result
}

fn try_simple_assignment_target(p: &mut Parser) -> Result<CompletedMarker, ()> {
	let checkpoint = p.checkpoint();
	let expr = conditional_expr(p).ok_or(())?;

	let assignment_target = try_expression_to_simple_assignment_target(p, expr);
	if assignment_target.is_err() {
		// This function shouldn't emit any error if it fails to parse the expression.
		// Ideally, rewind wouldn't be needed here because there's a try_expr function that tries to parse
		// the expression and otherwise returns an Error but doesn't add any diagnostics
		p.rewind(checkpoint);
	}

	assignment_target
}

fn assignment_target_with_optional_default(p: &mut Parser) -> Option<CompletedMarker> {
	let result = try_assignment_target_with_optional_default(p).ok();

	if result == None {
		p.error(missing_assignment_target_error(p));
	}

	result
}

/// Parses an assignment target and wraps it as a [JsAssignmentTargetWithDefault] if the target
/// is followed by a `=` token. Returns [Err] if the parser can't parse the assignment target nor
/// the default clause.
fn try_assignment_target_with_optional_default(p: &mut Parser) -> Result<CompletedMarker, ()> {
	let target = try_assignment_target(p);

	if p.at(T![=]) {
		let with_default = match target {
			Ok(target) => target.precede(p),
			Err(_) => {
				// It's possible to recover from the missing assignment target since the parser is still making
				// progress by eating the default part. Just make sure the parser marks the target as missing.
				p.error(missing_assignment_target_error(p));
				p.start()
			}
		};

		p.bump_any(); // eat the = token
		expr_or_assignment_target(p);

		Ok(with_default.complete(p, JS_ASSIGNMENT_TARGET_WITH_DEFAULT))
	} else {
		target
	}
}

/// Parses a [JsArrayAssignmentTarget] with all its elements
fn array_assignment_target(p: &mut Parser) -> CompletedMarker {
	let m = p.start();

	p.expect(T!['[']);
	let elements = p.start();

	while !p.at(EOF) && !p.at(T![']']) {
		if p.at(T![,]) {
			p.start().complete(p, SyntaxKind::JS_ARRAY_HOLE);
			p.bump_any();
			continue;
		}

		if p.at(T![...]) {
			array_assignment_target_rest_element(p);
			break;
		}

		let element = {
			let mut guard = p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...], T![=]]),
				..p.state.clone()
			});

			try_assignment_target_with_optional_default(&mut *guard)
		};

		if element.is_err() {
			// Use a custom error recovery here. The parser eats all tokens until it finds the end of the array (or file),
			// or any token that starts a new assignment target and puts them into an unknown assignment target
			p.error(missing_assignment_target_error(p));

			let unknown = p.start();
			while !p.at_ts(token_set!(EOF, T![,], T![']'], T![=], T![;], T![...]))
				&& !p.has_linebreak_before_n(0)
			{
				p.bump_any();
			}

			unknown.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
		}

		if !p.at(T![']']) {
			p.expect(T![,]);
		}
	}

	elements.complete(p, LIST);
	p.expect(T![']']);

	m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET)
}

fn array_assignment_target_rest_element(p: &mut Parser) {
	let m = p.start();
	p.expect(T![...]);

	assignment_target_with_optional_default(p);

	if p.eat(T![,]) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here"),
		);
		m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
	} else {
		m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET_REST_ELEMENT);
	}
}

/// Parses a [JsObjectAssignmentTarget]
fn object_assignment_target(p: &mut Parser) -> CompletedMarker {
	let m = p.start();

	p.expect(T!['{']);
	let elements = p.start();

	while !matches!(p.cur(), EOF | T!['}']) {
		if p.at(T![...]) {
			object_rest_property_assignment_target(p);
			break;
		}

		let element = try_property_assignment_target(p);

		if element.is_err() {
			p.error(
				p.err_builder("Expected a property name, but found none")
					.primary(p.cur_tok().range, ""),
			);

			let unknown = p.start();
			let recovery_set = token_set!(EOF, T![,], T![']'], T![...], T![;]);
			while !p.at_ts(recovery_set) && !p.has_linebreak_before_n(0) {
				p.bump_any();
			}

			unknown.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
		}

		if !p.at(T!['}']) {
			p.expect(T![,]);
		}
	}

	elements.complete(p, LIST);
	p.expect(T!['}']);

	m.complete(p, JS_OBJECT_ASSIGNMENT_TARGET)
}

const PROPERTY_ASSIGNMENT_TARGET_START_TOKENS: TokenSet =
	token_set![T![ident], T![yield], T![await], T![:], T![=]];

fn try_property_assignment_target(p: &mut Parser) -> Result<CompletedMarker, ()> {
	if !p.at_ts(PROPERTY_ASSIGNMENT_TARGET_START_TOKENS) {
		return Err(());
	}

	let m = p.start();
	let mut property_name = identifier_name(p)
		.expect("The parser is currently at an identifier, calling identifier_name should succeed");
	let is_shorthand_property = !p.eat(T![:]);

	if is_shorthand_property {
		property_name.change_kind(p, JS_IDENTIFIER_ASSIGNMENT_TARGET);
	} else {
		property_name.change_kind(p, JS_REFERENCE_IDENTIFIER_MEMBER);

		assignment_target(p);
	}

	{
		let mut guard = p.with_state(ParserState {
			expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
			..p.state.clone()
		});
		optional_equals_value_clause(&mut *guard);
	}

	Ok(m.complete(
		p,
		if is_shorthand_property {
			JS_SHORTHAND_PROPERTY_ASSIGNMENT_TARGET
		} else {
			JS_OBJECT_PROPERTY_ASSIGNMENT_TARGET
		},
	))
}

fn object_rest_property_assignment_target(p: &mut Parser) {
	let m = p.start();
	p.expect(T![...]);

	simple_assignment_target(p);

	if p.eat(T![,]) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here"),
		);
		m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
	} else {
		m.complete(p, JS_OBJECT_REST_PROPERTY_ASSIGNMENT_TARGET);
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
