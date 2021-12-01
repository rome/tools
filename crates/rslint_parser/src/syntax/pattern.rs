use crate::parser::{ParserProgress, RecoveryError};
use crate::syntax::expr::{expr_or_assignment, EXPR_RECOVERY_SET};
use crate::syntax::js_parse_error::expected_assignment_target;
use crate::ParsedSyntax::{Absent, Present};
use crate::TokenSet;
use crate::{CompletedMarker, ParseRecovery, ParsedSyntax, Parser, ParserState};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind::{EOF, JS_ARRAY_HOLE, LIST};
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

pub(crate) trait ParseWithDefaultPattern {
	fn parse_pattern_with_optional_default(&self, p: &mut Parser) -> ParsedSyntax {
		let pattern = self.parse_pattern(p);

		if p.at(T![=]) {
			let with_default =
				pattern.precede_or_missing_with_error(p, Self::expected_pattern_error);
			p.bump_any(); // eat the = token
			expr_or_assignment(p);
			Present(with_default.complete(p, self.pattern_with_default_kind()))
		} else {
			pattern
		}
	}

	fn pattern_with_default_kind(&self) -> SyntaxKind;
	fn expected_pattern_error(p: &Parser, range: Range<usize>) -> Diagnostic;
	fn parse_pattern(&self, p: &mut Parser) -> ParsedSyntax;
}

pub(crate) trait ParseArrayPattern<P: ParseWithDefaultPattern> {
	fn parse_array_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T!['[']) {
			return Absent;
		}

		let m = p.start();

		p.bump(T!['[']);
		let elements = p.start();
		let mut progress = ParserProgress::default();

		{
			let guard = &mut *p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...], T![=]]),
				..p.state.clone()
			});

			while !guard.at(EOF) && !guard.at(T![']']) {
				progress.assert_progressing(guard);

				let recovery = ParseRecovery::new(
					self.unknown_pattern_kind(),
					token_set!(EOF, T![,], T![']'], T![=], T![;], T![...]),
				)
				.enable_recovery_on_line_break();

				let element = self.parse_any_array_element(guard, &recovery).or_recover(
					guard,
					&recovery,
					Self::expected_element_error,
				);

				if element.is_err() {
					// Failed to recover
					break;
				}

				if !guard.at(T![']']) {
					guard.expect_required(T![,]);
				}
			}
		}

		elements.complete(p, LIST);
		p.expect(T![']']);

		Present(m.complete(p, self.array_pattern_kind()))
	}

	fn unknown_pattern_kind(&self) -> SyntaxKind;
	fn array_pattern_kind(&self) -> SyntaxKind;
	fn rest_pattern_kind(&self) -> SyntaxKind;
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic;
	fn pattern_with_default(&self) -> P;

	fn parse_any_array_element(&self, p: &mut Parser, recovery: &ParseRecovery) -> ParsedSyntax {
		match p.cur() {
			T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
			T![...] => match self.parse_rest_pattern(p) {
				Present(rest_pattern) => {
					validate_rest_pattern(
						p,
						rest_pattern,
						T![']'],
						recovery,
						self.unknown_pattern_kind(),
					);
					Present(rest_pattern)
				}
				Absent => Absent,
			},
			_ => self
				.pattern_with_default()
				.parse_pattern_with_optional_default(p),
		}
	}

	fn parse_rest_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T![...]) {
			return Absent;
		}

		let m = p.start();
		let rest_end = p.cur_tok().range.end;
		p.bump(T![...]);

		let with_default = self.pattern_with_default();

		with_default
			.parse_pattern(p)
			.or_missing_with_error(p, |p, _| P::expected_pattern_error(p, rest_end..rest_end));

		Present(m.complete(p, self.rest_pattern_kind()))
	}
}

pub(crate) trait ParseObjectPattern {
	fn parse_object_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T!['{']) {
			return Absent;
		}

		let m = p.start();

		p.bump(T!['{']);
		let elements = p.start();
		let mut progress = ParserProgress::default();

		{
			// TODO remove after migrating expression to `ParsedSyntax`
			let guard = &mut *p.with_state(ParserState {
				expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![T![,], T![...]]),
				..p.state.clone()
			});

			while !guard.at(T!['}']) {
				progress.assert_progressing(guard);

				if guard.at(T![,]) {
					// missing element
					guard.missing();
					guard.error(self.expected_property_pattern_error(guard, guard.cur_tok().range));
					guard.bump_any(); // bump ,
					continue;
				}
				let recovery_set = ParseRecovery::new(
					self.unknown_pattern_kind(),
					token_set!(EOF, T![,], T!['}'], T![...], T![;]),
				)
				.enable_recovery_on_line_break();

				let recover_result = self
					.parse_any_property_pattern(guard, &recovery_set)
					.or_recover(guard, &recovery_set, |p, range| {
						self.expected_property_pattern_error(p, range)
					});

				if recover_result.is_err() {
					break;
				}

				match recover_result {
					Err(RecoveryError::Eof) => break,
					Err(RecoveryError::AlreadyRecovered) => {
						// TODO
						guard.error(expected_assignment_target(guard, guard.cur_tok().range));
						break;
					}
					_ => {}
				}

				if !guard.at(T!['}']) {
					guard.expect_required(T![,]);
				}
			}
		}

		elements.complete(p, LIST);
		p.expect(T!['}']);

		Present(m.complete(p, self.object_pattern_kind()))
	}

	fn unknown_pattern_kind(&self) -> SyntaxKind;
	fn object_pattern_kind(&self) -> SyntaxKind;

	fn expected_property_pattern_error(&self, p: &Parser, range: Range<usize>) -> Diagnostic;

	fn parse_any_property_pattern(&self, p: &mut Parser, recovery: &ParseRecovery) -> ParsedSyntax {
		if p.at(T![...]) {
			match self.parse_rest_property_pattern(p) {
				Present(rest_pattern) => {
					validate_rest_pattern(
						p,
						rest_pattern,
						T!['}'],
						recovery,
						self.unknown_pattern_kind(),
					);
					Present(rest_pattern)
				}
				Absent => Absent,
			}
		} else {
			self.parse_property_pattern(p)
		}
	}

	fn parse_property_pattern(&self, p: &mut Parser) -> ParsedSyntax;

	fn parse_rest_property_pattern(&self, p: &mut Parser) -> ParsedSyntax;
}

/// Validates if the parsed completed rest marker is a valid rest element inside of a
/// array or object assignment target and converts it to an unknown assignment target if not.
/// A rest element must be:
///
/// * the last element
/// * not followed by a trailing comma
/// * not have a default value
fn validate_rest_pattern(
	p: &mut Parser,
	mut rest: CompletedMarker,
	end_token: SyntaxKind,
	recovery: &ParseRecovery,
	unknown_kind: SyntaxKind,
) {
	if p.at(end_token) {
		return;
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
		return;
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
}
