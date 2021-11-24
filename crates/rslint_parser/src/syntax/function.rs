use crate::parser::{ParseResult, ParsedSyntax};
use crate::syntax::decl::parameter_list;
use crate::syntax::pat::opt_binding_identifier;
use crate::syntax::stmt::{block_impl, is_semi};
use crate::syntax::typescript::{ts_type_or_type_predicate_ann, ts_type_params};
use crate::syntax::JsParseErrors;
use crate::{CompletedMarker, Parser, ParserState};
use rslint_syntax::SyntaxKind::{
	ERROR, JS_FUNCTION_BODY, JS_FUNCTION_DECLARATION, JS_FUNCTION_EXPRESSION,
	JS_IDENTIFIER_BINDING, TS_TYPE_ANNOTATION,
};
use rslint_syntax::{SyntaxKind, T};
use std::collections::HashMap;

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test function_decl
// function foo() {}
// function *foo() {}
// function foo(await) {}
// async function *foo() {}
// async function foo() {}
// function *foo() {
//   yield foo;
// }
pub(super) fn function_declaration(p: &mut Parser) -> CompletedMarker {
	function(p, JS_FUNCTION_DECLARATION)
}

pub(super) fn function_expression(p: &mut Parser) -> CompletedMarker {
	function(p, JS_FUNCTION_EXPRESSION)
}

fn function(p: &mut Parser, kind: SyntaxKind) -> CompletedMarker {
	let m = p.start();

	if kind == JS_FUNCTION_DECLARATION {
		// TS function declaration
		p.eat(T![declare]);
	}

	let in_async = p.at(T![ident]) && p.cur_src() == "async";
	if in_async {
		p.bump_remap(T![async]);
	}

	p.expect_required(T![function]);

	let in_generator = p.eat(T![*]);
	let guard = &mut *p.with_state(ParserState {
		labels: HashMap::new(),
		in_function: true,
		in_async,
		in_generator,
		..p.state.clone()
	});

	let id = opt_binding_identifier(guard);

	if let Some(mut identifier_marker) = id {
		identifier_marker.change_kind(guard, JS_IDENTIFIER_BINDING);
	} else if kind == JS_FUNCTION_DECLARATION {
		let err = guard
			.err_builder(
				"expected a name for the function in a function declaration, but found none",
			)
			.primary(guard.cur_tok().range, "");

		guard.error(err);
	}

	ts_parameter_types(guard);
	parameter_list(guard);
	ts_return_type(guard);

	if kind == JS_FUNCTION_DECLARATION {
		function_body_or_declaration(guard);
	} else {
		function_body(guard).make_required(guard, JsParseErrors::expected_function_body);
	}

	m.complete(guard, kind)
}

pub(super) fn function_body(p: &mut Parser) -> ParseResult {
	let mut guard = p.with_state(ParserState {
		in_constructor: false,
		in_function: true,
		..p.state.clone()
	});

	block_impl(&mut *guard, JS_FUNCTION_BODY)
}

// TODO 1725 This is probably not ideal (same with the `declare` keyword). We should
// use a different AST type for function declarations. For example, a function declaration should
// never have a body but that would be allowed with this approach. Same for interfaces, interface
// methods should never have a body
/// Either parses a typescript declaration body or the function body
pub(super) fn function_body_or_declaration(p: &mut Parser) {
	// omitting the body is allowed in ts
	if p.typescript() && !p.at(T!['{']) && is_semi(p, 0) {
		p.eat(T![;]);
	} else {
		let body = function_body(p);
		if p.state.in_declare {
			match body {
				Ok(mut body) => {
					let err = p
						.err_builder(
							"function implementations cannot be given in ambient (declare) contexts",
						)
						.primary(body.range(p), "");

					p.error(err);
					body.change_kind(p, ERROR);
				}
				_ => p.missing(),
			}
		} else {
			body.make_required(p, JsParseErrors::expected_function_body);
		}
	}
}

pub(crate) fn ts_parameter_types(p: &mut Parser) {
	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
		}
	}
}

pub(crate) fn ts_return_type(p: &mut Parser) {
	if p.at(T![:]) {
		let return_type = p.start();
		if let Some(ref mut ty) = ts_type_or_type_predicate_ann(p, T![:]) {
			ty.err_if_not_ts(p, "return types can only be used in TypeScript files");
		}
		return_type.complete(p, TS_TYPE_ANNOTATION);
	}
}
