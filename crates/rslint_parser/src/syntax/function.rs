use crate::parser::ParsedSyntax;
use crate::state::{
	ChangeParserState, InAsync, InConstructor, InFunction, InGenerator, NewLabelsScope,
};
use crate::syntax::binding::parse_binding;
use crate::syntax::decl::parse_parameter_list;
use crate::syntax::js_parse_error;
use crate::syntax::stmt::{is_semi, parse_block_impl};
use crate::syntax::typescript::{ts_type_or_type_predicate_ann, ts_type_params};
use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{Marker, Parser, SyntaxFeature};
use rslint_syntax::JsSyntaxKind::*;
use rslint_syntax::{JsSyntaxKind, T};

/// A function declaration, this could be async and or a generator. This takes a marker
/// because you need to first advance over async or start a marker and feed it in.
// test function_decl
// function foo() {}
// function *foo() {}
// async function *foo() {}
// async function foo() {}
// function *foo() {
//   yield foo;
// }
//
// test function_declaration_script
// // SCRIPT
// function test(await) {}
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
//
// test_err function_broken
// function foo())})}{{{  {}
pub(super) fn parse_function_statement(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	parse_function(p, m, FunctionKind::Statement)
}

pub(super) fn parse_function_expression(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	parse_function(p, m, FunctionKind::Expression)
}

// test export_function_clause
// export function test(a, b) {}
// export function* test(a, b) {}
// export async function test(a, b, ) {}
pub(super) fn parse_export_function_clause(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	parse_function(p, m, FunctionKind::Export)
}

// test export_default_function_clause
// export default function test(a, b) {}
pub(super) fn parse_export_default_function_case(p: &mut Parser) -> ParsedSyntax {
	if !(p.at(T![default]) || p.nth_at(1, T![function]) || p.nth_src(1) == "async") {
		return Absent;
	}

	let m = p.start();
	p.bump(T![default]);
	parse_function(p, m, FunctionKind::ExportDefault)
}

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum FunctionKind {
	Statement,
	Expression,
	Export,
	ExportDefault,
}

impl FunctionKind {
	fn is_id_optional(&self) -> bool {
		matches!(self, FunctionKind::Expression | FunctionKind::ExportDefault)
	}
}

impl From<FunctionKind> for JsSyntaxKind {
	fn from(kind: FunctionKind) -> Self {
		match kind {
			FunctionKind::Statement => JS_FUNCTION_STATEMENT,
			FunctionKind::Expression => JS_FUNCTION_EXPRESSION,
			FunctionKind::Export => JS_EXPORT_FUNCTION_CLAUSE,
			FunctionKind::ExportDefault => JS_EXPORT_DEFAULT_FUNCTION_CLAUSE,
		}
	}
}

fn parse_function(p: &mut Parser, m: Marker, kind: FunctionKind) -> ParsedSyntax {
	let uses_invalid_syntax =
		kind == FunctionKind::Statement && p.eat(T![declare]) && TypeScript.is_unsupported(p);

	let in_async = is_at_async_function(p, LineBreak::DoNotCheck);
	if in_async {
		p.bump_remap(T![async]);
	}

	p.expect(T![function]);

	let in_generator = p.eat(T![*]);

	p.with_state(
		InFunction(true)
			.and(InGenerator(in_generator))
			.and(InAsync(in_async))
			.and(NewLabelsScope),
		|p| {
			let id = parse_binding(p);

			if !kind.is_id_optional() {
				id.or_add_diagnostic(p, |p, range| {
					p.err_builder(
					"expected a name for the function in a function declaration, but found none",
				)
				.primary(range, "")
				});
			}

			TypeScript
				.parse_exclusive_syntax(p, parse_ts_parameter_types, |p, marker| {
					p.err_builder("type parameters can only be used in TypeScript files")
						.primary(marker.range(p), "")
				})
				.ok();

			parse_parameter_list(p).or_add_diagnostic(p, js_parse_error::expected_parameters);

			TypeScript
				.parse_exclusive_syntax(p, parse_ts_type_annotation_or_error, |p, marker| {
					p.err_builder("return types can only be used in TypeScript files")
						.primary(marker.range(p), "")
				})
				.ok();

			if kind == FunctionKind::Statement {
				function_body_or_declaration(p);
			} else {
				function_body(p).or_add_diagnostic(p, js_parse_error::expected_function_body);
			}
		},
	);

	let mut function = m.complete(p, kind.into());

	if uses_invalid_syntax {
		function.change_to_unknown(p);
	}

	Present(function)
}

pub(super) fn function_body(p: &mut Parser) -> ParsedSyntax {
	p.with_state(InFunction(true).and(InConstructor(false)), |p| {
		parse_block_impl(p, JS_FUNCTION_BODY)
	})
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
		body.or_add_diagnostic(p, js_parse_error::expected_function_body);
	}
}

pub(crate) fn parse_ts_parameter_types(p: &mut Parser) -> ParsedSyntax {
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

pub(crate) fn parse_ts_type_annotation_or_error(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![:]) {
		let return_type = p.start();
		if let Some(ref mut ty) = ts_type_or_type_predicate_ann(p, T![:]) {
			ty.err_if_not_ts(p, "return types can only be used in TypeScript files");
		}
		Present(return_type.complete(p, TS_TYPE_ANNOTATION))
	} else {
		Absent
	}
}

/// Tells [is_at_async_function] if it needs to check line breaks
#[derive(PartialEq)]
#[repr(u8)]
pub(super) enum LineBreak {
	// check line breaks
	DoCheck,
	// do not check line break
	DoNotCheck,
}

#[inline]
/// Checks if the parser is inside a "async function"
pub(super) fn is_at_async_function(p: &mut Parser, should_check_line_break: LineBreak) -> bool {
	let async_function_tokens = p.cur_src() == "async" && p.nth_at(1, T![function]);
	if should_check_line_break == LineBreak::DoCheck {
		async_function_tokens && !p.has_linebreak_before_n(1)
	} else {
		async_function_tokens
	}
}
