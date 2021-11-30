use crate::parser::ConditionalParsedSyntax::Valid;
use crate::parser::ParsedSyntax;
use crate::syntax::decl::parse_parameter_list;
use crate::syntax::js_parse_error;
use crate::syntax::pat::parse_identifier_binding;
use crate::syntax::stmt::{is_semi, parse_block_impl};
use crate::syntax::typescript::{ts_type_or_type_predicate_ann, ts_type_params};
use crate::ConditionalParsedSyntax::Invalid;
use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{CompletedMarker, Parser, ParserState};
use crate::{ConditionalParsedSyntax, SyntaxFeature};
use rslint_syntax::SyntaxKind::{
	ERROR, JS_FUNCTION_BODY, JS_FUNCTION_DECLARATION, JS_FUNCTION_EXPRESSION,
	JS_UNKNOWN_EXPRESSION, JS_UNKNOWN_STATEMENT, TS_TYPE_ANNOTATION,
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
//
// test_err function_decl_err
// function() {}
// function foo {}
// function {}
// function *() {}
// async function() {}
// async function *() {}
// function *foo() {}
// yield foo;
// function test(): number {}
// function foo(await) {}
// function foo(yield) {}
pub(super) fn parse_function_declaration(p: &mut Parser) -> ParsedSyntax {
	function(p, JS_FUNCTION_DECLARATION).or_invalid_to_unknown(p, JS_UNKNOWN_STATEMENT)
}

pub(super) fn function_expression(p: &mut Parser) -> CompletedMarker {
	function(p, JS_FUNCTION_EXPRESSION)
		.or_invalid_to_unknown(p, JS_UNKNOWN_EXPRESSION)
		.ok()
		.unwrap()
}

fn function(p: &mut Parser, kind: SyntaxKind) -> ConditionalParsedSyntax {
	let m = p.start();

	let mut uses_ts_syntax = kind == JS_FUNCTION_DECLARATION && p.eat(T![declare]);

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

	let id = parse_identifier_binding(guard);

	if kind == JS_FUNCTION_DECLARATION {
		id.or_missing_with_error(guard, |p, range| {
			p.err_builder(
				"expected a name for the function in a function declaration, but found none",
			)
			.primary(range, "")
		});
	} else {
		id.or_missing(guard);
	}

	let type_parameters =
		parse_ts_parameter_types(guard).exclusive_for(&TypeScript, guard, |p, marker| {
			p.err_builder("type parameters can only be used in TypeScript files")
				.primary(marker.range(p), "")
		});

	uses_ts_syntax |= type_parameters.is_present();

	if let Valid(type_parameters) = type_parameters {
		type_parameters.or_missing(guard);
	}

	parse_parameter_list(guard).or_missing_with_error(guard, js_parse_error::expected_parameters);

	let return_type = parse_ts_return_type(guard).exclusive_for(&TypeScript, guard, |p, marker| {
		p.err_builder("return types can only be used in TypeScript files")
			.primary(marker.range(p), "")
	});

	uses_ts_syntax |= return_type.is_present();

	if let Valid(return_type) = return_type {
		return_type.or_missing(guard);
	}

	if kind == JS_FUNCTION_DECLARATION {
		function_body_or_declaration(guard);
	} else {
		function_body(guard).or_missing_with_error(guard, js_parse_error::expected_function_body);
	}

	let function = m.complete(guard, kind);

	if uses_ts_syntax && TypeScript.is_unsupported(guard) {
		Invalid(function.into())
	} else {
		Valid(function.into())
	}
}

pub(super) fn function_body(p: &mut Parser) -> ParsedSyntax {
	let mut guard = p.with_state(ParserState {
		in_constructor: false,
		in_function: true,
		..p.state.clone()
	});

	parse_block_impl(&mut *guard, JS_FUNCTION_BODY)
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
				Present(mut body) => {
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
			body.or_missing_with_error(p, js_parse_error::expected_function_body);
		}
	}
}

fn parse_ts_parameter_types(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![<]) {
		Present(ts_type_params(p).unwrap())
	} else {
		Absent
	}
}

pub(crate) fn ts_parameter_types(p: &mut Parser) {
	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
		}
	}
}

fn parse_ts_return_type(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![:]) {
		let return_type = p.start();
		ts_type_or_type_predicate_ann(p, T![:]);
		Present(return_type.complete(p, TS_TYPE_ANNOTATION))
	} else {
		Absent
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
