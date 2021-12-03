use crate::parser::{ParsedSyntax, ParserProgress};
use crate::syntax::binding::parse_identifier_binding;
use crate::syntax::decl::{parse_formal_param_pat, parse_parameter_list, parse_parameters_list};
use crate::syntax::expr::expr_or_assignment;
use crate::syntax::function::{function_body, ts_parameter_types, ts_return_type};
use crate::syntax::js_parse_error;
use crate::syntax::object::{parse_computed_member_name, parse_literal_member_name};
use crate::syntax::stmt::{is_semi, optional_semi, parse_block_impl};
use crate::syntax::typescript::{
	abstract_readonly_modifiers, maybe_ts_type_annotation, try_parse_index_signature,
	ts_heritage_clause, ts_modifier, ts_type_params, DISALLOWED_TYPE_NAMES,
};
use crate::ConditionalSyntax::{Invalid, Valid};
use crate::ParsedSyntax::{Absent, Present};
use crate::{
	CompletedMarker, ConditionalSyntax, Event, Marker, ParseRecovery, Parser, ParserState,
	StrictMode, TokenSet,
};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

/// Parses a class expression, e.g. let a = class {}
pub(super) fn parse_class_expression(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	parse_class(p, ClassKind::Expression).or_invalid_to_unknown(p, JS_UNKNOWN_EXPRESSION)
}

// test class_decl
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}

// test_err class_decl_err
// class {}
// class extends bar {}
// class foo { set {} }
// class extends {}

// test_err class_extends_err
// class A extends bar extends foo {}
// class A extends bar, foo {}
/// Parses a class declaration if it is valid and otherwise returns [Invalid].
///
/// A class can be invalid if
/// * It uses an illegal identifier name
pub(super) fn parse_class_declaration(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	parse_class(p, ClassKind::Declaration)
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

fn parse_class(p: &mut Parser, kind: ClassKind) -> ParsedSyntax<ConditionalSyntax> {
	if !p.at(T![class]) {
		return Absent;
	}
	let m = p.start();
	let class_token = p.cur_tok().range;
	p.expect_required(T![class]);

	// class bodies are implicitly strict
	let mut guard = p.with_state(ParserState {
		strict: Some(StrictMode::Class(p.cur_tok().range)),
		..p.state.clone()
	});

	let mut uses_invalid_syntax = false;

	// parse class id
	if guard.cur_src() != "implements" {
		match parse_identifier_binding(&mut *guard) {
			Present(Valid(id)) => {
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
			}
			Present(Invalid(_)) => uses_invalid_syntax = true,
			Absent => {
				if kind == ClassKind::Declaration && !guard.state.in_default {
					let err = guard
						.err_builder("class declarations must have a name")
						.primary(class_token.start..guard.cur_tok().range.start, "");

					guard.missing();
					guard.error(err);
				}
			}
		}
	}

	if guard.at(T![<]) {
		if let Some(mut complete) = ts_type_params(&mut *guard) {
			complete.err_if_not_ts(
				&mut *guard,
				"classes can only have type parameters in TypeScript files",
			);
		}
	}

	// TODO: these two functions should return `ParsedSyntax`, so we can handle possible errors/missing/etc.
	extends_clause(&mut guard);
	implements_clause(&mut guard);

	guard.expect_required(T!['{']);
	parse_class_members(&mut *guard);
	guard.expect_required(T!['}']);

	let result = Present(m.complete(&mut *guard, kind.into()));
	if uses_invalid_syntax {
		result.into_invalid()
	} else {
		result.into_valid()
	}
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

	let mut progress = ParserProgress::default();
	while p.cur_src() == "implements" {
		progress.assert_progressing(p);
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
	let mut progress = ParserProgress::default();
	while p.at(T![extends]) {
		progress.assert_progressing(p);
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

/// Parses a list of class members
///
/// # Panics
///
/// The method can panic if [parse_class_member] returns [ParsedSyntax::Absent]
fn parse_class_members(p: &mut Parser) {
	let members = p.start();

	let mut progress = ParserProgress::default();
	while !p.at(EOF) && !p.at(T!['}']) {
		progress.assert_progressing(p);

		let parsed_member = parse_class_member(p);

		// In this case we want to mark members that have errors as invalid.
		//
		// This is because the parsing of members is incremental and when we encounter
		// an error down the line, we might have already progressed the parser.
		//
		// So when there's an error, we mark the whole marker as present (completed) and invalid
		// and here we abandon the invalid marker and later we try to recover.
		//
		//  Not that if we get a [ParsedSyntax::Absent], we want to panic
		let to_recover = match parsed_member {
			Present(Valid(marker)) => Present(marker),
			Present(Invalid(conditional_syntax)) => {
				conditional_syntax.abandon(p);
				Absent
			}
			Absent => panic!("A class member can't be `Absent`"),
		};

		let member_recovered = to_recover.or_recover(
			p,
			&ParseRecovery::new(
				JS_UNKNOWN_MEMBER,
				token_set![T![;], T![ident], T![async], T![yield], T!['}'], T![#]],
			),
			js_parse_error::expected_class_member,
		);

		if member_recovered.is_err() {
			break;
		}
	}

	members.complete(p, LIST);
}

fn parse_class_member(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	let mut member_marker = p.start();

	// test class_empty_element
	// class foo { ;;;;;;;;;; get foo() {};;;;}
	if p.eat(T![;]) {
		return Present(Valid(member_marker.complete(p, JS_EMPTY_CLASS_MEMBER)));
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
		if is_at_method_class_member(p, offset) {
			parse_literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return Present(Valid(parse_method_class_member_body(p, member_marker)));
		} else if is_at_property_class_member(p, offset) {
			parse_literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return Present(Valid(parse_property_class_member_body(p, member_marker)));
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
		if is_at_method_class_member(p, offset) {
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

			return Present(Valid(parse_method_class_member(p, member_marker)));
		} else if is_at_property_class_member(p, offset) {
			if declare {
				p.bump_remap(T![declare]);
			}
			p.bump_any();

			return Present(Valid(parse_property_class_member_body(p, member_marker)));
		}
	}

	let is_static = p.nth_src(offset) == "static";

	// Let's assume static is an identifier and not the static keyword
	if is_static {
		offset += 1;

		if is_at_method_class_member(p, offset) {
			consume_modifiers(p, declare, has_access_modifier, is_static, true);
			return Present(Valid(parse_method_class_member_body(p, member_marker)));
		} else if is_at_property_class_member(p, offset) {
			consume_modifiers(p, declare, has_access_modifier, is_static, true);

			return if declare {
				Present(Valid(property_declaration_class_member_body(
					p,
					member_marker,
					JS_LITERAL_MEMBER_NAME,
				)))
			} else {
				Present(Valid(parse_property_class_member_body(p, member_marker)))
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
				return Present(Valid(sig));
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

		return Present(Valid(parse_method_class_member(&mut *guard, member_marker)));
	};

	if p.cur_src() == "async"
		&& !p.nth_at(1, T![?])
		&& !is_at_method_class_member(p, 1)
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

		return Present(Valid(parse_method_class_member(&mut *guard, member_marker)));
	}

	let member_name = p.cur_src();
	let is_constructor = matches!(
		member_name,
		"constructor" | "\"constructor\"" | "'constructor'"
	);
	let member = parse_class_member_name(p)
		.or_missing_with_error(p, js_parse_error::expected_class_member_name);

	if is_at_method_class_member(p, 0) {
		if let Some(range) = readonly_range.clone() {
			let err = p
				.err_builder("class methods cannot be readonly")
				.primary(range, "");

			p.error(err);
		}

		// test_err class_constructor_err
		// class B { static constructor() {} }
		return if is_constructor {
			let constructor = parse_constructor_class_member_body(p, member_marker);

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

			Present(Valid(constructor))
		} else {
			Present(Valid(parse_method_class_member_body(p, member_marker)))
		};
	}

	if let Some(member) = member {
		if is_at_property_class_member(p, 0) {
			let property = if declare {
				property_declaration_class_member_body(p, member_marker, member.kind())
			} else {
				parse_property_class_member_body(p, member_marker)
			};

			if is_constructor {
				let err = p
					.err_builder("class properties may not be called `constructor`")
					.primary(property.range(p), "");

				p.error(err);
			}

			return Present(Valid(property));
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

				// test_err getter_class_no_body
				// class Setters {
				//   get foo()

				// test setter_class_member
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

				// test_err setter_class_member
				// class Setters {
				//   set foo() {}
				// }

				// test_err setter_class_no_body
				// class Setters {
				//   set foo(a)

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
				parse_class_member_name(p)
					.or_missing_with_error(p, js_parse_error::expected_class_member_name);
				p.expect_required(T!['(']);

				let completed = if is_getter {
					p.expect_required(T![')']);
					ts_return_type(p);
					function_body(p).or_missing_with_error(p, js_parse_error::expected_class_body);

					member_marker.complete(p, JS_GETTER_CLASS_MEMBER)
				} else {
					// TODO: review error handling once the pattern functions is refactored
					parse_formal_param_pat(p)
						.or_missing_with_error(p, js_parse_error::expected_parameter);
					p.expect_required(T![')']);
					function_body(p)
						.or_missing_with_error(p, js_parse_error::expected_function_body);

					member_marker.complete(p, JS_SETTER_CLASS_MEMBER)
				};

				return Present(Valid(completed));
			}
		}
	}
	Present(Invalid(member_marker.complete(p, JS_UNKNOWN_MEMBER).into()))
}

const PROPERTY_START_SET: TokenSet = token_set![T![!], T![:], T![=], T!['}']];

fn property_declaration_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
	member_name_kind: SyntaxKind,
) -> CompletedMarker {
	let property = parse_property_class_member_body(p, member_marker);
	if member_name_kind == JS_PRIVATE_CLASS_MEMBER_NAME {
		let err = p
			.err_builder("private class properties with `declare` are invalid")
			.primary(property.range(p), "");

		p.error(err);
	}

	property
}

/// Parses the body of a property class member (anything after the member name)
fn parse_property_class_member_body(p: &mut Parser, member_marker: Marker) -> CompletedMarker {
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
	parse_equal_value_clause(p).or_missing(p);

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

pub(crate) fn parse_equal_value_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if p.at(T![=]) {
		let m = p.start();
		p.bump(T![=]);

		expr_or_assignment(p);

		Present(m.complete(p, JS_EQUAL_VALUE_CLAUSE))
	} else {
		Absent
	}
}

fn parse_method_class_member(p: &mut Parser, m: Marker) -> CompletedMarker {
	parse_class_member_name(p).or_missing_with_error(p, js_parse_error::expected_class_member_name);
	parse_method_class_member_body(p, m)
}

// test_err class_member_method_parameters
// class B { foo(a {} }

// test_err class_member_method_body
// class B { foo(a)

/// Parses the body (everything after the identifier name) of a method class member
fn parse_method_class_member_body(p: &mut Parser, m: Marker) -> CompletedMarker {
	optional_member_token(p);
	ts_parameter_types(p);
	parse_parameter_list(p).or_missing_with_error(p, js_parse_error::expected_class_parameters);
	ts_return_type(p);
	function_body(p).or_missing_with_error(p, js_parse_error::expected_class_body);

	m.complete(p, JS_METHOD_CLASS_MEMBER)
}

fn parse_constructor_class_member_body(p: &mut Parser, member_marker: Marker) -> CompletedMarker {
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

	parse_constructor_parameter_list(p)
		.or_missing_with_error(p, js_parse_error::expected_constructor_parameters);

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

		parse_block_impl(p, JS_FUNCTION_BODY)
			.or_missing_with_error(p, js_parse_error::expected_function_body);
	}

	// FIXME(RDambrosio016): if there is no body we need to issue errors for any assign patterns

	// TODO(RDambrosio016): ideally the following errors should just point to the modifiers
	member_marker.complete(p, JS_CONSTRUCTOR_CLASS_MEMBER)
}

fn parse_constructor_parameter_list(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	let m = p.start();
	parse_parameters_list(p, parse_constructor_parameter);
	Present(m.complete(p, JS_CONSTRUCTOR_PARAMETER_LIST))
}

fn parse_constructor_parameter(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	let modifiers_marker = p.start();
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
		modifiers_marker.abandon(p);
		parse_formal_param_pat(p)
	} else {
		if let Present(ref mut pat) = parse_formal_param_pat(p) {
			pat.undo_completion(p).abandon(p);
		}
		Present(modifiers_marker.complete(p, TS_CONSTRUCTOR_PARAM))
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
fn parse_class_member_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		T![#] => parse_private_class_member_name(p),
		T!['['] => parse_computed_member_name(p),
		_ => parse_literal_member_name(p),
	}
}

pub(crate) fn parse_private_class_member_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![#]) {
		return Absent;
	}
	let m = p.start();
	p.expect_required(T![#]);
	p.expect_required(T![ident]);
	Present(m.complete(p, JS_PRIVATE_CLASS_MEMBER_NAME))
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
		// Guaranteed to be at the static keyword, parsing a class member must succeed
		parse_class_member_name(p).ok().unwrap();
	}
}

/// Tests if the parser is currently (considering the offset) at the body of a property member.
/// The method assumes that the identifier has already been consumed.
fn is_at_property_class_member(p: &Parser, mut offset: usize) -> bool {
	if p.nth_at(offset, T![?]) {
		offset += 1;
	}

	PROPERTY_START_SET.contains(p.nth(offset)) || is_semi(p, offset)
}

fn is_at_method_class_member(p: &Parser, mut offset: usize) -> bool {
	if p.nth_at(offset, T![?]) {
		offset += 1;
	}

	p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<])
}
