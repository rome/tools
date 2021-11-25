use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParsedSyntax;
use crate::parser::ParsedSyntax::{Absent, Present};
use crate::syntax::decl::{formal_param_pat, parameter_list};
use crate::syntax::expr::{assign_expr, expr, identifier_name, literal_expression};
use crate::syntax::function::{function_body, ts_parameter_types, ts_return_type};
use crate::syntax::js_parse_error;
use crate::{CompletedMarker, ParseRecovery, Parser, ParserState, TokenSet};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::T;

const STARTS_MEMBER_NAME: TokenSet = token_set![
	JS_STRING_LITERAL,
	JS_NUMBER_LITERAL,
	T![ident],
	T![await],
	T![yield],
	T!['[']
];

/// An object literal such as `{ a: b, "b": 5 + 5 }`.
// test object_expr
// let a = {};
// let b = {foo,}
pub(super) fn object_expr(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T!['{']);
	let props_list = p.start();
	let mut first = true;

	while !p.at(EOF) && !p.at(T!['}']) {
		if first {
			first = false;
		} else {
			p.expect(T![,]);

			if p.at(T!['}']) {
				break;
			}
		}

		let recovered_member = object_member(p).or_recover(
			p,
			ParseRecovery::new(JS_UNKNOWN_MEMBER, token_set![T![,], T!['}'], T![;], T![:]])
				.with_recovery_on_line_break(),
			js_parse_error::expected_object_member,
		);

		if recovered_member.is_err() {
			break;
		}
	}

	props_list.complete(p, LIST);

	p.expect_required(T!['}']);
	m.complete(p, JS_OBJECT_EXPRESSION)
}

/// An individual object property such as `"a": b` or `5: 6 + 6`.
fn object_member(p: &mut Parser) -> ParsedSyntax {
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
				&& STARTS_MEMBER_NAME.contains(p.nth(1)) =>
		{
			getter_object_member(p)
		}

		// test object_expr_setter
		// let b = {
		//  set [foo](bar) {
		//     return 5;
		//  }
		// }
		T![ident]
			if p.cur_src() == "set"
				&& !p.has_linebreak_before_n(1)
				&& STARTS_MEMBER_NAME.contains(p.nth(1)) =>
		{
			setter_object_member(p)
		}

		// test object_expr_async_method
		// let a = {
		//   async foo() {},
		//   async *foo() {}
		// }
		T![ident] if is_parser_at_async_method_member(p) => method_object_member(p),

		// test object_expr_spread_prop
		// let a = {...foo}
		T![...] => {
			let m = p.start();
			p.bump_any();
			assign_expr(p);
			Present(m.complete(p, JS_SPREAD))
		}

		T![*] => {
			// test object_expr_generator_method
			// let b = { *foo() {} }
			method_object_member(p)
		}

		_ => {
			let checkpoint = p.checkpoint();
			let m = p.start();
			let identifier_member_name = p.at(T![ident]) || p.cur().is_keyword();
			let member_name =
				object_member_name(p).make_required(p, js_parse_error::expected_object_member);

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
				method_object_member_body(p);
				Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
			} else if let Some(mut member_name) = member_name {
				// test object_expr_assign_prop
				// let b = { foo = 4, foo = bar }
				if p.eat(T![=]) {
					member_name.change_kind(p, NAME);
					assign_expr(p);
					return Present(m.complete(p, INITIALIZED_PROP));
				}

				// ({foo})
				// test object_expr_ident_prop
				if identifier_member_name
					&& (matches!(p.cur(), T![,] | T!['}']) || p.has_linebreak_before_n(0))
				{
					member_name.change_kind(p, JS_REFERENCE_IDENTIFIER_EXPRESSION);
					Present(m.complete(p, JS_SHORTHAND_PROPERTY_OBJECT_MEMBER))
				} else {
					// let b = { a: true }
					// If the member name was a literal OR we're at a colon
					p.expect_required(T![:]);
					assign_expr(p);
					Present(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
				}
			} else {
				// test_err object_expr_error_prop_name
				// let a = { /: 6, /: /foo/ }
				// let a = {{}}
				// test_err object_expr_non_ident_literal_prop
				// let b = {5}

				SingleTokenParseRecovery::new(token_set![T![:], T![,]], ERROR).recover(p);

				if p.eat(T![:]) {
					assign_expr(p);
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
fn getter_object_member(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![ident]) || p.cur_src() != "get" {
		return Absent;
	}

	let m = p.start();

	p.bump_remap(T![get]);

	object_member_name(p).make_required(p, js_parse_error::expected_object_member_name);

	p.expect_required(T!['(']);
	p.expect_required(T![')']);

	ts_return_type(p);

	function_body(p).make_required(p, js_parse_error::expected_function_body);

	Present(m.complete(p, JS_GETTER_OBJECT_MEMBER))
}

/// Parses a setter object member like `{ set a(value) { .. } }`
fn setter_object_member(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![ident]) || p.cur_src() != "set" {
		return Absent;
	}
	let m = p.start();

	p.bump_remap(T![set]);

	object_member_name(p).make_required(p, js_parse_error::expected_object_member_name);

	p.state.allow_object_expr = p.expect_required(T!['(']);
	formal_param_pat(p);
	p.expect_required(T![')']);

	function_body(p).make_required(p, js_parse_error::expected_function_body);

	p.state.allow_object_expr = true;
	Present(m.complete(p, JS_SETTER_OBJECT_MEMBER))
}

// test object_prop_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
pub fn object_prop_name(p: &mut Parser, binding: bool) -> Option<CompletedMarker> {
	match p.cur() {
		JS_STRING_LITERAL | JS_NUMBER_LITERAL => literal_expression(p),
		T!['['] => computed_member_name(p).ok(),
		_ if binding => super::pat::binding_identifier(p),
		_ => identifier_name(p),
	}
}

// test object_member_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
/// Parses a `JsAnyObjectMemberName` and returns its completion marker
fn object_member_name(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T!['['] => computed_member_name(p),
		_ => literal_member_name(p),
	}
}

fn is_at_object_member_name(p: &Parser) -> bool {
	p.at_ts(STARTS_MEMBER_NAME)
}

pub(crate) fn computed_member_name(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T!['[']) {
		return Absent;
	}

	let m = p.start();
	p.expect_required(T!['[']);
	expr(p);
	p.expect_required(T![']']);
	Present(m.complete(p, JS_COMPUTED_MEMBER_NAME))
}

pub(super) fn literal_member_name(p: &mut Parser) -> ParsedSyntax {
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
fn method_object_member(p: &mut Parser) -> ParsedSyntax {
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
	object_member_name(p).make_required(p, js_parse_error::expected_object_member_name);

	{
		let mut guard = p.with_state(ParserState {
			in_async: is_async,
			in_generator,
			..p.state.clone()
		});
		method_object_member_body(&mut *guard);
	}

	Present(m.complete(p, JS_METHOD_OBJECT_MEMBER))
}

/// Parses the body of a method object member starting right after the member name.
fn method_object_member_body(p: &mut Parser) {
	let old = p.state.to_owned();
	p.state.in_function = true;

	ts_parameter_types(p);
	parameter_list(p);
	ts_return_type(p);
	function_body(p).make_required(p, js_parse_error::expected_function_body);

	p.state = old;
}

fn is_parser_at_async_method_member(p: &Parser) -> bool {
	p.cur() == T![ident]
		&& p.cur_src() == "async"
		&& !p.has_linebreak_before_n(1)
		&& (STARTS_MEMBER_NAME.contains(p.nth(1)) || p.nth_at(1, T![*]))
}
