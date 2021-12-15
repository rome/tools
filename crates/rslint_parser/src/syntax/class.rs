use crate::parser::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::syntax::binding::parse_binding;
use crate::syntax::decl::{parse_formal_param_pat, parse_parameter_list, parse_parameters_list};
use crate::syntax::expr::expr_or_assignment;
use crate::syntax::function::{function_body, parse_ts_return_type_if_ts, ts_parameter_types};
use crate::syntax::js_parse_error;
use crate::syntax::object::{parse_computed_member_name, parse_literal_member_name};
use crate::syntax::stmt::{is_semi, optional_semi, parse_block_impl};
use crate::syntax::typescript::{
	abstract_readonly_modifiers, maybe_ts_type_annotation, try_parse_index_signature,
	ts_heritage_clause, ts_modifier, ts_type_params, DISALLOWED_TYPE_NAMES,
};
use crate::CompletedNodeOrMissingMarker::NodeMarker;
use crate::ParsedSyntax::{Absent, Present};
use crate::{
	CompletedMarker, ConditionalSyntax, Event, Invalid, Marker, ParseNodeList, ParseRecovery,
	Parser, ParserState, StrictMode, TokenSet, Valid,
};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

/// Parses a class expression, e.g. let a = class {}
pub(super) fn parse_class_expression(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	parse_class(p, ClassKind::Expression)
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
	let mut class_is_valid = true;
	let m = p.start();
	let class_token_range = p.cur_tok().range;
	p.expect_required(T![class]);

	// class bodies are implicitly strict
	let mut guard = p.with_state(ParserState {
		strict: Some(StrictMode::Class(p.cur_tok().range)),
		..p.state.clone()
	});

	// parse class id
	if guard.cur_src() != "implements" {
		let id = parse_binding(&mut *guard);

		match id {
			Present(id) => {
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
			Absent => {
				if kind == ClassKind::Declaration && !guard.state.in_default {
					let err = guard
						.err_builder("class declarations must have a name")
						.primary(class_token_range.start..guard.cur_tok().range.start, "");

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

	if extends_clause(&mut guard).or_missing(&mut *guard).is_err() {
		class_is_valid = false;
	}

	if implements_clause(&mut guard)
		.or_missing(&mut *guard)
		.is_err()
	{
		class_is_valid = false;
	}

	guard.expect_required(T!['{']);
	ClassMembersList.parse_list(&mut *guard);
	guard.expect_required(T!['}']);

	let class_marker = m.complete(&mut *guard, kind.into());

	Present(class_marker).into_conditional(class_is_valid)
}

fn implements_clause(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	if p.cur_src() != "implements" {
		return Absent;
	}

	let mut is_valid = true;
	let implements_clause = p.start();

	let start = p.cur_tok().range.start;
	p.bump_remap(T![implements]);

	let list = p.start();
	let elems = ts_heritage_clause(&mut *p, false);
	// test_err class_implements
	// class B implements C {}
	if !p.typescript() {
		let err = p
			.err_builder("classes can only implement interfaces in TypeScript files")
			.primary(start..(p.marker_vec_range(&elems).end), "");

		p.error(err);
		is_valid = false;
	}

	let mut progress = ParserProgress::default();
	while p.cur_src() == "implements" {
		progress.assert_progressing(p);
		let start = p.cur_tok().range.start;
		p.bump_any();
		let elems = ts_heritage_clause(&mut *p, false);

		let err = p
			.err_builder("classes cannot have multiple `implements` clauses")
			.primary(start..p.marker_vec_range(&elems).end, "");

		p.error(err);
		is_valid = false;
	}

	list.complete(p, LIST);

	let completed_syntax = Present(implements_clause.complete(p, TS_IMPLEMENTS_CLAUSE));
	completed_syntax.into_conditional(is_valid)
}

fn extends_clause(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	if p.cur_src() != "extends" {
		return Absent;
	}

	let mut is_valid = true;
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
		is_valid = false;
	}

	// handle `extends foo extends bar` explicitly
	let mut progress = ParserProgress::default();
	while p.at(T![extends]) {
		progress.assert_progressing(p);
		p.bump_any();

		let elems = ts_heritage_clause(p, true);
		let err = p
			.err_builder("classes cannot extend multiple classes")
			.primary(p.marker_vec_range(&elems), "");

		p.error(err);
		is_valid = false;
	}

	Present(m.complete(p, JS_EXTENDS_CLAUSE)).into_conditional(is_valid)
}

struct ClassMembersList;

impl ParseNodeList for ClassMembersList {
	type ParsedElement = CompletedMarker;

	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax<Self::ParsedElement> {
		parse_class_member(p).or_invalid_to_unknown(p, JS_UNKNOWN_MEMBER)
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
			&ParseRecovery::new(
				JS_UNKNOWN_MEMBER,
				token_set![T![;], T![ident], T![async], T![yield], T!['}'], T![#]],
			),
			js_parse_error::expected_class_member,
		)
	}
}

fn parse_class_member(p: &mut Parser) -> ParsedSyntax<ConditionalSyntax> {
	let mut member_marker = p.start();
	let checkpoint = p.checkpoint();
	let mut member_is_valid = true;
	// test class_empty_element
	// class foo { ;;;;;;;;;; get foo() {};;;;}
	if p.eat(T![;]) {
		return Present(member_marker.complete(p, JS_EMPTY_CLASS_MEMBER)).into_valid();
	}

	let has_access_modifier = matches!(p.cur_src(), "public" | "private" | "protected");
	let mut offset = if has_access_modifier { 1 } else { 0 };

	let declare = p.nth_src(offset) == "declare";
	if declare {
		offset += 1;
	}
	let mut declare_diagnostic = None;

	// Let's assume declare is an identifier and not a keyword
	if declare && !has_access_modifier {
		// test class_declare
		// class B { declare() {} }
		// class B { declare = foo }

		// declare() and declare: foo
		if is_at_method_class_member(p, offset) {
			parse_literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return Present(parse_method_class_member_body(p, member_marker)).into_valid();
		} else if is_at_property_class_member(p, offset) {
			parse_literal_member_name(p).ok().unwrap(); // bump declare as identifier
			return parse_property_class_member_body(p, member_marker);
		} else {
			// test_err class_declare_member
			// class B { declare foo = bar }
			let msg = if p.typescript() {
				"a `declare` modifier cannot be applied to a class element"
			} else {
				"`declare` modifiers can only be used in TypeScript files"
			};
			let err = p.err_builder(msg).primary(p.cur_tok().range, "");

			declare_diagnostic = Some(err);
		}
	};

	if has_access_modifier {
		if is_at_method_class_member(p, offset) {
			if declare && declare_diagnostic.is_some() {
				// test_err class_declare_method
				// class B { declare fn() {} }
				let msg = if p.typescript() {
					"a `declare` modifier cannot be applied to a class method"
				} else {
					"`declare` modifiers can only be used in TypeScript files"
				};

				let err = p.err_builder(msg).primary(p.cur_tok().range, "");

				p.error(err);

				p.bump_remap(T![declare]);
				member_is_valid = false;
			}

			return Present(parse_method_class_member(p, member_marker))
				.into_conditional(member_is_valid);
		} else if is_at_property_class_member(p, offset) {
			if declare && declare_diagnostic.is_some() {
				p.bump_remap(T![declare]);
			}
			p.bump_any();

			return parse_property_class_member_body(p, member_marker);
		}
	}

	// test static_method
	// class foo {
	//  static foo(bar) {}
	//  static *foo() {}
	//  static async foo() {}
	//  static async *foo() {}
	// }
	let is_static = p.nth_src(offset) == "static";
	let static_token_range = p.nth_tok(offset).range;

	// Let's assume static is an identifier and not the static keyword
	if is_static {
		offset += 1;

		if is_at_method_class_member(p, offset) {
			let is_valid = Modifiers::default()
				.set_declare(declare)
				.set_accessibility(has_access_modifier)
				.set_static(is_static)
				.set_declare_err(declare_diagnostic)
				.consume(p);
			return Present(parse_method_class_member_body(p, member_marker))
				.into_conditional(is_valid);
		} else if is_at_property_class_member(p, offset) {
			Modifiers::default()
				.set_declare(declare)
				.set_accessibility(has_access_modifier)
				.set_static(is_static)
				.set_declare_err(declare_diagnostic)
				.consume(p);

			return if declare {
				property_declaration_class_member_body(p, member_marker, JS_LITERAL_MEMBER_NAME)
			} else {
				parse_property_class_member_body(p, member_marker)
			};
		}
	}

	// Seems that static is a keyword since the parser wasn't able to parse a valid method or property named static
	let is_valid = Modifiers::default()
		.set_declare(declare)
		.set_accessibility(has_access_modifier)
		.set_static(is_static)
		.set_remap_static(true)
		.set_declare_err(declare_diagnostic)
		.consume(p);

	if !is_valid {
		member_is_valid = false
	}

	let accessibility_marker = p.start();
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
		// test_err class_member_modifier
		// class A { abstract foo; }
		if !p.typescript() {
			let err = p
				.err_builder(
					"`abstract` and `readonly` modifiers can only be used in TypeScript files",
				)
				.primary(range, "");

			p.error(err);
			member_is_valid = false;
		}
		accessibility_marker.complete(p, TS_ACCESSIBILITY);
	} else {
		accessibility_marker.abandon(p);
		p.missing();
	}

	if !is_static && !has_access_modifier {
		let checkpoint = p.checkpoint();
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
				return Present(sig).into_conditional(member_is_valid);
			}
			Err(m) => {
				p.rewind(checkpoint);
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

		return Present(parse_method_class_member(&mut *guard, member_marker))
			.into_conditional(member_is_valid);
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

		return Present(parse_method_class_member(&mut *guard, member_marker))
			.into_conditional(member_is_valid);
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
			let mut constructor_has_error = false;
			if let Present(Valid(constructor)) = constructor {
				if is_static {
					let err = p
						.err_builder("constructors cannot be static")
						.primary(constructor.range(p), "")
						.secondary(static_token_range, "Remove the `static` word");

					p.error(err);
					constructor_has_error = true;
				}
			}

			if has_modifier {
				let err = p.err_builder("constructors cannot have modifiers").primary(
					abstract_range.or_else(|| readonly_range.clone()).unwrap(),
					"",
				);

				p.error(err);
				constructor_has_error = true;
			}

			return if constructor_has_error {
				if let Present(Valid(marker)) = constructor {
					Present(Invalid(marker.into()))
				} else {
					constructor
				}
			} else {
				constructor
			};
		} else {
			Present(parse_method_class_member_body(p, member_marker))
				.into_conditional(member_is_valid)
		};
	}

	if let NodeMarker(member) = member {
		if is_at_property_class_member(p, 0) {
			let property = if declare {
				property_declaration_class_member_body(p, member_marker, member.kind())
			} else {
				parse_property_class_member_body(p, member_marker)
			};

			if let Present(Valid(property)) = property {
				if is_constructor {
					let err = p
						.err_builder("class properties may not be called `constructor`")
						.primary(property.range(p), "");

					p.error(err);
				}
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

				let completed = if is_getter {
					p.expect_required(T!['(']);
					p.expect_required(T![')']);
					parse_ts_return_type_if_ts(p).or_missing(p);
					function_body(p)
						.or_missing_with_error(p, js_parse_error::expected_class_method_body);

					member_marker.complete(p, JS_GETTER_CLASS_MEMBER)
				} else {
					p.state.allow_object_expr = p.expect_required(T!['(']);
					parse_formal_param_pat(p)
						.or_missing_with_error(p, js_parse_error::expected_parameter);
					p.expect_required(T![')']);
					function_body(p)
						.or_missing_with_error(p, js_parse_error::expected_class_method_body);

					p.state.allow_object_expr = true;
					member_marker.complete(p, JS_SETTER_CLASS_MEMBER)
				};

				return Present(completed).into_conditional(member_is_valid);
			}
		}
	}

	// if we're arrived here it means that the parser hasn't advanced, so we rewind the parser
	// so we remove also possible empty tokens we marked along the way

	// test_err block_stmt_in_class
	// class S{{}}
	p.rewind(checkpoint);
	member_marker.abandon(p);
	Absent
}

fn property_declaration_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
	member_name_kind: SyntaxKind,
) -> ParsedSyntax<ConditionalSyntax> {
	let property = parse_property_class_member_body(p, member_marker);
	if let Present(Valid(property)) = property {
		if member_name_kind == JS_PRIVATE_CLASS_MEMBER_NAME {
			let err = p
				.err_builder("private class properties with `declare` are invalid")
				.primary(property.range(p), "");

			p.error(err);
		}
	}

	property
}

/// Parses the body of a property class member (anything after the member name)
fn parse_property_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
) -> ParsedSyntax<ConditionalSyntax> {
	let parsed_syntax = optional_member_token(p);
	let mut property_is_valid = if let Ok(optional_range) = parsed_syntax {
		if p.at(T![!]) {
			let range = p.cur_tok().range;

			let error = p
				.err_builder("class properties cannot be both optional and definite")
				.primary(range, "")
				.secondary(optional_range, "");

			p.error(error);
			p.bump_any(); // Bump ! token
			false
		} else {
			false
		}
	} else {
		true
	};

	// test_err class_member_bang
	// class B { foo!; }
	if p.at(T![!]) {
		let range = p.cur_tok().range;
		let error = p
			.err_builder("definite assignment assertions can only be used in TypeScript files")
			.primary(range, "");

		p.error(error);
		p.bump_any(); // Bump ! token
		property_is_valid = false;
	}

	maybe_ts_type_annotation(p);
	parse_initializer_clause(p).or_missing(p);

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

	Present(member_marker.complete(p, JS_PROPERTY_CLASS_MEMBER)).into_conditional(property_is_valid)
}

/// Eats the ? token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut Parser) -> Result<Range<usize>, ()> {
	if p.at(T![?]) {
		let range = p.cur_tok().range;
		// test_err optional_member
		// class B { foo?; }
		if !p.typescript() {
			let err = p
				.err_builder("`?` modifiers can only be used in TypeScript files")
				.primary(p.cur_tok().range, "");

			p.error(err);
		}
		p.bump_any();
		Ok(range)
	} else {
		p.missing();
		Err(())
	}
}

pub(crate) fn parse_initializer_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if p.at(T![=]) {
		let m = p.start();
		p.bump(T![=]);

		expr_or_assignment(p);

		Present(m.complete(p, JS_INITIALIZER_CLAUSE))
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
	let member_kind = if optional_member_token(p).is_ok() {
		JS_UNKNOWN_MEMBER
	} else {
		JS_METHOD_CLASS_MEMBER
	};

	ts_parameter_types(p);
	parse_parameter_list(p).or_missing_with_error(p, js_parse_error::expected_class_parameters);
	parse_ts_return_type_if_ts(p).or_missing(p);
	function_body(p).or_missing_with_error(p, js_parse_error::expected_class_method_body);

	m.complete(p, member_kind)
}

fn parse_constructor_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
) -> ParsedSyntax<ConditionalSyntax> {
	let constructor_is_valid = if let Ok(range) = optional_member_token(p) {
		let err = p
			.err_builder("constructors cannot be optional")
			.primary(range, "");

		p.error(err);
		false
	} else {
		true
	};

	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");

			let err = p
				.err_builder("constructors cannot have type parameters")
				.primary(ty.range(p), "");

			p.error(err);
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
			.or_missing_with_error(p, js_parse_error::expected_class_method_body);
	}

	// FIXME(RDambrosio016): if there is no body we need to issue errors for any assign patterns

	// TODO(RDambrosio016): ideally the following errors should just point to the modifiers
	let completed_marker = member_marker.complete(p, JS_CONSTRUCTOR_CLASS_MEMBER);
	Present(completed_marker).into_conditional(constructor_is_valid)
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
		Modifiers::default().set_accessibility(true).consume(p);
		// test_err class_constructor_parameter
		// class B { constructor(protected b) {} }
		if !p.typescript() {
			let err = p
				.err_builder("accessibility modifiers for a parameter inside a constructor can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
		}
		true
	} else {
		false
	};

	let has_readonly = if let Some(range) = ts_modifier(p, &["readonly"]) {
		// test_err class_constructor_parameter_readonly
		// class B { constructor(readonly b) {} }
		if !p.typescript() {
			let err = p
				.err_builder("readonly modifiers for a parameter inside a constructor can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			p.bump_any();
		}
		true
	} else {
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

#[derive(Default)]
pub(crate) struct Modifiers {
	declare: bool,
	accessibility: bool,
	is_static: bool,
	remap_static: bool,
	declare_err: Option<Diagnostic>,
}

impl Modifiers {
	pub fn set_declare(mut self, is_declare: bool) -> Self {
		self.declare = is_declare;
		self
	}

	pub fn set_accessibility(mut self, accessibility: bool) -> Self {
		self.accessibility = accessibility;
		self
	}

	pub fn set_static(mut self, is_static: bool) -> Self {
		self.is_static = is_static;
		self
	}

	pub fn set_remap_static(mut self, should_remap: bool) -> Self {
		self.remap_static = should_remap;
		self
	}

	pub fn set_declare_err(mut self, diagnostic: Option<Diagnostic>) -> Self {
		self.declare_err = diagnostic;
		self
	}

	/// It consumes modifiers like:
	///
	/// - `public`
	/// - `private`
	/// - `protected`
	/// - `static`
	/// - `declare`
	///
	/// And it advances the parser.
	///
	/// It creates a diagnostic if some modifiers are not correct.
	pub fn consume(self, p: &mut Parser) -> bool {
		let mut member_is_valid = true;
		if self.accessibility {
			let kind = match p.cur_src() {
				"public" => PUBLIC_KW,
				"private" => PRIVATE_KW,
				"protected" => PROTECTED_KW,
				_ => unreachable!(),
			};
			if !p.typescript() {
				// test_err class_invalid_modifiers
				// class A { public foo() {} }
				let range = p.cur_tok().range;
				let err = p
					.err_builder("accessibility modifiers can only be used in TypeScript files")
					.primary(range, "");

				p.error(err);
				p.bump_any();
				member_is_valid = false;
			} else {
				p.bump_remap(kind);
			}
		} else {
			p.missing();
		}
		if self.declare {
			p.bump_remap(T![declare]);
			if let Some(err) = self.declare_err {
				p.error(err);
				member_is_valid = false;
			}
		} else {
			p.missing();
		}
		if self.is_static && self.remap_static {
			p.bump_remap(STATIC_KW);
		} else if self.is_static && !self.remap_static {
			// Guaranteed to be at the static keyword, parsing a class member must succeed
			parse_class_member_name(p).ok().unwrap();
		} else {
			p.missing();
		}
		member_is_valid
	}
}

const PROPERTY_START_SET: TokenSet = token_set![T![!], T![:], T![=], T!['}'], T![;]];

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
