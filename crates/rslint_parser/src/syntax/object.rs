#[allow(deprecated)]
use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::parser::{ParsedSyntax, RecoveryResult};
use crate::syntax::decl::{parse_formal_param_pat, parse_parameter_list};
use crate::syntax::expr::{expr, expr_or_assignment};
use crate::syntax::function::{
	function_body, parse_ts_parameter_types, parse_ts_return_type_if_ts,
};
use crate::syntax::js_parse_error;
use crate::CompletedNodeOrMissingMarker::NodeMarker;
use crate::{CompletedMarker, ParseRecovery, ParseSeparatedList, Parser, ParserState, TokenSet};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};

// test object_expr
// let a = {};
// let b = {foo,}
//
// test_err object_expr_err
// let a = {, foo}
// let b = { foo bar }

struct ObjectMembersList;

impl ParseSeparatedList for ObjectMembersList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		parse_object_member(p)
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at(T!['}'])
	}

	fn recover(
		&mut self,
		p: &mut Parser,
		parsed_element: ParsedSyntax<Self::ParsedElement>,
	) -> RecoveryResult {
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_MEMBER, token_set![T![,], T!['}'], T![;], T![:]])
				.enable_recovery_on_line_break(),
			js_parse_error::expected_object_member,
		)
	}

	fn list_kind() -> SyntaxKind {
		JS_OBJECT_MEMBER_LIST
	}

	fn separating_element_kind(&mut self) -> SyntaxKind {
		T![,]
	}

	fn allow_trailing_separating_element(&self) -> bool {
		true
	}
}

/// An object literal such as `{ a: b, "b": 5 + 5 }`.
pub(super) fn parse_object_expression(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['{']) {
		return Absent;
	}
	let m = p.start();
	p.bump(T!['{']);

	ObjectMembersList.parse_list(p);

	p.expect_required(T!['}']);
	Present(m.complete(p, JS_OBJECT_EXPRESSION))
}

/// An individual object property such as `"a": b` or `5: 6 + 6`.
fn parse_object_member(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		// test object_expr_getter
		// let a = {
		//  get foo() {
		//    return foo;
		//  }
		// }
		T![ident]
			if p.cur_src() == "get"
				&& !p.has_linebreak_before_n(1)
				&& is_nth_at_object_member_name(p, 1) =>
		{
			parse_getter_object_member(p)
		}

		// test object_expr_setter
		// let b = {
		//  set [foo](bar) {
		//     return 5;
		//  }
		// }

		// test_err object_expr_setter
		// let b = {
		//  set foo() {
		//     return 5;
		//  }
		// }
		T![ident]
			if p.cur_src() == "set"
				&& !p.has_linebreak_before_n(1)
				&& is_nth_at_object_member_name(p, 1) =>
		{
			parse_setter_object_member(p)
		}

		// test object_expr_async_method
		// let a = {
		//   async foo() {},
		//   async *foo() {}
		// }
		T![ident] if is_parser_at_async_method_member(p) => parse_method_object_member(p),

		// test object_expr_spread_prop
		// let a = {...foo}
		T![...] => {
			let m = p.start();
			p.bump_any();
			expr_or_assignment(p);
			Present(m.complete(p, JS_SPREAD))
		}

		T![*] => {
			// test object_expr_generator_method
			// let b = { *foo() {} }
			parse_method_object_member(p)
		}

		_ => {
			let checkpoint = p.checkpoint();
			let m = p.start();
			let async_missing = p.missing();
			let generator_missing = p.missing();
			let identifier_member_name =
				matches!(p.cur(), T![ident] | T![await] | T![yield]) || p.cur().is_keyword();
			let member_name = parse_object_member_name(p)
				.or_missing_with_error(p, js_parse_error::expected_object_member);

			// test object_expr_method
			// let b = {
			// foo() {},
			// "bar"(a, b, c) {},
			// ["foo" + "bar"](a) {},
			// 5(...rest) {}
			// }

			// test_err object_expr_method
			// let b = { foo) }
			if p.at(T!['(']) || p.at(T![<]) {
				parse_method_object_member_body(p);
				Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
			} else if let NodeMarker(mut member_name) = member_name {
				async_missing.undo(p);
				generator_missing.undo(p);

				// ({foo})
				// test object_expr_ident_prop
				if identifier_member_name
					&& (matches!(p.cur(), T![,] | T!['}']) || p.has_linebreak_before_n(0))
				{
					member_name.change_kind(p, JS_REFERENCE_IDENTIFIER);
					Present(m.complete(p, JS_SHORTHAND_PROPERTY_OBJECT_MEMBER))
				} else {
					// let b = { a: true }
					// If the member name was a literal OR we're at a colon
					p.expect_required(T![:]);
					if expr_or_assignment(p).is_none() {
						p.missing();
					}
					Present(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
				}
			} else {
				async_missing.undo(p);
				generator_missing.undo(p);

				// test_err object_expr_error_prop_name
				// let a = { /: 6, /: /foo/ }
				// let a = {{}}

				// test_err object_expr_non_ident_literal_prop
				// let b = {5}

				#[allow(deprecated)]
				SingleTokenParseRecovery::new(token_set![T![:], T![,]], ERROR).recover(p);

				if p.eat(T![:]) {
					expr_or_assignment(p);
					Present(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
				} else {
					// It turns out that this isn't a valid member after all. Make sure to throw
					// away everything that has been parsed so far so that the caller can
					// do it's error recovery
					m.abandon(p);
					p.rewind(checkpoint);
					Absent
				}
			}
		}
	}
}

/// Parses a getter object member: `{ get a() { return "a"; } }`
fn parse_getter_object_member(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![ident]) || p.cur_src() != "get" {
		return Absent;
	}

	let m = p.start();

	p.bump_remap(T![get]);

	parse_object_member_name(p)
		.or_missing_with_error(p, js_parse_error::expected_object_member_name);

	p.expect_required(T!['(']);
	p.expect_required(T![')']);

	parse_ts_return_type_if_ts(p).or_missing(p);

	function_body(p).or_missing_with_error(p, js_parse_error::expected_function_body);

	Present(m.complete(p, JS_GETTER_OBJECT_MEMBER))
}

/// Parses a setter object member like `{ set a(value) { .. } }`
fn parse_setter_object_member(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![ident]) || p.cur_src() != "set" {
		return Absent;
	}
	let m = p.start();

	p.bump_remap(T![set]);

	parse_object_member_name(p)
		.or_missing_with_error(p, js_parse_error::expected_object_member_name);

	p.state.allow_object_expr = p.expect_required(T!['(']);
	parse_formal_param_pat(p).or_missing_with_error(p, js_parse_error::expected_parameter);
	p.expect_required(T![')']);

	function_body(p).or_missing_with_error(p, js_parse_error::expected_function_body);

	p.state.allow_object_expr = true;
	Present(m.complete(p, JS_SETTER_OBJECT_MEMBER))
}

// test object_member_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
/// Parses a `JsAnyObjectMemberName` and returns its completion marker
pub(crate) fn parse_object_member_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		T!['['] => parse_computed_member_name(p),
		_ => parse_literal_member_name(p),
	}
}

fn is_nth_at_object_member_name(p: &Parser, offset: usize) -> bool {
	let nth = p.nth(offset);

	let start_names = token_set![
		JS_STRING_LITERAL,
		JS_NUMBER_LITERAL,
		T![ident],
		T![await],
		T![yield],
		T!['[']
	];

	nth.is_keyword() || start_names.contains(nth)
}

pub(crate) fn is_at_object_member_name(p: &Parser) -> bool {
	is_nth_at_object_member_name(p, 0)
}

pub(crate) fn parse_computed_member_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['[']) {
		return Absent;
	}

	let m = p.start();
	p.expect_required(T!['[']);
	expr(p);
	p.expect_required(T![']']);
	Present(m.complete(p, JS_COMPUTED_MEMBER_NAME))
}

pub(super) fn parse_literal_member_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	let m = p.start();
	match p.cur() {
		JS_STRING_LITERAL | JS_NUMBER_LITERAL | T![ident] => {
			p.bump_any();
		}
		t if t.is_keyword() => {
			p.bump_remap(T![ident]);
		}
		_ => {
			m.abandon(p);
			return Absent;
		}
	}
	Present(m.complete(p, JS_LITERAL_MEMBER_NAME))
}

/// Parses a method object member
fn parse_method_object_member(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	let is_async = is_parser_at_async_method_member(p);
	if !is_async && !p.at(T![*]) && !is_at_object_member_name(p) {
		return Absent;
	}

	let m = p.start();

	// test async_method
	// class foo {
	//  async foo() {}
	//  async *foo() {}
	// }
	if is_async {
		p.bump_remap(T![async]);
	} else {
		p.missing();
	}

	let in_generator = p.eat_optional(T![*]);
	parse_object_member_name(p)
		.or_missing_with_error(p, js_parse_error::expected_object_member_name);

	{
		let mut guard = p.with_state(ParserState {
			in_async: is_async,
			in_generator,
			..p.state.clone()
		});
		parse_method_object_member_body(&mut *guard);
	}

	Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
}

/// Parses the body of a method object member starting right after the member name.
fn parse_method_object_member_body(p: &mut Parser) {
	let old = p.state.to_owned();
	p.state.in_function = true;

	parse_ts_parameter_types(p).or_missing(p);
	parse_parameter_list(p).or_missing_with_error(p, js_parse_error::expected_parameters);
	parse_ts_return_type_if_ts(p).or_missing(p);
	function_body(p).or_missing_with_error(p, js_parse_error::expected_function_body);

	p.state = old;
}

fn is_parser_at_async_method_member(p: &Parser) -> bool {
	p.cur() == T![ident]
		&& p.cur_src() == "async"
		&& !p.has_linebreak_before_n(1)
		&& (is_nth_at_object_member_name(p, 1) || p.nth_at(1, T![*]))
}
