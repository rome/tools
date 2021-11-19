use crate::parse_recoverer::ParseRecoverer;
use crate::syntax::decl::{formal_param_pat, parameter_list, BASE_METHOD_RECOVERY_SET};
use crate::syntax::expr::{assign_expr, expr, identifier_name, literal};
use crate::syntax::function::{function_body, ts_parameter_types, ts_return_type};
use crate::{CompletedMarker, Parser, ParserState, TokenSet};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::T;

const STARTS_OBJ_PROP: TokenSet = token_set![
	JS_STRING_LITERAL_TOKEN,
	JS_NUMBER_LITERAL_TOKEN,
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
	p.expect(T!['{']);
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

		object_member(p);
	}

	props_list.complete(p, LIST);

	p.expect(T!['}']);
	m.complete(p, JS_OBJECT_EXPRESSION)
}

/// An individual object property such as `"a": b` or `5: 6 + 6`.
fn object_member(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur() {
		// test object_expr_getter_setter
		// let a = {
		//  get foo() {
		//    return foo;
		//  }
		// }
		T![ident]
			if p.cur_src() == "get"
				&& !p.has_linebreak_before_n(1)
				&& STARTS_OBJ_PROP.contains(p.nth(1)) =>
		{
			Some(getter_object_member(p))
		}

		// test object_expr_getter_setter
		// let b = {
		//  set [foo](bar) {
		//     return 5;
		//  }
		// }
		T![ident]
			if p.cur_src() == "set"
				&& !p.has_linebreak_before_n(1)
				&& STARTS_OBJ_PROP.contains(p.nth(1)) =>
		{
			Some(setter_object_member(p))
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
			Some(m.complete(p, JS_SPREAD))
		}

		T![*] => {
			// test object_expr_generator_method
			// let b = { *foo() {} }
			method_object_member(p)
		}

		_ => {
			let m = p.start();
			let identifier_member_name = p.at(T![ident]) || p.cur().is_keyword();
			let member_name = object_member_name(p);

			// test object_expr_method
			// let b = {
			//  foo() {},
			// }
			if p.at(T!['(']) || p.at(T![<]) {
				method_object_member_body(p).ok()?;
				Some(m.complete(p, JS_METHOD_OBJECT_MEMBER))
			} else if let Some(mut member_name) = member_name {
				// test object_expr_assign_prop
				// let b = { foo = 4, foo = bar }
				if p.eat(T![=]) {
					member_name.change_kind(p, NAME);
					assign_expr(p);
					return Some(m.complete(p, INITIALIZED_PROP));
				}

				// ({foo})
				// test object_expr_ident_prop
				if identifier_member_name
					&& (matches!(p.cur(), T![,] | T!['}']) || p.has_linebreak_before_n(0))
				{
					member_name.change_kind(p, JS_REFERENCE_IDENTIFIER_EXPRESSION);
					Some(m.complete(p, JS_SHORTHAND_PROPERTY_OBJECT_MEMBER))
				} else {
					// let b = { a: true }
					// If the member name was a literal OR we're at a colon
					p.expect(T![:]);
					assign_expr(p);
					Some(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
				}
			} else {
				// test_err object_expr_error_prop_name
				// let a = { /: 6, /: /foo/ }
				// let a = {{}}
				// test_err object_expr_non_ident_literal_prop
				// let b = {5}

				ParseRecoverer::new(token_set![T![:], T![,]], ERROR).recover(p);

				if p.eat(T![:]) {
					assign_expr(p);
					Some(m.complete(p, JS_PROPERTY_OBJECT_MEMBER))
				} else {
					None
				}
			}
		}
	}
}

/// Parses a getter object member: `{ get a() { return "a"; } }`
fn getter_object_member(p: &mut Parser) -> CompletedMarker {
	debug_assert!(p.at(T![ident]), "Expected an identifier");
	debug_assert!(p.cur_src() == "get", "Expected a get identifier");

	let m = p.start();

	p.bump_remap(T![get]);

	object_member_name(p);

	p.expect(T!['(']);
	p.expect(T![')']);

	ts_return_type(p);

	function_body(p);

	m.complete(p, JS_GETTER_OBJECT_MEMBER)
}

/// Parses a setter object member like `{ set a(value) { .. } }`
fn setter_object_member(p: &mut Parser) -> CompletedMarker {
	debug_assert!(p.at(T![ident]), "Expected an identifier");
	debug_assert!(p.cur_src() == "set", "Expected a set identifier");

	let m = p.start();

	p.bump_remap(T![set]);

	object_member_name(p);

	p.state.allow_object_expr = p.expect(T!['(']);
	formal_param_pat(p);
	p.expect(T![')']);

	function_body(p);

	p.state.allow_object_expr = true;
	m.complete(p, JS_SETTER_OBJECT_MEMBER)
}

// test object_prop_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
pub fn object_prop_name(p: &mut Parser, binding: bool) -> Option<CompletedMarker> {
	match p.cur() {
		JS_STRING_LITERAL_TOKEN | JS_NUMBER_LITERAL_TOKEN => literal(p),
		T!['['] => {
			let m = p.start();
			p.bump_any();
			assign_expr(p);
			p.expect(T![']']);
			Some(m.complete(p, COMPUTED_PROPERTY_NAME))
		}
		_ if binding => super::pat::binding_identifier(p),
		_ => identifier_name(p),
	}
}

// test object_prop_name
// let a = {"foo": foo, [6 + 6]: foo, bar: foo, 7: foo}
/// Parses a `JsAnyObjectMemberName` and returns its completion marker
fn object_member_name(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur() {
		T!['['] => Some(computed_member_name(p)),
		_ => literal_member_name(p),
	}
}

pub(crate) fn computed_member_name(p: &mut Parser) -> CompletedMarker {
	let m = p.start();

	p.expect(T!['[']);
	expr(p);
	p.expect(T![']']);
	m.complete(p, JS_COMPUTED_MEMBER_NAME)
}

pub(super) fn literal_member_name(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let mut complete_with_this_kind = JS_LITERAL_MEMBER_NAME;
	match p.cur() {
		JS_STRING_LITERAL_TOKEN | JS_NUMBER_LITERAL_TOKEN | T![ident] => {
			p.bump_any();
		}
		t if t.is_keyword() => {
			p.bump_remap(T![ident]);
		}
		_ => {
			let err = p
				.err_builder("Expected an identifier, a keyword, or a string or number literal")
				.primary(
					p.cur_tok().range,
					"Expected an identifier, a keyword, or a string or number literal here",
				);
			p.error(err);
			return None;
		}
	}
	Some(m.complete(p, complete_with_this_kind))
}

/// Parses a method object member
fn method_object_member(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();

	// test async_method
	// class foo {
	//  async foo() {}
	//  async *foo() {}
	// }
	let state = if is_parser_at_async_method_member(p) {
		p.bump_remap(T![async]);

		ParserState {
			in_async: true,
			in_generator: p.eat(T![*]),
			..p.state.clone()
		}
	} else {
		ParserState {
			in_generator: p.eat(T![*]),
			..p.state.clone()
		}
	};

	let mut guard = p.with_state(state);
	object_member_name(&mut *guard);
	method_object_member_body(&mut *guard).ok()?;
	drop(guard);

	Some(m.complete(p, JS_METHOD_OBJECT_MEMBER))
}

/// Parses the body of a method object member starting right after the member name.
fn method_object_member_body(p: &mut Parser) -> Result<(), ()> {
	let old = p.state.to_owned();
	p.state.in_function = true;

	let result = if matches!(p.cur(), T!['('] | T![<]) {
		ts_parameter_types(p);
		parameter_list(p);
		ts_return_type(p);
		function_body(p);
		Ok(())
	} else {
		let err = p
			.err_builder("expected a method definition, but found none")
			.primary(p.cur_tok().range, "");

		ParseRecoverer::with_error(BASE_METHOD_RECOVERY_SET, ERROR, err).recover(p);
		Err(())
	};

	p.state = old;
	result
}

fn is_parser_at_async_method_member(p: &Parser) -> bool {
	p.cur() == T![ident]
		&& p.cur_src() == "async"
		&& !p.has_linebreak_before_n(1)
		&& (STARTS_OBJ_PROP.contains(p.nth(1)) || p.nth_at(1, T![*]))
}
