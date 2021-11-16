use crate::syntax::decl::{formal_param_pat, parameters_common};
use crate::syntax::expr::{assign_expr, identifier_name};
use crate::syntax::function::{args_body, function_body_or_declaration};
use crate::syntax::object::object_prop_name;
use crate::syntax::pat::binding_identifier;
use crate::syntax::typescript::{
	abstract_readonly_modifiers, maybe_ts_type_annotation, try_parse_index_signature,
	ts_heritage_clause, ts_modifier, ts_type_params, DISALLOWED_TYPE_NAMES,
};
use crate::{CompletedMarker, Event, Marker, Parser, ParserState, StrictMode, TokenSet};
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

/// Parses a class expression, e.g. let a = class {}
pub(super) fn class_expression(p: &mut Parser) -> CompletedMarker {
	class_impl(p, ClassKind::Expression)
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
/// Parses a class declaration
pub(super) fn class_declaration(p: &mut Parser) -> CompletedMarker {
	class_impl(p, ClassKind::Declaration)
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum ClassKind {
	Declaration,
	Expression,
}

impl From<ClassKind> for SyntaxKind {
	fn from(kind: ClassKind) -> Self {
		match kind {
			ClassKind::Declaration => SyntaxKind::CLASS_DECL,
			ClassKind::Expression => SyntaxKind::CLASS_EXPR,
		}
	}
}

fn class_impl(p: &mut Parser, kind: ClassKind) -> CompletedMarker {
	let m = p.start();
	p.expect(T![class]);
	// class bodies are implicitly strict
	let mut guard = p.with_state(ParserState {
		strict: Some(StrictMode::Class(p.cur_tok().range)),
		..p.state.clone()
	});

	let idt = if (guard.at_ts(token_set![T![await], T![yield], T![ident]])
		&& guard.cur_src() != "implements")
		|| (guard.typescript() && guard.at(T![this]))
	{
		if guard.at(T![this]) {
			let m = guard.start();
			guard.bump_remap(T![ident]);
			Some(m.complete(&mut *guard, NAME))
		} else {
			binding_identifier(&mut *guard)
		}
	} else {
		None
	};

	if idt.is_none() && kind == ClassKind::Declaration && !guard.state.in_default {
		let err = guard
			.err_builder("class declarations must have a name")
			.primary(guard.cur_tok().range, "");

		guard.error(err);
	}

	if let Some(idt) = idt {
		let text = guard.span_text(idt.range(&*guard));
		if guard.typescript() && DISALLOWED_TYPE_NAMES.contains(&text) {
			let err = guard
				.err_builder(&format!(
					"`{}` cannot be used as a class name because it is already reserved as a type",
					text
				))
				.primary(idt.range(&*guard), "");

			guard.error(err);
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

	if guard.at(T![<]) {
		ts_type_params(&mut *guard);
	}

	if guard.cur_src() == "extends" {
		guard.bump_any();
		let mut elems = ts_heritage_clause(&mut *guard, true);
		if !elems.is_empty() {
			elems
				.remove(0)
				.undo_completion(&mut *guard)
				.abandon(&mut *guard);
		}
		for elem in elems {
			let err = guard
				.err_builder("classes cannot extend multiple classes")
				.primary(elem.range(&*guard), "");

			guard.error(err);
		}
	};

	// handle `extends foo extends bar` explicitly
	while guard.at(T![extends]) {
		let m = guard.start();
		guard.bump_any();
		let elems = ts_heritage_clause(&mut *guard, true);
		let err = guard
			.err_builder("classes cannot extend multiple classes")
			.primary(guard.marker_vec_range(&elems), "");

		guard.error(err);
		m.complete(&mut *guard, ERROR);
	}

	let mut implements_list = None;
	if guard.cur_src() == "implements" {
		let start = guard.cur_tok().range.start;
		let maybe_err = guard.start();
		guard.bump_remap(T![implements]);

		implements_list = Some(guard.start());
		let elems = ts_heritage_clause(&mut *guard, false);
		if !guard.typescript() {
			let err = guard
				.err_builder("classes can only implement interfaces in TypeScript files")
				.primary(start..(guard.marker_vec_range(&elems).end), "");

			guard.error(err);
			maybe_err.complete(&mut *guard, ERROR);
		} else {
			maybe_err.abandon(&mut *guard)
		}
	}

	while guard.cur_src() == "implements" {
		let start = guard.cur_tok().range.start;
		let m = guard.start();
		guard.bump_any();
		let elems = ts_heritage_clause(&mut *guard, false);

		let err = guard
			.err_builder("classes cannot have multiple `implements` clauses")
			.primary(start..guard.marker_vec_range(&elems).end, "");

		guard.error(err);
		m.complete(&mut *guard, ERROR);
	}

	if let Some(implements_list) = implements_list {
		implements_list.complete(&mut guard, LIST);
	}

	class_body(&mut *guard);

	m.complete(&mut *guard, kind.into())
}

fn class_body(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T!['{']);
	let elements_list = p.start();

	while !p.at(EOF) && !p.at(T!['}']) {
		match p.cur() {
			// test class_empty_element
			// class foo { ;;;;;;;;;; get foo() {};;;;}
			T![;] => {
				let inner = p.start();
				p.bump_any();
				inner.complete(p, JS_EMPTY_STATEMENT);
			}
			// test static_method
			// class foo {
			//  static foo(bar) {}
			//  static *foo() {}
			//  static async foo() {}
			//  static async *foo() {}
			// }
			_ => {
				class_member_no_semi(p);
			}
		}
	}
	elements_list.complete(p, LIST);

	p.expect(T!['}']);
	m.complete(p, CLASS_BODY)
}

fn class_member_no_semi(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let has_accessibility = matches!(p.cur_src(), "public" | "private" | "protected");
	let mut offset = has_accessibility as usize;
	let declare = p.nth_src(offset) == "declare";
	offset += declare as usize;
	if declare && !has_accessibility {
		if p.nth_at(offset, T![?]) {
			offset += 1;
		}
		// declare() and declare: foo
		if is_method(p, offset) {
			p.bump_any();
			maybe_opt(p);
			args_body(p);
			return Some(m.complete(p, METHOD));
		} else if is_prop(p, offset - 1) {
			p.bump_any();
			let opt = maybe_opt(p);
			return Some(make_prop(p, m, CLASS_PROP, false, false, opt));
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

	if has_accessibility {
		if p.nth_at(offset, T![?]) {
			offset += 1;
		}
		if is_method(p, offset) {
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
			identifier_name(p);
			maybe_opt(p);
			args_body(p);
			return Some(m.complete(p, METHOD));
		} else if is_prop(p, offset - 1) {
			if declare {
				p.bump_remap(T![declare]);
			}
			p.bump_any();
			let opt = maybe_opt(p);
			return Some(make_prop(p, m, CLASS_PROP, false, false, opt));
		}
	}

	let is_static = p.nth_src(offset) == "static";
	if is_static {
		offset += 1;
	};

	if is_static {
		if p.nth_at(offset, T![?]) {
			offset += 1;
		}
		if p.nth_at(offset, T![<]) || p.nth_at(offset, T!['(']) {
			consume_leading_tokens(p, declare, has_accessibility, is_static, true);
			maybe_opt(p);
			args_body(p);
			return Some(m.complete(p, METHOD));
		} else if is_prop(p, offset - 1) {
			consume_leading_tokens(p, declare, has_accessibility, is_static, true);
			let opt = maybe_opt(p);
			return Some(make_prop(p, m, CLASS_PROP, declare, false, opt));
		}
	}
	consume_leading_tokens(p, declare, has_accessibility, is_static, false);

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

	if !is_static && !has_accessibility {
		let check = p.checkpoint();
		if let Some(range) = abstract_range.clone() {
			let err = p
				.err_builder("the `abstract` modifier cannot be used on index signatures")
				.primary(range, "");

			p.error(err);
		}
		if let Some(mut sig) = try_parse_index_signature(p, m.clone()) {
			sig.err_if_not_ts(
				p,
				"class index signatures can only be used in TypeScript files",
			);
			return Some(sig);
		} else {
			p.rewind(check);
		}
	};
	let generator_range = p.cur_tok().range;
	if p.eat(T![*]) {
		let is_constructor = p.cur_src() == "constructor";
		let mut guard = p.with_state(ParserState {
			in_generator: true,
			in_function: true,
			..p.state.clone()
		});
		class_prop_name(&mut *guard);
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
		args_body(&mut *guard);
		return Some(m.complete(&mut *guard, METHOD));
	};

	if p.cur_src() == "async"
		&& !p.nth_at(1, T![?])
		&& !is_method(p, 1)
		&& !p.has_linebreak_before_n(1)
	{
		let async_range = p.cur_tok().range;
		p.bump_remap(T![async]);
		let in_generator = p.eat(T![*]);
		let is_constructor = p.cur_src() == "constructor";
		let mut guard = p.with_state(ParserState {
			in_async: true,
			in_generator,
			in_function: true,
			..p.state.clone()
		});
		class_prop_name(&mut *guard);

		if is_constructor {
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

		args_body(&mut *guard);
		drop(guard);
		return Some(m.complete(p, METHOD));
	}

	let is_constructor = p.cur_src() == "constructor";
	let key = class_prop_name(p);
	let opt = maybe_opt(p);
	if is_method(p, 0) {
		if let Some(range) = readonly_range.clone() {
			let err = p
				.err_builder("class methods cannot be readonly")
				.primary(range, "");

			p.error(err);
		}

		if is_constructor {
			if let Some(range) = opt {
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
			constructor_params(p);
			if let Some(range) = maybe_ts_type_annotation(p) {
				let err = p
					.err_builder("constructors cannot have type annotations")
					.primary(range, "");

				p.error(err);
			}

			function_body_or_declaration(p);

			// FIXME(RDambrosio016): if there is no body we need to issue errors for any assign patterns

			// TODO(RDambrosio016): ideally the following errors should just point to the modifiers
			let complete = m.complete(p, CONSTRUCTOR);
			if is_static {
				let err = p
					.err_builder("constructors cannot be static")
					.primary(complete.range(p), "");

				p.error(err);
			}

			if has_modifier {
				let err = p.err_builder("constructors cannot have modifiers").primary(
					abstract_range.or_else(|| readonly_range.clone()).unwrap(),
					"",
				);

				p.error(err);
			}
			return Some(complete);
		} else {
			args_body(p);
			return Some(m.complete(p, METHOD));
		}
	}
	let kind = if key.map(|(x, _)| x.kind()) == Some(PRIVATE_NAME) {
		PRIVATE_PROP
	} else {
		CLASS_PROP
	};
	let is_constructor =
		kind != CLASS_PROP && key.map(|(x, _)| p.span_text(x.range(p))) == Some("constructor");

	if is_prop(p, 0) && key.is_some() {
		return Some(make_prop(p, m, kind, declare, is_constructor, opt));
	}

	let next_line_generator = p.has_linebreak_before_n(0) && p.at(T![*]);

	if let Some((key, idx)) = key.filter(|(x, _)| x.kind() != PRIVATE_NAME) {
		if matches!(p.span_text(key.range(p)), "get" | "set") && !next_line_generator {
			let getter = p.span_text(key.range(p)) == "get";
			let remap_kind = if getter { T![get] } else { T![set] };
			key.undo_completion(p).abandon(p);
			match p.events[idx.unwrap()] {
				Event::Token { ref mut kind, .. } => *kind = remap_kind,
				_ => unreachable!(),
			};
			class_prop_name(p);

			if let Some(range) = readonly_range {
				let err = p
					.err_builder("getters and setters cannot be readonly")
					.primary(range, "");

				p.error(err);
			}

			args_body(p);
			return Some(m.complete(p, if getter { GETTER } else { SETTER }));
		}
	}

	let err = p
		.err_builder("expected `;`, a property, or a method for a class body, but found none")
		.primary(p.cur_tok().range, "");
	p.err_recover(
		err,
		token_set![T![;], T![ident], T![async], T![yield], T!['}'], T![#]],
		false,
	);
	None
}

fn is_prop(p: &Parser, offset: usize) -> bool {
	(p.at(T![?]) && is_prop(p, offset + 1))
		|| token_set![T![!], T![:], T![=], T!['}']].contains(p.nth(offset))
		|| is_semi(p, offset + 1)
}

pub(super) fn is_semi(p: &Parser, offset: usize) -> bool {
	p.nth_at(offset, T![;])
		|| p.nth_at(offset, EOF)
		|| p.nth_at(offset, T!['}'])
		|| p.has_linebreak_before_n(offset)
}

fn make_prop(
	p: &mut Parser,
	m: Marker,
	kind: SyntaxKind,
	declare: bool,
	constructor: bool,
	opt: Option<Range<usize>>,
) -> CompletedMarker {
	if p.at(T![!]) {
		let range = p.cur_tok().range;
		let mut is_err = false;
		if let Some(opt) = opt {
			let err = p
				.err_builder("class properties cannot be both optional and definite")
				.primary(range.clone(), "")
				.secondary(opt, "");

			p.error(err);
			is_err = true;
		}
		if !p.typescript() {
			let err = p
				.err_builder("definite assignment assertions can only be used in TypeScript files")
				.primary(range, "");

			p.error(err);
			is_err = true;
		}
		if is_err {
			p.bump_remap(ERROR);
		} else {
			p.bump_any();
		}
	}

	maybe_ts_type_annotation(p);
	if p.eat(T![=]) {
		assign_expr(p);
	}
	let start = match p.events[m.old_start as usize] {
		Event::Start { start, .. } => start,
		_ => unreachable!(),
	};

	// inlined stmt::semi
	if !p.eat(T![;]) && !p.at(EOF) && !p.at(T!['}']) && !p.has_linebreak_before_n(0) {
		let err = p
			.err_builder("expected a semicolon for a class property, but found none")
			.primary(start..p.cur_tok().range.start, "");

		p.error(err);
	}
	let complete = m.complete(p, kind);
	if !p.syntax.class_fields {
		let err = p
			.err_builder("class fields are unsupported")
			.primary(complete.range(p), "");

		p.error(err);
	}
	if constructor {
		let err = p
			.err_builder("class properties may not be called `constructor`")
			.primary(complete.range(p), "");

		p.error(err);
	}
	if declare && kind == PRIVATE_PROP {
		let err = p
			.err_builder("private class properties with `declare` are invalid")
			.primary(complete.range(p), "");

		p.error(err);
	}
	complete
}

fn maybe_opt(p: &mut Parser) -> Option<Range<usize>> {
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

fn constructor_params(p: &mut Parser) -> CompletedMarker {
	parameters_common(p, true)
}

pub(super) fn constructor_param_pat(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let has_accessibility = if ts_access_modifier(p).is_some() {
		let range = p.cur_tok().range;
		let maybe_err = p.start();
		consume_leading_tokens(p, false, true, false, false);
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

fn class_prop_name(p: &mut Parser) -> Option<(CompletedMarker, Option<usize>)> {
	if p.at(T![#]) {
		let m = p.start();
		p.bump_any();
		identifier_name(p);
		Some((m.complete(p, PRIVATE_NAME), None))
	} else if let Some(obj) = object_prop_name(p, false) {
		// need to return an index to the token event so class_member can later
		// potentially change it to a get/set keyword
		if obj.kind() == NAME {
			Some((obj, Some(obj.start_pos as usize + 1)))
		} else {
			Some((obj, None))
		}
	} else {
		None
	}
}

fn is_method(p: &Parser, offset: usize) -> bool {
	(p.at(T![?]) && is_method(p, offset + 1))
		|| (p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<]))
}

fn consume_leading_tokens(
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
