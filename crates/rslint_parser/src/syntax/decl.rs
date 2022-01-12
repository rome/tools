//! Class and function declarations.

use super::binding::parse_binding_pattern;
use super::expr::parse_expr_or_assignment;
use super::typescript::*;
#[allow(deprecated)]
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::parser::ParserProgress;
use crate::state::{AllowObjectExpression, InFunction};
use crate::syntax::binding::parse_binding_pattern_with_optional_default;
use crate::syntax::function::function_body;
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::expected_binding;
use crate::{JsSyntaxKind::*, *};

#[allow(clippy::unnecessary_unwrap)]
pub(super) fn parse_formal_param_pat(p: &mut Parser) -> ParsedSyntax {
	if p.typescript() {
		if let Some(modifier) = maybe_eat_incorrect_modifier(p) {
			let err = p
				.err_builder("modifiers on parameters are only allowed in constructors")
				.primary(modifier.range(p), "");

			p.error(err);
		}
	}

	parse_binding_pattern_with_optional_default(p)
}

// test parameter_list
// function evalInComputedPropertyKey({ [computed]: ignored }) {}
/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['(']) {
		return Absent;
	}
	let m = p.start();
	parse_parameters_list(p, parse_formal_param_pat, JS_PARAMETER_LIST);
	Present(m.complete(p, JS_PARAMETERS))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
	p: &mut Parser,
	parse_param: impl Fn(&mut Parser) -> ParsedSyntax,
	list_kind: JsSyntaxKind,
) {
	let mut first = true;
	let has_l_paren = p.expect(T!['(']);

	p.with_state(AllowObjectExpression(has_l_paren), |p| {
		let parameters_list = p.start();
		let mut progress = ParserProgress::default();

		while !p.at(EOF) && !p.at(T![')']) {
			progress.assert_progressing(p);

			if first {
				first = false;
			} else {
				p.expect(T![,]);
			}

			if p.at(T![')']) {
				break;
			}

			if p.at(T![...]) {
				let m = p.start();
				p.bump_any();
				parse_binding_pattern(p).or_add_diagnostic(p, expected_binding);

				// TODO #1725 Review error handling and recovery
				// rest patterns cannot be optional: `...foo?: number[]`
				if p.at(T![?]) {
					let err = p
						.err_builder("rest patterns cannot be optional")
						.primary(p.cur_tok().range(), "");

					p.error(err);
					let m = p.start();
					p.bump_any();
					m.complete(p, JS_UNKNOWN_BINDING);
				}

				// type annotation `...foo: number[]`
				if p.eat(T![:]) {
					let complete = ts_type(p);
					if let Some(mut res) = complete {
						res.err_if_not_ts(
							p,
							"type annotations can only be used in TypeScript files",
						);
					}
				}

				if p.at(T![=]) {
					let start = p.cur_tok().start();
					let m = p.start();
					p.bump_any();

					let end = parse_expr_or_assignment(&mut *p)
						.ok()
						.map(|marker| usize::from(marker.range(p).end()))
						.unwrap_or_else(|| p.cur_tok().start());

					let err = p
						.err_builder("rest elements may not have default initializers")
						.primary(start..end, "");

					p.error(err);
					m.complete(p, JS_UNKNOWN);
				}

				m.complete(p, JS_REST_PARAMETER);

				// FIXME: this should be handled better, we should keep trying to parse params but issue an error for each one
				// which would allow for better recovery from `foo, ...bar, foo`
				if p.at(T![,]) {
					let m = p.start();
					let range = p.cur_tok().range();
					p.bump_any();
					m.complete(p, JS_UNKNOWN);
					let err = p
						.err_builder("rest elements may not have trailing commas")
						.primary(range, "");

					p.error(err);
				}
			} else {
				// test_err formal_params_no_binding_element
				// function foo(true) {}

				// test_err formal_params_invalid
				// function (a++, c) {}
				let recovered_result = parse_param(p).or_recover(
					p,
					&ParseRecovery::new(
						JS_UNKNOWN_BINDING,
						token_set![
							T![ident],
							T![await],
							T![yield],
							T![,],
							T!['['],
							T![...],
							T![')'],
						],
					)
					.enable_recovery_on_line_break(),
					js_parse_error::expected_parameter,
				);

				if recovered_result.is_err() {
					break;
				}
			}
		}

		parameters_list.complete(p, list_kind);
	});

	p.expect(T![')']);
}

pub(super) fn parse_arrow_body(p: &mut Parser) -> ParsedSyntax {
	p.with_state(InFunction(true), |p| {
		if p.at(T!['{']) {
			function_body(p)
		} else {
			parse_expr_or_assignment(p)
		}
	})
}
