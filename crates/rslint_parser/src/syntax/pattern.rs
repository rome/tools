use crate::parser::RecoveryError;
use crate::syntax::expr::{expr_or_assignment, EXPR_RECOVERY_SET};
use crate::syntax::js_parse_error::expected_assignment_target;
use crate::ParsedSyntax::{Absent, Present};
use crate::TokenSet;
use crate::{CompletedMarker, ParseRecovery, ParsedSyntax, Parser, ParserState};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind::{EOF, JS_ARRAY_HOLE, LIST};
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

pub(crate) trait PatternWithDefault {
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

pub(crate) trait ArrayPattern<P: PatternWithDefault> {
	fn parse_array_pattern(&self, p: &mut Parser) -> ParsedSyntax {
		if !p.at(T!['[']) {
			return Absent;
		}

		let m = p.start();

		p.bump(T!['[']);
		let elements = p.start();

		while !p.at(EOF) && !p.at(T![']']) {
			let recovery = ParseRecovery::new(
				self.unknown_pattern_kind(),
				token_set!(EOF, T![,], T![']'], T![=], T![;], T![...]),
			)
			.enable_recovery_on_line_break();

			if let Present(rest) = self.parse_rest_pattern(p) {
				if validate_rest_pattern(p, rest, T![']'], &recovery, self.unknown_pattern_kind()) {
					break;
				}
			} else {
				let element = {
					let mut guard = p.with_state(ParserState {
						expr_recovery_set: EXPR_RECOVERY_SET.union(token_set![
							T![,],
							T![...],
							T![=]
						]),
						..p.state.clone()
					});

					self.parse_any_array_element(&mut *guard).or_recover(
						&mut *guard,
						&recovery,
						Self::expected_element_error,
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

		Present(m.complete(p, self.array_pattern_kind()))
	}

	fn unknown_pattern_kind(&self) -> SyntaxKind;
	fn array_pattern_kind(&self) -> SyntaxKind;
	fn rest_pattern_kind(&self) -> SyntaxKind;
	fn expected_element_error(p: &Parser, range: Range<usize>) -> Diagnostic;
	fn pattern_with_default(&self) -> P;

	fn parse_any_array_element(&self, p: &mut Parser) -> ParsedSyntax {
		match p.cur() {
			T![,] => Present(p.start().complete(p, JS_ARRAY_HOLE)),
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
		p.bump(T![...]);

		let with_default = self.pattern_with_default();

		with_default
			.parse_pattern(p)
			.or_missing_with_error(p, P::expected_pattern_error);

		Present(m.complete(p, self.rest_pattern_kind()))
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
						// TODO
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
