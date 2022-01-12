use crate::parser::{ParsedSyntax, ParserProgress, RecoveryResult};
use crate::state::{
	AllowObjectExpression, ChangeParserState, EnableStrictMode, InAsync, InConstructor, InFunction,
	InGenerator,
};
use crate::syntax::binding::parse_binding;
use crate::syntax::decl::{parse_formal_param_pat, parse_parameter_list, parse_parameters_list};
use crate::syntax::expr::parse_expr_or_assignment;
use crate::syntax::function::{
	function_body, parse_ts_type_annotation_or_error, ts_parameter_types,
};
use crate::syntax::js_parse_error;
use crate::syntax::js_parse_error::expected_parameter;
use crate::syntax::object::{
	is_at_literal_member_name, parse_computed_member_name, parse_literal_member_name,
};
use crate::syntax::stmt::{is_semi, optional_semi, parse_block_impl};
use crate::syntax::typescript::{
	maybe_ts_type_annotation, ts_heritage_clause, ts_modifier, ts_type_params,
	DISALLOWED_TYPE_NAMES,
};
use crate::JsSyntaxFeature::TypeScript;
use crate::ParsedSyntax::{Absent, Present};
use crate::{
	CompletedMarker, Event, Marker, ParseNodeList, ParseRecovery, Parser, StrictMode,
	SyntaxFeature, TokenSet,
};
use rome_rowan::SyntaxKind;
use rslint_syntax::JsSyntaxKind::*;
use rslint_syntax::{JsSyntaxKind, T};
use std::ops::Range;

/// Parses a class expression, e.g. let a = class {}
pub(super) fn parse_class_expression(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![class]) {
		return Absent;
	}

	let m = p.start();
	parse_class(p, m, ClassKind::Expression)
}

// test class_declaration
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
pub(super) fn parse_class_statement(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![class]) {
		return Absent;
	}

	let m = p.start();
	parse_class(p, m, ClassKind::Statement)
}

// test export_class_clause
// export class A {}
// export class A extends B {}
pub(super) fn parse_export_class_clause(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![class]) {
		return Absent;
	}

	let m = p.start();
	parse_class(p, m, ClassKind::Export)
}

// test export_default_class_clause
// export default class {}
pub(super) fn parse_export_default_class_case(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![default]) && !p.nth_at(1, T![class]) {
		return Absent;
	}

	let m = p.start();
	p.bump(T![default]);

	parse_class(p, m, ClassKind::ExportDefault)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
	Statement,
	Expression,
	Export,
	ExportDefault,
}

impl ClassKind {
	fn is_id_optional(&self) -> bool {
		matches!(self, ClassKind::Expression | ClassKind::ExportDefault)
	}
}

impl From<ClassKind> for JsSyntaxKind {
	fn from(kind: ClassKind) -> Self {
		match kind {
			ClassKind::Statement => JS_CLASS_STATEMENT,
			ClassKind::Expression => JS_CLASS_EXPRESSION,
			ClassKind::Export => JS_EXPORT_CLASS_CLAUSE,
			ClassKind::ExportDefault => JS_EXPORT_DEFAULT_CLASS_CLAUSE,
		}
	}
}

fn parse_class(p: &mut Parser, m: Marker, kind: ClassKind) -> ParsedSyntax {
	let mut class_is_valid = true;
	let class_token_range = p.cur_tok().range();

	p.expect(T![class]);

	let p = &mut *p.with_scoped_state(EnableStrictMode(StrictMode::Class(p.cur_tok().range())));

	// test_err class_decl_no_id
	// class {}
	// class implements B {}

	let id = if p.cur_src() == "implements" {
		Absent
	} else {
		parse_binding(p)
	};

	// parse class id
	match id {
		Present(id) => {
			let text = p.span_text(id.range(p));
			if p.typescript() && DISALLOWED_TYPE_NAMES.contains(&text) {
				let err = p
					.err_builder(&format!(
							"`{}` cannot be used as a class name because it is already reserved as a type",
							text
						))
					.primary(id.range(p), "");

				p.error(err);
			}
		}
		Absent => {
			if !kind.is_id_optional() {
				let err = p
					.err_builder("class declarations must have a name")
					.primary(class_token_range.start..p.cur_tok().start(), "");

				p.error(err);
			}
		}
	}

	if p.at(T![<]) {
		if let Some(mut complete) = ts_type_params(p) {
			complete.err_if_not_ts(
				p,
				"classes can only have type parameters in TypeScript files",
			);
		}
	}

	extends_clause(p).ok();

	if implements_clause(p).is_present() && TypeScript.is_unsupported(p) {
		class_is_valid = false;
	}

	p.expect(T!['{']);
	ClassMembersList.parse_list(p);
	p.expect(T!['}']);

	let mut class_marker = m.complete(p, kind.into());

	if !class_is_valid {
		class_marker.change_to_unknown(p);
	}

	Present(class_marker)
}

fn implements_clause(p: &mut Parser) -> ParsedSyntax {
	if p.cur_src() != "implements" {
		return Absent;
	}

	let mut is_valid = true;
	let implements_clause = p.start();

	let start = p.cur_tok().start();
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
		let start = p.cur_tok().start();
		p.bump_any();
		let elems = ts_heritage_clause(&mut *p, false);

		let err = p
			.err_builder("classes cannot have multiple `implements` clauses")
			.primary(start..p.marker_vec_range(&elems).end, "");

		p.error(err);
		is_valid = false;
	}

	list.complete(p, TS_TYPE_LIST);

	let kind = if is_valid {
		TS_IMPLEMENTS_CLAUSE
	} else {
		JS_UNKNOWN
	};
	Present(implements_clause.complete(p, kind))
}

fn extends_clause(p: &mut Parser) -> ParsedSyntax {
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

	let mut completed = m.complete(p, JS_EXTENDS_CLAUSE);

	if !is_valid {
		completed.change_to_unknown(p);
	}

	Present(completed)
}

struct ClassMembersList;

impl ParseNodeList for ClassMembersList {
	fn parse_element(&mut self, p: &mut Parser) -> ParsedSyntax {
		parse_class_member(p)
	}

	fn is_at_list_end(&mut self, p: &mut Parser) -> bool {
		p.at(T!['}'])
	}

	fn recover(&mut self, p: &mut Parser, parsed_element: ParsedSyntax) -> RecoveryResult {
		// test_err invalid_method_recover
		// class {
		//   [1 + 1] = () => {
		//     let a=;
		//   };
		// };
		parsed_element.or_recover(
			p,
			&ParseRecovery::new(
				JS_UNKNOWN_MEMBER,
				token_set![T![;], T![ident], T![async], T![yield], T!['}'], T![#]],
			),
			js_parse_error::expected_class_member,
		)
	}

	fn list_kind() -> JsSyntaxKind {
		JS_CLASS_MEMBER_LIST
	}
}

// test class_declare
// class B { declare() {} }
// class B { declare = foo }

// test static_method
// class foo {
//  static foo(bar) {}
//  static *foo() {}
//  static async foo() {}
//  static async *foo() {}
// }

fn parse_class_member(p: &mut Parser) -> ParsedSyntax {
	let member_marker = p.start();
	// test class_empty_element
	// class foo { ;;;;;;;;;; get foo() {};;;;}
	if p.eat(T![;]) {
		return Present(member_marker.complete(p, JS_EMPTY_CLASS_MEMBER));
	}

	let (valid, modifiers) = match parse_class_member_modifiers(p) {
		Ok(modifiers) => (true, modifiers),
		Err(modifiers) => (false, modifiers),
	};

	let member = parse_class_member_impl(p, member_marker, modifiers);

	if !valid {
		member.map(|mut syntax| {
			syntax.change_to_unknown(p);
			syntax
		})
	} else {
		member
	}
}

fn parse_class_member_impl(
	p: &mut Parser,
	member_marker: Marker,
	modifiers: ClassMemberModifiers,
) -> ParsedSyntax {
	let generator_range = p.cur_tok().range();
	let checkpoint = p.checkpoint();

	// Seems like we're at a generator method
	if p.at(T![*]) {
		p.bump_any(); // bump * token

		let is_constructor = p.cur_src() == "constructor";
		if let Some(range) = modifiers.get_range(ModifierKind::Readonly) {
			let err = p
				.err_builder("class methods cannot be readonly")
				.primary(range, "");

			p.error(err);
		}

		if is_constructor {
			let err = p
				.err_builder("constructors can't be generators")
				.primary(generator_range, "");

			p.error(err);
		}

		return p.with_state(InFunction(true).and(InGenerator(true)), |p| {
			Present(parse_method_class_member(p, member_marker))
		});
	};

	// Seems like we're at an async method
	if p.cur_src() == "async"
		&& !p.nth_at(1, T![?])
		&& !is_at_method_class_member(p, 1)
		&& !p.has_linebreak_before_n(1)
	{
		let async_range = p.cur_tok().range();
		p.bump_remap(T![async]);
		let in_generator = p.eat(T![*]);

		if p.cur_src() == "constructor" {
			let err = p
				.err_builder("constructors cannot be async")
				.primary(async_range, "");

			p.error(err);
		}

		if let Some(range) = modifiers.get_range(ModifierKind::Readonly) {
			let err = p
				.err_builder("methods cannot be readonly")
				.primary(range, "");

			p.error(err);
		}

		return Present(
			p.with_state(
				InFunction(true)
					.and(InGenerator(in_generator))
					.and(InAsync(true)),
				|p| parse_method_class_member(p, member_marker),
			),
		);
	}

	let member_name = p.cur_src();
	let is_constructor = matches!(
		member_name,
		"constructor" | "\"constructor\"" | "'constructor'"
	) && modifiers.get_range(ModifierKind::Static).is_none();
	let member_name =
		parse_class_member_name(p).or_add_diagnostic(p, js_parse_error::expected_class_member_name);

	if is_at_method_class_member(p, 0) {
		// test class_static_constructor_method
		// class B { static constructor() {} }

		// test constructor_class_member
		// class Foo {
		// 	constructor(a) {
		// 		this.a = a;
		// 	}
		// }
		// class Bar {
		// 	"constructor"(b) {
		// 		this.b = b;
		// 	}
		// }
		return if is_constructor {
			let constructor = parse_constructor_class_member_body(p, member_marker);

			return constructor.map(|constructor| {
				if constructor.kind().is_unknown() {
					return constructor;
				}

				if let Some(readonly_range) = modifiers.get_range(ModifierKind::Readonly) {
					p.error(
						p.err_builder("constructors cannot be `readonly`")
							.primary(readonly_range, ""),
					);
				}
				if let Some(abstract_range) = modifiers.get_range(ModifierKind::Abstract) {
					p.error(
						p.err_builder("constructors cannot be `abstract`")
							.primary(abstract_range, ""),
					);
				}

				constructor
			});
		} else {
			// test method_class_member
			// class Test {
			// 	method() {}
			// 	async asyncMethod() {}
			// 	async* asyncGeneratorMethod() {}
			// 	* generatorMethod() {}
			// 	"foo"() {}
			// 	["foo" + "bar"]() {}
			// 	5() {}
			// 	#private() {}
			// }
			// class ContextualKeywords {
			//    // Methods called static
			// 	  static() {}
			// 	  async static() {}
			// 	  * static() {}
			//    async* static() {}
			//    declare() {}
			// 	  get() {} // Method called get
			// 	  set() {} // Method called set
			// }
			// class Static {
			// 	  static method() {}
			// 	  static async asyncMethod() {}
			// 	  static async* asyncGeneratorMethod() {}
			// 	  static * generatorMethod() {}
			//    static static() {}
			// 	  static async static() {}
			// 	  static async* static() {}
			// 	  static * static() {}
			// }
			if let Some(range) = modifiers.get_range(ModifierKind::Readonly) {
				let err = p
					.err_builder("class methods cannot be readonly")
					.primary(range, "");

				p.error(err);
			}

			Present(parse_method_class_member_body(p, member_marker))
		};
	}

	if let Some(member_name) = member_name {
		// test property_class_member
		// class foo {
		// 	property
		// 	declare;
		// 	initializedProperty = "a"
		// 	"a";
		// 	5
		// 	["a" + "b"]
		// 	static staticProperty
		// 	static staticInitializedProperty = 1
		// 	#private
		// 	#privateInitialized = "a"
		// 	static #staticPrivate
		// 	static #staticPrivateInitializedProperty = 1
		// }
		if is_at_property_class_member(p, 0) {
			// test_err class_declare_member
			// class B { declare foo }
			let property = if modifiers.get_range(ModifierKind::Declare).is_some() {
				property_declaration_class_member_body(p, member_marker, member_name.kind())
			} else {
				parse_property_class_member_body(p, member_marker)
			};

			return property.map(|mut property| {
				if !property.kind().is_unknown() && is_constructor {
					let err = p
						.err_builder("class properties may not be called `constructor`")
						.primary(property.range(p), "");

					p.error(err);
					property.change_to_unknown(p);
				}
				property
			});
		}

		if member_name.kind() == JS_LITERAL_MEMBER_NAME {
			let is_at_line_break_or_generator = p.has_linebreak_before_n(0) && p.at(T![*]);
			let member_name_text = member_name.text(p);
			if matches!(member_name_text, "get" | "set") && !is_at_line_break_or_generator {
				let is_getter = member_name_text == "get";

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
				//
				// test_err method_getter_err
				// class foo {
				//  get {}
				// }
				//
				// test_err getter_class_no_body
				// class Setters {
				//   get foo()
				//
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
				//
				// test_err setter_class_member
				// class Setters {
				//   set foo() {}
				// }
				//
				// test_err setter_class_no_body
				// class Setters {
				//   set foo(a)

				// The tree currently holds a STATIC_MEMBER_NAME node that wraps a ident token but we now found
				// out that the 'get' or 'set' isn't a member name in this context but instead are the
				// 'get'/'set' keywords for getters/setters. That's why we need to undo the member name node,
				// extract the 'get'/'set' ident token and change its kind to 'get'/'set'
				match p.events[(member_name.start_pos as usize) + 1] {
					Event::Token { ref mut kind, .. } => {
						*kind = if is_getter { T![get] } else { T![set] }
					}
					_ => unreachable!(),
				};
				member_name.undo_completion(p).abandon(p);

				if let Some(range) = modifiers.get_range(ModifierKind::Readonly) {
					let err = p
						.err_builder("getters and setters cannot be readonly")
						.primary(range, "");

					p.error(err);
				}

				// So we've seen a get that now must be followed by a getter/setter name
				parse_class_member_name(p)
					.or_add_diagnostic(p, js_parse_error::expected_class_member_name);

				let completed = if is_getter {
					p.expect(T!['(']);
					p.expect(T![')']);
					parse_ts_type_annotation_or_error(p).ok();
					function_body(p)
						.or_add_diagnostic(p, js_parse_error::expected_class_method_body);

					member_marker.complete(p, JS_GETTER_CLASS_MEMBER)
				} else {
					let has_l_paren = p.expect(T!['(']);
					p.with_state(AllowObjectExpression(has_l_paren), |p| {
						parse_formal_param_pat(p)
							.or_add_diagnostic(p, js_parse_error::expected_parameter);
						p.expect(T![')']);
						function_body(p)
							.or_add_diagnostic(p, js_parse_error::expected_class_method_body);
					});

					member_marker.complete(p, JS_SETTER_CLASS_MEMBER)
				};

				return Present(completed);
			}
		}
	}

	// if we're arrived here it means that the parser has advanced but we have no clue what kind of member
	// this is supposed to be. Let's rewind the parser so that the error recovery in the members loop
	// kicks in and any potential `missing` slots get removed.

	// test_err block_stmt_in_class
	// class S{{}}
	p.rewind(checkpoint);
	member_marker.abandon(p);
	Absent
}

fn property_declaration_class_member_body(
	p: &mut Parser,
	member_marker: Marker,
	member_name_kind: JsSyntaxKind,
) -> ParsedSyntax {
	let property = parse_property_class_member_body(p, member_marker);
	property.map(|mut property| {
		if member_name_kind == JS_PRIVATE_CLASS_MEMBER_NAME {
			let err = p
				.err_builder("private class properties with `declare` are invalid")
				.primary(property.range(p), "");

			p.error(err);
			property.change_to_unknown(p);
		}

		property
	})
}

/// Parses the body of a property class member (anything after the member name)
fn parse_property_class_member_body(p: &mut Parser, member_marker: Marker) -> ParsedSyntax {
	let optional_token = optional_member_token(p);
	let mut property_is_valid = optional_token.is_ok();

	let range = p.cur_tok().range();
	if p.eat(T![!]) {
		if let Ok(Some(optional_token)) = optional_token {
			let range = p.cur_tok().range();

			let error = p
				.err_builder("class properties cannot be both optional and definite")
				.primary(range, "")
				.secondary(optional_token, "");

			p.error(error);
			p.bump_any(); // Bump ! token
			property_is_valid = false;
		} else if !p.typescript() {
			// test_err class_member_bang
			// class B { foo!; }
			let error = p
				.err_builder("definite assignment assertions can only be used in TypeScript files")
				.primary(range, "");

			p.error(error);
			property_is_valid = false;
		}
	}

	parse_ts_type_annotation_or_error(p).ok();
	parse_initializer_clause(p).ok();

	if !optional_semi(p) {
		// Gets the start of the member
		let start = match p.events[member_marker.old_start as usize] {
			Event::Start { start, .. } => start,
			_ => unreachable!(),
		};

		let err = p
			.err_builder("expected a semicolon for a class property, but found none")
			.primary(start..p.cur_tok().start(), "");

		p.error(err);
	}

	let mut property = member_marker.complete(p, JS_PROPERTY_CLASS_MEMBER);
	if !property_is_valid {
		property.change_to_unknown(p);
	}

	Present(property)
}

/// Eats the ? token for optional member. Emits an error if this isn't typescript
fn optional_member_token(p: &mut Parser) -> Result<Option<Range<usize>>, ()> {
	if p.eat(T![?]) {
		let range = p.cur_tok().range();
		p.bump_any();

		// test_err optional_member
		// class B { foo?; }
		if p.typescript() {
			Ok(Some(range))
		} else {
			let err = p
				.err_builder("`?` modifiers can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			Err(())
		}
	} else {
		Ok(None)
	}
}

// test_err class_property_initializer
// class B { lorem = ; }
pub(crate) fn parse_initializer_clause(p: &mut Parser) -> ParsedSyntax {
	if p.at(T![=]) {
		let m = p.start();
		p.bump(T![=]);

		parse_expr_or_assignment(p)
			.or_add_diagnostic(p, js_parse_error::expected_expression_assignment);

		Present(m.complete(p, JS_INITIALIZER_CLAUSE))
	} else {
		Absent
	}
}

fn parse_method_class_member(p: &mut Parser, m: Marker) -> CompletedMarker {
	parse_class_member_name(p).or_add_diagnostic(p, js_parse_error::expected_class_member_name);
	parse_method_class_member_body(p, m)
}

// test_err class_member_method_parameters
// class B { foo(a {} }

// test_err class_member_method_body
// class B { foo(a)

/// Parses the body (everything after the identifier name) of a method class member
fn parse_method_class_member_body(p: &mut Parser, m: Marker) -> CompletedMarker {
	let member_kind = if optional_member_token(p).is_ok() {
		JS_METHOD_CLASS_MEMBER
	} else {
		JS_UNKNOWN_MEMBER
	};

	ts_parameter_types(p);
	parse_parameter_list(p).or_add_diagnostic(p, js_parse_error::expected_class_parameters);
	parse_ts_type_annotation_or_error(p).ok();
	function_body(p).or_add_diagnostic(p, js_parse_error::expected_class_method_body);

	m.complete(p, member_kind)
}

fn parse_constructor_class_member_body(p: &mut Parser, member_marker: Marker) -> ParsedSyntax {
	if let Ok(Some(range)) = optional_member_token(p) {
		let err = p
			.err_builder("constructors cannot be optional")
			.primary(range, "");

		p.error(err);
	}

	let mut constructor_is_valid = true;
	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");

			let err = p
				.err_builder("constructors cannot have type parameters")
				.primary(ty.range(p), "");

			p.error(err);
			constructor_is_valid = false;
		}
	}

	parse_constructor_parameter_list(p)
		.or_add_diagnostic(p, js_parse_error::expected_constructor_parameters);

	if let Some(range) = maybe_ts_type_annotation(p) {
		let err = p
			.err_builder("constructors cannot have type annotations")
			.primary(range, "");

		p.error(err);
		constructor_is_valid = false;
	}

	p.with_state(InFunction(true).and(InConstructor(true)), |p| {
		parse_block_impl(p, JS_FUNCTION_BODY)
			.or_add_diagnostic(p, js_parse_error::expected_class_method_body);
	});

	// FIXME(RDambrosio016): if there is no body we need to issue errors for any assign patterns

	let mut completed_marker = member_marker.complete(p, JS_CONSTRUCTOR_CLASS_MEMBER);

	if !constructor_is_valid {
		completed_marker.change_to_unknown(p);
	}

	Present(completed_marker)
}

fn parse_constructor_parameter_list(p: &mut Parser) -> ParsedSyntax {
	let m = p.start();
	parse_parameters_list(
		p,
		parse_constructor_parameter,
		JS_CONSTRUCTOR_PARAMETER_LIST,
	);
	Present(m.complete(p, JS_CONSTRUCTOR_PARAMETERS))
}

fn parse_constructor_parameter(p: &mut Parser) -> ParsedSyntax {
	// test_err class_constructor_parameter
	// class B { constructor(protected b) {} }

	if matches!(p.cur_src(), "public" | "protected" | "private" | "readonly") {
		let ts_param = p.start();
		if let Some(range) = parse_access_modifier(p) {
			if !p.typescript() {
				let err = p
					.err_builder("accessibility modifiers for a parameter inside a constructor can only be used in TypeScript files")
					.primary(range, "");

				p.error(err);
			}
		}

		if let Some(range) = ts_modifier(p, &["readonly"]) {
			// test_err class_constructor_parameter_readonly
			// class B { constructor(readonly b) {} }
			if !p.typescript() {
				let err = p
					.err_builder("readonly modifiers for a parameter inside a constructor can only be used in TypeScript files")
					.primary(range, "");

				p.error(err);
			}
		}

		parse_formal_param_pat(p).or_add_diagnostic(p, expected_parameter);

		Present(ts_param.complete(p, TS_CONSTRUCTOR_PARAM))
	} else {
		parse_formal_param_pat(p)
	}
}

fn is_at_class_member_name(p: &Parser, offset: usize) -> bool {
	matches!(p.nth(offset), T![#] | T!['[']) || is_at_literal_member_name(p, offset)
}

/// Parses a `JsAnyClassMemberName` and returns its completion marker
fn parse_class_member_name(p: &mut Parser) -> ParsedSyntax {
	match p.cur() {
		T![#] => parse_private_class_member_name(p),
		T!['['] => parse_computed_member_name(p),
		_ => parse_literal_member_name(p),
	}
}

pub(crate) fn parse_private_class_member_name(p: &mut Parser) -> ParsedSyntax {
	if !p.at(T![#]) {
		return Absent;
	}
	let m = p.start();
	p.expect(T![#]);
	p.expect(T![ident]);
	Present(m.complete(p, JS_PRIVATE_CLASS_MEMBER_NAME))
}

fn parse_access_modifier(p: &mut Parser) -> Option<Range<usize>> {
	let kind = match p.cur_src() {
		"public" => PUBLIC_KW,
		"private" => PRIVATE_KW,
		"protected" => PROTECTED_KW,
		_ => return None,
	};

	let range = p.cur_tok().range();
	p.bump_remap(kind);
	Some(range)
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

fn is_at_modifier(p: &Parser, offset: usize) -> bool {
	matches!(
		p.nth_src(offset),
		"public" | "private" | "protected" | "static" | "abstract" | "readonly" | "declare"
	) || is_at_class_member_name(p, offset)
}

// test_err class_invalid_modifiers
// class A { public foo() {} }
// class B { static static foo() {} }

// test class_member_modifiers
// class A { public() {} }
// class A { static protected() {} }
// class A { static }

/// Parses all possible modifiers regardless of what the current member is. It's up to the caller
/// to create diagnostics for not allowed modifiers.
///
/// Inserts `missing` marker for all possible class modifiers. These must be undone if a member
/// doesn't support a specific modifier.
///
/// Returns [Ok] if the modifiers are in the correct order, no typescript modifiers are used, or this
/// is a typescript file
/// Returns [Err] otherwise
fn parse_class_member_modifiers(
	p: &mut Parser,
) -> Result<ClassMemberModifiers, ClassMemberModifiers> {
	let mut previous_modifier: Option<Modifier> = None;
	let mut valid = true;
	let mut modifiers = ClassMemberModifiers::default();

	let mut progress = ParserProgress::default();
	loop {
		progress.assert_progressing(p);

		if let Some(current_modifier) = parse_modifier(p) {
			if let Some(existing) = modifiers.get_range(current_modifier.kind) {
				let name = p.span_text(current_modifier.range.clone());
				let err = p
					.err_builder(&format!("`{}` modifier already seen.", name,))
					.primary(
						current_modifier.range.clone(),
						&format!("remove the duplicate `{}` here", name),
					)
					.secondary(existing.clone(), "first usage");
				p.error(err);
				valid = false;
				continue;
			}

			// Checks the precedence of modifiers. The precedence is defined by the order of the
			// enum variants in [Modifier]
			if let Some(previous_modifier) = &previous_modifier {
				if previous_modifier.kind > current_modifier.kind {
					p.error(
						p.err_builder(&format!(
							"`{}` modifier must precede `{}`.",
							p.span_text(current_modifier.range.clone()),
							p.span_text(previous_modifier.range.clone())
						))
						.primary(current_modifier.range.clone(), "")
						.secondary(previous_modifier.range.clone(), ""),
					);
					modifiers.set_range(current_modifier.clone());
					valid = false;
					continue;
				}
			}

			if !p.typescript() && !matches!(&current_modifier.kind, ModifierKind::Static) {
				p.error(
					p.err_builder(&format!(
						"`{}` modifier can only be used in TypeScript files",
						p.span_text(current_modifier.range.clone())
					))
					.primary(current_modifier.range.clone(), ""),
				);
				valid = false;
			}

			modifiers.set_range(current_modifier.clone());

			previous_modifier = Some(current_modifier);
		} else if valid {
			return Ok(modifiers);
		} else {
			return Err(modifiers);
		}
	}
}

// test_err class_declare_method
// class B { declare fn() {} }
//
// test_err class_member_modifier
// class A { abstract foo; }
fn parse_modifier(p: &mut Parser) -> Option<Modifier> {
	// Test if this modifier is followed by another modifier, member name or any other token that
	// starts a new member. If that's the case, then this is fairly likely a modifier. If not, then
	// this is probably not a modifier, but the name of the member. For example, all these are valid
	// members: `static() {}, private() {}, protected() {}`... but are modifiers if followed by another modifier or a name:
	// `static x() {} private static() {}`...
	if !is_at_modifier(p, 1)
		&& !is_at_class_member_name(p, 1)
		&& !matches!(p.nth(1), T![*] | T![async])
		|| p.has_linebreak_before_n(1)
	{
		// all modifiers can also be valid member names. That's why we shouldn't parse a modifier
		// if it isn't followed by a valid member name or another modifier
		return None;
	}

	let range = p.cur_tok().range();

	let (modifier_kind, kw_kind) = match p.cur_src() {
		"declare" => (ModifierKind::Declare, DECLARE_KW),
		"public" => (ModifierKind::Accessibility, PUBLIC_KW),
		"protected" => (ModifierKind::Accessibility, PROTECTED_KW),
		"private" => (ModifierKind::Accessibility, PRIVATE_KW),
		"static" => (ModifierKind::Static, STATIC_KW),
		"readonly" => (ModifierKind::Readonly, READONLY_KW),
		"abstract" => (ModifierKind::Abstract, ABSTRACT_KW),
		_ => {
			return None;
		}
	};

	p.bump_remap(kw_kind);

	Some(Modifier {
		range,
		kind: modifier_kind,
	})
}

/// The different modifiers a class member may have.
/// The order represents the order of the modifiers as they should appear in the source text
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
#[repr(u8)]
enum ModifierKind {
	Declare = 0,
	Accessibility = 1,
	Static = 2,
	Readonly = 3,
	Abstract = 4,
	// Update the array length in `ParsedModifiers` when adding/removing modifiers here or you'll
	// have a bad time :D
}

/// Stores the range of a parsed modifier with its kind
#[derive(Debug, Clone)]
struct Modifier {
	kind: ModifierKind,
	range: Range<usize>,
}

/// Stores all parsed modifiers in an array, and ensures that "missing" markers are inserted
/// for all modifiers. These missing markers can later be undone if they are not needed for a specific
/// member type (for example, `declare` is only allowed on properties).
#[derive(Debug, Default)]
struct ClassMemberModifiers {
	// replace length with std::mem::variant_count() when it becomes stable
	modifiers: [Option<Range<usize>>; 5],
}

impl ClassMemberModifiers {
	/// Returns the range for the passed in modifier or [None] if the modifier isn't set or is a missing marker
	fn get_range(&self, kind: ModifierKind) -> Option<&Range<usize>> {
		self.modifiers[kind as usize].as_ref()
	}

	/// Sets the range of a parsed modifier
	fn set_range(&mut self, modifier: Modifier) {
		self.modifiers[modifier.kind as usize] = Some(modifier.range);
	}
}
