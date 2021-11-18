use crate::syntax::expr::{expr_or_assignment_target, identifier_name};
use crate::{CompletedMarker, Parser};
use crate::{SyntaxKind::*, *};

/// Converts the passed in target (expression) to an assignment target
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub(crate) fn expression_to_assignment_target(
	p: &mut Parser,
	target: CompletedMarker,
	checkpoint: Checkpoint,
) -> Option<CompletedMarker> {
	if let Ok(assignment_target) = try_fast_expression_to_assignment_target_conversion(p, target) {
		return Some(assignment_target);
	}
	p.rewind(checkpoint);

	assignment_target(p)
}

/// Parses an assignment target and inserts and creates an unknown assignment target if
/// the current token isn't a valid start for an assignment target
pub(crate) fn assignment_target(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur() {
		T![ident] | T![yield] | T![await] => identifier_assignment_target(p),
		T!['['] => Some(array_assignment_target(p)),
		// TODO object target
		_ => {
			let err = p
				.err_builder("Expected an identifier or an assignment pattern but found none")
				.primary(p.cur_tok().range, "");

			// TODO change the error kind to `UnknownAssignmentTarget`
			p.err_recover(
				err,
				token_set![T![ident], T![yield], T![await], T!['['],],
				p.state.allow_object_expr,
			);
			None
		}
	}
}

/// Parses an assignment target and wraps it as [JsAssignmentTargetWithDefault] if the target
/// is followed by a `=` token
fn assignment_target_with_optional_default(p: &mut Parser) -> Option<CompletedMarker> {
	let target = assignment_target(p);

	if p.at(T![=]) {
		let with_default = if let Some(target) = target {
			target.precede(p)
		} else {
			p.start()
		};

		p.bump_any(); // eat the = token
		expr_or_assignment_target(p);

		Some(with_default.complete(p, JS_ASSIGNMENT_TARGET_WITH_DEFAULT))
	} else {
		target
	}
}

/// Parses an identifier assignment target
fn identifier_assignment_target(p: &mut Parser) -> Option<CompletedMarker> {
	identifier_name(p).map(|mut identifier| {
		identifier.change_kind(p, JS_IDENTIFIER_ASSIGNMENT_TARGET);
		identifier
	})
}

/// Parses an array assignment target with all its elements
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

		let element = assignment_target_with_optional_default(p);

		if element.is_none() {
			p.err_recover_no_err(
				token_set![T![,], T![']']],
				false,
				// TODO create an UnknownAssignmentTarget node
			);
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

	if p.at(T![,]) {
		p.error(
			p.err_builder("rest element may not have a trailing comma")
				.primary(p.cur_tok().range, "Remove the trailing comma here"),
		);
		m.complete(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
	} else {
		m.complete(p, JS_ARRAY_ASSIGNMENT_TARGET_REST_ELEMENT);
	}
}

fn try_fast_expression_to_assignment_target_conversion(
	p: &mut Parser,
	mut target: CompletedMarker,
) -> Result<CompletedMarker, ()> {
	let result = match target.kind() {
		JS_STATIC_MEMBER_EXPRESSION => {
			let children =
				&p.events.as_slice()[target.start_pos as usize..target.finish_pos as usize];
			if children.iter().any(|event| {
				matches!(
					event,
					Event::Start {
						kind: JS_REFERENCE_PRIVATE_MEMBER,
						..
					}
				)
			}) {
				// Private members are not valid inside assignment targets, bail out and force a reparse
				Err(())
			} else {
				Ok(JS_STATIC_MEMBER_ASSIGNMENT_TARGET)
			}
		}
		JS_COMPUTED_MEMBER_EXPRESSION => Ok(JS_COMPUTED_MEMBER_ASSIGNMENT_TARGET),
		JS_REFERENCE_IDENTIFIER_EXPRESSION => Ok(JS_IDENTIFIER_ASSIGNMENT_TARGET),
		_ => Err(()),
	};

	match result {
		Ok(new_kind) => {
			target.change_kind(p, new_kind);
			Ok(target)
		}
		Err(_) => {
			target.undo_completion(p).abandon(p);
			Err(())
		}
	}
}

//
// fn map_target_assignment_events(p: &mut Parser, target: &CompletedMarker) {
// 	// let mut pending_events: Vec<Pending> = Vec::new();
//
// 	// This is to naive... the LHS may contain some array expressions, for example in the default initializers
// 	for event in p.events[target.start_pos as usize..target.finish_pos as usize].iter_mut() {
// 		match event {
// 			Event::Start {
// 				kind: TOMBSTONE | LIST,
// 				..
// 			} => {}
// 			Event::Start { kind, .. } => {
// 				if let Ok(mapped_kind) = try_map_kind_to_assignment_target(*kind) {
// 					*kind = mapped_kind;
// 				} else {
// 					*kind = JS_UNKNOWN_ASSIGNMENT_TARGET;
// 					// TODO insert error
// 					// target.change_kind(p, JS_UNKNOWN_ASSIGNMENT_TARGET);
// 				}
// 			}
// 			Event::Finish { .. } => {}
//
// 			Event::Token { .. } | Event::Missing | Event::MultipleTokens { .. } => {}
// 		}
// 	}
// }
