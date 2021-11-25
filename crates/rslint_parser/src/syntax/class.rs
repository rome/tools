use crate::parser::single_token_parse_recovery::SingleTokenParseRecovery;
use crate::parser::ParsedSyntax;
use crate::syntax::decl::{formal_param_pat, parameter_list, parameters_list};
use crate::syntax::expr::assign_expr;
use crate::syntax::function::{function_body, ts_parameter_types, ts_return_type};
use crate::syntax::object::{computed_member_name, literal_member_name};
use crate::syntax::pat::opt_binding_identifier;
use crate::syntax::stmt::{block_impl, is_semi, optional_semi};
use crate::syntax::typescript::{
	abstract_readonly_modifiers, maybe_ts_type_annotation, try_parse_index_signature,
	ts_heritage_clause, ts_modifier, ts_type_params, DISALLOWED_TYPE_NAMES,
};
use crate::syntax::JsParseErrors;
use crate::ParsedSyntax::Present;
use crate::{CompletedMarker, Event, Marker, Parser, ParserState, StrictMode, TokenSet};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

/// Parses a class expression, e.g. let a = class {}
pub(super) fn class_expression(p: &mut Parser) -> CompletedMarker {
	class(p, ClassKind::Expression)
}

// test class_decl
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}

// test_err class_decl_err
// class {}
// class extends bar {}
// class extends {}
// class
// class foo { set {} }
// class A extends bar extends foo {}
// class A extends bar, foo {}
/// Parses a class declaration
pub(super) fn class_declaration(p: &mut Parser) -> CompletedMarker {
	class(p, ClassKind::Declaration)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
	Declaration,
	Expression,
}

impl From<ClassKind> for SyntaxKind {
	fn from(kind: ClassKind) -> Self {
		match kind {
			ClassKind::Declaration => SyntaxKind::JS_CLASS_DECLARATION,
			ClassKind::Expression => SyntaxKind::JS_CLASS_EXPRESSION,
		}
	}
}

fn class(p: &mut Parser, kind: ClassKind) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T![class]);

	// class bodies are implicitly strict
	let mut guard = p.with_state(ParserState {
		strict: Some(StrictMode::Class(p.cur_tok().range)),
		..p.state.clone()
	});

	// parse class id
	let id = if guard.cur_src() != "implements" {
		opt_binding_identifier(&mut *guard)
	} else {
		None
	};

	if let Some(mut id) = id {
		id.change_kind(&mut *guard, JS_IDENTIFIER_BINDING);

		let text = guard.span_text(id.range(&*guard));
		if guard.typescript() && DISALLOWED_TYPE_NAMES.contains(&text) {
			let err = guard
				.err_builder(&format!(
					"`{}` cannot be used as a class name because it is already reserved as a type",
					text
				))
				.primary(id.range(&*guard), "");

			guard.error(err);
		}
	} else if kind == ClassKind::Declaration && !guard.state.in_default {
		let err = guard
			.err_builder("class declarations must have a name")
			.primary(guard.cur_tok().range, "");

		guard.error(err);
	}

	if guard.at(T![<]) {
		if let Some(mut complete) = ts_type_params(&mut *guard) {
			complete.err_if_not_ts(
				&mut *guard,
				"classes can only have type parameters in TypeScript files",
			);
		}
	}

	extends_clause(&mut guard);
	implements_clause(&mut guard);

	guard.expect_required(T!['{']);
	class_members(&mut *guard);
	guard.expect_required(T!['}']);

	m.complete(&mut *guard, kind.into())
}

fn implements_clause(p: &mut Parser) {
	if p.cur_src() != "implements" {
		return;
	}

	let implements_clause = p.start();

	let start = p.cur_tok().range.start;
	let maybe_err = p.start();
	p.bump_remap(T![implements]);

	let list = p.start();
	let elems = ts_heritage_clause(&mut *p, false);
	if !p.typescript() {
		let err = p
			.err_builder("classes can only implement interfaces in TypeScript files")
			.primary(start..(p.marker_vec_range(&elems).end), "");

		p.error(err);
		maybe_err.complete(&mut *p, ERROR);
	} else {
		maybe_err.abandon(&mut *p)
	}

	while p.cur_src() == "implements" {
		let start = p.cur_tok().range.start;
		let m = p.start();
		p.bump_any();
		let elems = ts_heritage_clause(&mut *p, false);

		let err = p
			.err_builder("classes cannot have multiple `implements` clauses")
			.primary(start..p.marker_vec_range(&elems).end, "");

		p.error(err);
		m.complete(&mut *p, ERROR);
	}

	list.complete(p, LIST);
	implements_clause.complete(p, TS_IMPLEMENTS_CLAUSE);
}

fn extends_clause(p: &mut Parser) {
	if p.cur_src() != "extends" {
		return;
	}

	let m = p.start();
	p.bump_any();

	let mut elems = ts_heritage_clause(p, true);
	if !elems.is_empty() {
		// Unwrap expression
		elems.remove(0).undo_completion(p).abandon(p)
	}

	for elem in elems {
		let err = p
			.err_builder("classes cannot extend multiple classes")
			.primary(elem.range(p), "");

		p.error(err);
	}

	// handle `extends foo extends bar` explicitly
	while p.at(T![extends]) {
		let m = p.start();
		p.bump_any();

		let elems = ts_heritage_clause(p, true);
		let err = p
			.err_builder("classes cannot extend multiple classes")
			.primary(p.marker_vec_range(&elems), "");

		p.error(err);
		m.complete(p, ERROR);
	}

	m.complete(p, JS_EXTENDS_CLAUSE);
}

fn class_members(p: &mut Parser) -> CompletedMarker {
	let members = p.start();

	while !p.at(EOF) && !p.at(T!['}']) {
		class_member(p);
	}

	members.complete(p, LIST)
}

#[allow(deprecated)]
fn class_member(p: &mut Parser) -> CompletedMarker {
	let mut member_marker = p.start();

	// test class_empty_element
	// class foo { ;;;;;;;;;; get foo() {};;;;}
	if p.eat(T![;]) {
		return member_marker.complete(p, JS_EMPTY_CLASS_MEMBER);
	}

	// test static_method
	// class foo {
	//  static foo(bar) {}
	//  static *foo() {}
	//  static async foo() {}
	//  static async *foo() {}
	// }
	let has_access_modifier = matches!(p.cur_src(), "public" | "private" | "protected");
	let mut offset = if has_access_modifier { 1 } else { 0 };

	let declare = p.nth_src(offset) == "declare";
	if declare {
		offset += 1;
	}

	// Let's assume declare is an identifier and not a keyword
	if declare && !has_access_modifier {
		// declare() and declare: foo
		if is_method_class_member(p, offset) {
			literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return method_class_member_body(p, member_marker);
		} else if is_property_class_member(p, offset) {
			literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return property_class_member_body(p, member_marker);
		} else {
			let msg = if p.typescript() {
				"a `declare` modifier cannot be applied to a class element"
			} else {
				"`declare` modifiers can only be used in TypeScript files"
			};
			let err = p.err_builder(msg).primary(p.cur_tok().range, "");

			p.error(err);
			let m = p.start();
			p.bump_any();
			m.complete(p, ERROR);
		}
	};

	if has_access_modifier {
		if is_method_class_member(p, offset) {
			if declare {
				let msg = if p.typescript() {
					"a `declare` modifier cannot be applied to a class method"
				} else {
					"`declare` modifiers can only be used in TypeScript files"
				};

				let err = p.err_builder(msg).primary(p.cur_tok().range, "");

				p.error(err);

				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
			}

			return method_class_member(p, member_marker);
		} else if is_property_class_member(p, offset) {
			if declare {
				p.bump_remap(T![declare]);
			}
			p.bump_any();

			return property_class_member_body(p, member_marker);
		}
	}

	let is_static = p.nth_src(offset) == "static";

	// Let's assume static is an identifier and not the static keyword
	if is_static {
		offset += 1;

		if is_method_class_member(p, offset) {
			consume_modifiers(p, declare, has_access_modifier, is_static, true);
			return method_class_member_body(p, member_marker);
		} else if is_property_class_member(p, offset) {
			consume_modifiers(p, declare, has_access_modifier, is_static, true);

			return if declare {
				property_declaration_class_member_body(p, member_marker, JS_LITERAL_MEMBER_NAME)
			} else {
				property_class_member_body(p, member_marker)
			};
		}

		// Seems that static is a keyword since the parser wasn't able to parse a valid method or property named static
	}

	consume_modifiers(p, declare, has_access_modifier, is_static, false);

	let maybe_err = p.start();
	let (abstract_range, readonly_range) = abstract_readonly_modifiers(p);
	let has_modifier = abstract_range
		.clone()
		.or_else(|| readonly_range.clone())
		.is_some();

	if has_modifier {
		let range = abstract_range
			.clone()
			.or_else(|| readonly_range.clone())
			.unwrap();
		if !p.typescript() {
			let err = p
				.err_builder(
					"`abstract` and `readonly` modifiers can only be used in TypeScript files",
				)
				.primary(range, "");

			p.error(err);
			maybe_err.complete(p, ERROR);
		} else {
			maybe_err.abandon(p);
		}
	} else {
		maybe_err.abandon(p);
	}

	if !is_static && !has_access_modifier {
		let check = p.checkpoint();
		if let Some(range) = abstract_range.clone() {
			let err = p
				.err_builder("the `abstract` modifier cannot be used on index signatures")
				.primary(range, "");

			p.error(err);
		}
		member_marker = match try_parse_index_signature(p, member_marker) {
			Ok(mut sig) => {
				sig.err_if_not_ts(
					p,
					"class index signatures can only be used in TypeScript files",
				);
				return sig;
			}
			Err(m) => {
				p.rewind(check);
				m
			}
		};
	};

	let generator_range = p.cur_tok().range;

	if p.eat(T![*]) {
		let is_constructor = p.cur_src() == "constructor";
		let mut guard = p.with_state(ParserState {
			in_generator: true,
			in_function: true,
			..p.state.clone()
		});

		if let Some(range) = readonly_range {
			let err = guard
				.err_builder("class methods cannot be readonly")
				.primary(range, "");

			guard.error(err);
		}

		if is_constructor {
			let err = guard
				.err_builder("constructors can't be generators")
				.primary(generator_range, "");

			guard.error(err);
		}

		return method_class_member(&mut *guard, member_marker);
	};

	if p.cur_src() == "async"
		&& !p.nth_at(1, T![?])
		&& !is_method_class_member(p, 1)
		&& !p.has_linebreak_before_n(1)
	{
		let async_range = p.cur_tok().range;
		p.bump_remap(T![async]);
		let in_generator = p.eat(T![*]);

		let mut guard = p.with_state(ParserState {
			in_async: true,
			in_generator,
			in_function: true,
			..p.state.clone()
		});

		if guard.cur_src() == "constructor" {
			let err = guard
				.err_builder("constructors cannot be async")
				.primary(async_range, "");

			guard.error(err);
		}

		if let Some(range) = readonly_range {
			let err = guard
				.err_builder("constructors cannot be readonly")
				.primary(range, "");

			guard.error(err);
		}

		return method_class_member(&mut *guard, member_marker);
	}

	let member_name = p.cur_src();
	let is_constructor = matches!(
		member_name,
		"constructor" | "\"constructor\"" | "'constructor'"
	);
	let member = class_member_name(p).make_required(p, JsParseErrors::expected_class_member_name);

	if is_method_class_member(p, 0) {
		if let Some(range) = readonly_range.clone() {
			let err = p
				.err_builder("class methods cannot be readonly")
				.primary(range, "");

			p.error(err);
		}

		return if is_constructor {
			let constructor = constructor_class_member_body(p, member_marker);

			if is_static {
				let err = p
					.err_builder("constructors cannot be static")
					.primary(constructor.range(p), "");

				p.error(err);
			}

			if has_modifier {
				let err = p.err_builder("constructors cannot have modifiers").primary(
					abstract_range.or_else(|| readonly_range.clone()).unwrap(),
					"",
				);

				p.error(err);
			}

			constructor
		} else {
			method_class_member_body(p, member_marker)
		};
	}

	if let Some(member) = member {
		if is_property_class_member(p, 0) {
			let property = if declare {
				property_declaration_class_member_body(p, member_marker, member.kind())
			} else {
				property_class_member_body(p, member_marker)
			};

			if is_constructor {
				let err = p
					.err_builder("class properties may not be called `constructor`")
					.primary(property.range(p), "");

				p.error(err);
			}

			return property;
		}

		if member.kind() == JS_LITERAL_MEMBER_NAME {
			let is_at_line_break_or_generator = p.has_linebreak_before_n(0) && p.at(T![*]);
			let member_name = member.text(p);
			if matches!(member_name, "get" | "set") && !is_at_line_break_or_generator {
				let is_getter = member_name == "get";

				// test getter_class_member
				// class Getters {
				// 	get foo() {}
				// 	get static() {}
				// 	static get bar() {}
				// 	get "baz"() {}
				// 	get ["a" + "b"]() {}
				// 	get 5() {}
				// 	get #private() {}
				// }
				// class NotGetters {
				// 	get() {}
				// 	async get() {}
				// 	static get() {}
				// }

				// test setter_class_number
				// class Setters {
				// 	set foo(a) {}
				// 	set static(a) {}
				// 	static set bar(a) {}
				// 	set "baz"(a) {}
				// 	set ["a" + "b"](a) {}
				// 	set 5(a) {}
				// 	set #private(a) {}
				// }
				// class NotSetters {
				// 	set(a) {}
				// 	async set(a) {}
				// 	static set(a) {}
				// }

				// The tree currently holds a STATIC_MEMBER_NAME node that wraps a ident token but we now found
				// out that the 'get' or 'set' isn't a member name in this context but instead are the
				// 'get'/'set' keywords for getters/setters. That's why we need to undo the member name node,
				// extract the 'get'/'set' ident token and change its kind to 'get'/'set'
				match p.events[(member.start_pos as usize) + 1] {
					Event::Token { ref mut kind, .. } => {
						*kind = if is_getter { T![get] } else { T![set] }
					}
					_ => unreachable!(),
				};
				member.undo_completion(p).abandon(p);

				if let Some(range) = readonly_range {
					let err = p
						.err_builder("getters and setters cannot be readonly")
						.primary(range, "");

					p.error(err);
				}

				// So we've seen a get that now must be followed by a getter/setter name
				class_member_name(p).make_required(p, JsParseErrors::expected_class_member_name);
				p.expect_required(T!['(']);

				let completed = if is_getter {
					p.expect_required(T![')']);
					ts_return_type(p);
					function_body(p).make_required(p, JsParseErrors::expected_function_body);

					member_marker.complete(p, JS_GETTER_CLASS_MEMBER)
				} else {
					formal_param_pat(p);
					p.expect_required(T![')']);
					function_body(p).make_required(p, JsParseErrors::expected_function_body);

					member_marker.complete(p, JS_SETTER_CLASS_MEMBER)
				};

				return completed;
			}
		}
	}

	let err = p
		.err_builder("expected `;`, a property, or a method for a class body, but found none")
		.primary(p.cur_tok().range, "");
	SingleTokenParseRecovery::with_error(
		token_set![T![;], T![ident], T![async], T![yield], T!['}'], T![#]],
		JS_UNKNOWN_MEMBER,
		err,
	)
	.recover(p);

	member_marker.complete(p, JS_UNKNOWN_MEMBER)
}

const PROPERTY_START_SET: TokenSet = token_set![T![!], T![:], T![=], T!['}']];

/// Tests if the parser is currently (considering the offset) at the body of a property member.
/// The method assumes that the identifier has already been consumed.
fn is_property_class_member(p: &Parser, mut offset: usize) -> bool {
	if p.nth_at(offset, T![?]) {
		offset += 1;
	}

	PROPERTY_START_SET.contains(p.nth(offset)) || is_semi(p, offset)
}

fn property_declaration_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
	member_name_kind: SyntaxKind,
) -> CompletedMarker {
	let property = property_class_member_body(p, member_marker);

	if member_name_kind == JS_PRIVATE_CLASS_MEMBER_NAME {
		let err = p
			.err_builder("private class properties with `declare` are invalid")
			.primary(property.range(p), "");

		p.error(err);
	}

	property
}

/// Parses the body of a property class member (anything after the member name)
fn property_class_member_body(p: &mut Parser, member_marker: Marker) -> CompletedMarker {
	let optional_range = optional_member_token(p);
	if p.at(T![!]) {
		let range = p.cur_tok().range;

		let error = if !p.typescript() {
			Some(
				p.err_builder(
					"definite assignment assertions can only be used in TypeScript files",
				)
				.primary(range, ""),
			)
		} else {
			optional_range.map(|optional| {
				p.err_builder("class properties cannot be both optional and definite")
					.primary(range.clone(), "")
					.secondary(optional, "")
			})
		};

		if let Some(error) = error {
			p.error(error);
			p.bump_remap(ERROR);
		} else {
			p.bump_any(); // Bump ! token
		}
	}

	maybe_ts_type_annotation(p);
	optional_equals_value_clause(p);

	if !optional_semi(p) {
		// Gets the start of the member
		let start = match p.events[member_marker.old_start as usize] {
			Event::Start { start, .. } => start,
			_ => unreachable!(),
		};

		let err = p
			.err_builder("expected a semicolon for a class property, but found none")
			.primary(start..p.cur_tok().range.start, "");

		p.error(err);
	}

	let complete = member_marker.complete(p, JS_PROPERTY_CLASS_MEMBER);

	if !p.syntax.class_fields {
		let err = p
			.err_builder("class fields are unsupported")
			.primary(complete.range(p), "");

		p.error(err);
	}

	complete
}

/// Eats the ? token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut Parser) -> Option<Range<usize>> {
	if p.at(T![?]) {
		let range = p.cur_tok().range;
		if !p.typescript() {
			let err = p
				.err_builder("`?` modifiers can only be used in TypeScript files")
				.primary(p.cur_tok().range, "");

			p.error(err);
			p.bump_remap(ERROR);
		} else {
			p.bump_any();
		}
		Some(range)
	} else {
		None
	}
}

fn optional_equals_value_clause(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![=]) {
		let m = p.start();
		p.bump_any(); // eat the = token

		assign_expr(p);

		Some(m.complete(p, JS_EQUAL_VALUE_CLAUSE))
	} else {
		None
	}
}

fn is_method_class_member(p: &Parser, mut offset: usize) -> bool {
	if p.nth_at(offset, T![?]) {
		offset += 1;
	}

	p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<])
}

fn method_class_member(p: &mut Parser, m: Marker) -> CompletedMarker {
	class_member_name(p).make_required(p, JsParseErrors::expected_function_body);
	method_class_member_body(p, m)
}

/// Parses the body (everything after the identifier name) of a method class member
fn method_class_member_body(p: &mut Parser, m: Marker) -> CompletedMarker {
	optional_member_token(p);
	ts_parameter_types(p);
	parameter_list(p);
	ts_return_type(p);
	function_body(p).make_required(p, JsParseErrors::expected_function_body);

	m.complete(p, JS_METHOD_CLASS_MEMBER)
}

fn constructor_class_member_body(p: &mut Parser, member_marker: Marker) -> CompletedMarker {
	if let Some(range) = optional_member_token(p) {
		let err = p
			.err_builder("constructors cannot be optional")
			.primary(range, "");

		p.error(err);
	}

	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");

			let err = p
				.err_builder("constructors cannot have type parameters")
				.primary(ty.range(p), "");

			p.error(err);
			ty.change_kind(p, ERROR);
		}
	}

	constructor_parameter_list(p);

	if let Some(range) = maybe_ts_type_annotation(p) {
		let err = p
			.err_builder("constructors cannot have type annotations")
			.primary(range, "");

		p.error(err);
	}

	{
		let mut guard = p.with_state(ParserState {
			in_function: true,
			in_constructor: true,
			..p.state.clone()
		});

		let p = &mut *guard;

		block_impl(p, JS_FUNCTION_BODY).make_required(p, JsParseErrors::expected_function_body);
	}

	// FIXME(RDambrosio016): if there is no body we need to issue errors for any assign patterns

	// TODO(RDambrosio016): ideally the following errors should just point to the modifiers
	member_marker.complete(p, JS_CONSTRUCTOR_CLASS_MEMBER)
}

fn constructor_parameter_list(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	parameters_list(p, constructor_parameter);
	m.complete(p, JS_CONSTRUCTOR_PARAMETER_LIST)
}

fn constructor_parameter(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let has_accessibility = if ts_access_modifier(p).is_some() {
		let range = p.cur_tok().range;
		let maybe_err = p.start();
		consume_modifiers(p, false, true, false, false);
		if !p.typescript() {
			let err = p
				.err_builder("accessibility modifiers can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			maybe_err.complete(p, ERROR);
		} else {
			maybe_err.abandon(p);
		}
		true
	} else {
		false
	};

	let maybe_err = p.start();
	let has_readonly = if let Some(range) = ts_modifier(p, &["readonly"]) {
		if !p.typescript() {
			let err = p
				.err_builder("readonly modifiers can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			maybe_err.complete(p, ERROR);
		} else {
			maybe_err.abandon(p);
		}
		true
	} else {
		maybe_err.abandon(p);
		false
	};

	if !has_accessibility && !has_readonly {
		m.abandon(p);
		formal_param_pat(p)
	} else {
		if let Some(ref mut pat) = formal_param_pat(p) {
			pat.undo_completion(p).abandon(p);
		}
		Some(m.complete(p, TS_CONSTRUCTOR_PARAM))
	}
}

fn ts_access_modifier<'a>(p: &'a Parser) -> Option<&'a str> {
	if matches!(p.cur_src(), "public" | "private" | "protected") {
		Some(p.cur_src())
	} else {
		None
	}
}

/// Parses a `JsAnyClassMemberName` and returns its completion marker
fn class_member_name(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T![#] => Present(private_class_member_name(p)),
		T!['['] => computed_member_name(p),
		_ => literal_member_name(p),
	}
}

pub(crate) fn private_class_member_name(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect_required(T![#]);
	p.expect_required(T![ident]);
	m.complete(p, JS_PRIVATE_CLASS_MEMBER_NAME)
}

fn consume_modifiers(
	p: &mut Parser,
	declare: bool,
	accessibility: bool,
	static_: bool,
	dont_remap_static: bool,
) {
	if accessibility {
		let kind = match p.cur_src() {
			"public" => PUBLIC_KW,
			"private" => PRIVATE_KW,
			"protected" => PROTECTED_KW,
			_ => unreachable!(),
		};
		if !p.typescript() {
			let m = p.start();
			let range = p.cur_tok().range;
			let err = p
				.err_builder("accessibility modifiers can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			p.bump_any();
			m.complete(p, ERROR);
		} else {
			p.bump_remap(kind);
		}
	}
	if declare {
		p.bump_remap(T![declare]);
	}
	if static_ && !dont_remap_static {
		p.bump_remap(STATIC_KW);
	} else if static_ && dont_remap_static {
		p.bump_any();
	}
}
