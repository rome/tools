//! Class and function declarations.

use super::expr::expr_or_assignment;
use super::pat::pattern;
use super::typescript::*;
#[allow(deprecated)]
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::parser::ParserProgress;
use crate::syntax::function::function_body;
use crate::syntax::js_parse_error;
use crate::{SyntaxKind::*, *};

#[allow(clippy::unnecessary_unwrap)]
pub(super) fn parse_formal_param_pat(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	if p.typescript() {
		if let Some(modifier) = maybe_eat_incorrect_modifier(p) {
			let err = p
				.err_builder("modifiers on parameters are only allowed in constructors")
				.primary(modifier.range(p), "");

			p.error(err);
		}
	}

	let checkpoint = p.checkpoint();
	let pat = if let Some(pattern) = pattern(p, true) {
		pattern
	} else {
		p.rewind(checkpoint);

		m.abandon(p);
		// TODO: not correct in case there was any typescript modifier. Revisit when patterns are refactored
		return ParsedSyntax::Absent;
	};

	let pat_range = pat.range(p);
	let mut kind = pat.kind();
	pat.undo_completion(p).abandon(p);

	let mut opt = None;

	if p.at(T![?]) {
		opt = Some(p.cur_tok().range);
		let range = p.cur_tok().range;
		match kind {
			JS_IDENTIFIER_BINDING | ARRAY_PATTERN | JS_OBJECT_BINDING => {
				p.bump_any();
			}
			_ if p.state.in_declare => {
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
			}
			_ => {
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
				let err = p
					.err_builder("Binding patterns cannot be optional")
					.primary(pat_range, "");

				p.error(err);
			}
		}
		if !p.typescript() {
			let err = p
				.err_builder(
					"optional parameter syntax with `?` can only be used in TypeScript files",
				)
				.primary(range, "");

			p.error(err);
		}
	}
	maybe_ts_type_annotation(p);
	if p.at(T![=]) {
		let start = p.cur_tok().range.start;
		p.bump_any();

		let expr = expr_or_assignment(p);
		let end = expr
			.map(|x| usize::from(x.range(p).end()))
			.unwrap_or_else(|| p.cur_tok().range.start);
		if let Some(range) = opt {
			let err = p
				.err_builder("optional parameters cannot have initializers")
				.primary(start..end, "")
				.secondary(range, "");

			p.error(err);
		}

		kind = ASSIGN_PATTERN;
	}
	ParsedSyntax::Present(m.complete(p, kind))
}

/// parse the whole list of parameters, brackets included
pub(super) fn parse_parameter_list(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['(']) {
		return Absent;
	}
	let m = p.start();
	parse_parameters_list(p, parse_formal_param_pat);
	Present(m.complete(p, JS_PARAMETER_LIST))
}

/// Parses a (param, param) list into the current active node
pub(super) fn parse_parameters_list(
	p: &mut Parser,
	parse_param: impl Fn(&mut Parser) -> ParsedSyntax,
) {
	let mut first = true;

	p.state.allow_object_expr = p.expect_required(T!['(']);

	let parameters_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) && !p.at(T![')']) {
		progress.assert_progressing(p);

		if first {
			first = false;
		} else {
			p.expect_required(T![,]);
		}

		if p.at(T![')']) {
			break;
		}

		if p.at(T![...]) {
			let m = p.start();
			p.bump_any();
			pattern(p, true);

			// rest patterns cannot be optional: `...foo?: number[]`
			if p.at(T![?]) {
				let err = p
					.err_builder("rest patterns cannot be optional")
					.primary(p.cur_tok().range, "");

				p.error(err);
				let m = p.start();
				p.bump_any();
				m.complete(p, JS_UNKNOWN_PATTERN);
			}

			// type annotation `...foo: number[]`
			if p.eat(T![:]) {
				let complete = ts_type(p);
				if let Some(mut res) = complete {
					res.err_if_not_ts(p, "type annotations can only be used in TypeScript files");
				}
			}

			if p.at(T![=]) {
				let start = p.cur_tok().range.start;
				let m = p.start();
				p.bump_any();
				let expr = expr_or_assignment(&mut *p);
				let end = expr
					.map(|x| usize::from(x.range(p).end()))
					.unwrap_or_else(|| p.cur_tok().range.start);
				let err = p
					.err_builder("rest elements may not have default initializers")
					.primary(start..end, "");

				p.error(err);
				m.complete(p, ERROR);
			}

			m.complete(p, JS_REST_PARAMETER);

			// FIXME: this should be handled better, we should keep trying to parse params but issue an error for each one
			// which would allow for better recovery from `foo, ...bar, foo`
			if p.at(T![,]) {
				let m = p.start();
				let range = p.cur_tok().range;
				p.bump_any();
				m.complete(p, ERROR);
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
				ParseRecovery::new(
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
			if let Ok(recovered_result) = recovered_result {
				if recovered_result.kind() == ASSIGN_PATTERN
					&& p.state.in_binding_list_for_signature
				{
					let err = p
						.err_builder(
							"assignment patterns cannot be used in function/constructor types",
						)
						.primary(recovered_result.range(p), "");

					p.error(err);
				}
			}
		}
	}

	parameters_list.complete(p, LIST);
	p.state.allow_object_expr = true;
	p.expect_required(T![')']);
}

pub(super) fn parse_arrow_body(p: &mut Parser) -> ParsedSyntax {
	let mut guard = p.with_state(ParserState {
		in_function: true,
		..p.state.clone()
	});
	if guard.at(T!['{']) {
		function_body(&mut *guard)
	} else {
		expr_or_assignment(&mut *guard).into()
	}
}
