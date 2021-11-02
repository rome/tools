//! Class and function declarations.

use super::expr::{assign_expr, identifier_name, object_prop_name, STARTS_EXPR};
use super::pat::{binding_identifier, opt_binding_identifier, pattern};
use super::stmt::block_stmt;
use super::typescript::*;
use crate::{SyntaxKind::*, *};
use std::collections::HashMap;

pub const BASE_METHOD_RECOVERY_SET: TokenSet = token_set![
	T!['['],
	T![ident],
	T![yield],
	T![await],
	T![;],
	T!['}'],
	NUMBER,
	STRING
];

pub fn decorators(p: &mut Parser) -> Vec<CompletedMarker> {
	let mut decorators = Vec::with_capacity(1);
	while p.at(T![@]) {
		let decorator = decorator(p);
		if !p.syntax.decorators {
			let err = p
				.err_builder("decorators are not allowed")
				.primary(decorator.range(p), "");

			p.error(err);
		}
		decorators.push(decorator);
	}
	decorators
}

pub fn decorator(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.expect(T![@]);

	if !STARTS_EXPR.contains(p.cur()) {
		let err = p
			.err_builder("decorators require an expression to call")
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		assign_expr(p);
	}

	m.complete(p, TS_DECORATOR)
}

pub fn maybe_private_name(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T![#]) {
		let m = p.start();
		p.bump_any();
		identifier_name(p);
		Some(m.complete(p, PRIVATE_NAME))
	} else {
		identifier_name(p)
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

fn args_body(p: &mut Parser) {
	if p.at(T![<]) {
		if let Some(ref mut ty) = ts_type_params(p) {
			ty.err_if_not_ts(p, "type parameters can only be used in TypeScript files");
		}
	}
	formal_parameters(p);
	if p.at(T![:]) {
		if let Some(ref mut ty) = ts_type_or_type_predicate_ann(p, T![:]) {
			ty.err_if_not_ts(p, "return types can only be used in TypeScript files");
		}
	}
	fn_body(p);
}

fn fn_body(p: &mut Parser) {
	// omitting the body is allowed in ts
	if p.typescript() && !p.at(T!['{']) && is_semi(p, 0) {
		p.eat(T![;]);
	} else {
		let mut complete = block_stmt(p, true, None);
		if let Some(ref mut block) = complete {
			if p.state.in_declare {
				let err = p
					.err_builder(
						"function implementations cannot be given in ambient (declare) contexts",
					)
					.primary(block.range(p), "");

				p.error(err);
				block.change_kind(p, ERROR);
			}
		}
	}
}

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
pub fn function_decl(p: &mut Parser, m: Marker, fn_expr: bool) -> CompletedMarker {
	// test_err function_decl_err
	// function() {}
	// function {}
	// function *() {}
	// async function() {}
	// async function *() {}
	// function *foo() {}
	// yield foo;
	p.expect(T![function]);
	let in_generator = p.eat(T![*]);
	let guard = &mut *p.with_state(ParserState {
		labels: HashMap::new(),
		in_function: true,
		in_generator,
		..p.state.clone()
	});

	let complete = opt_binding_identifier(guard);
	if complete.is_none() && !fn_expr {
		let err = guard
			.err_builder(
				"expected a name for the function in a function declaration, but found none",
			)
			.primary(guard.cur_tok().range, "");

		guard.error(err);
	}
	args_body(guard);
	m.complete(guard, FN_DECL)
}

#[allow(clippy::unnecessary_unwrap)]
fn formal_param_pat(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	if p.typescript() {
		if let Some(modifier) = maybe_eat_incorrect_modifier(p) {
			let err = p
				.err_builder("modifiers on parameters are only allowed in constructors")
				.primary(modifier.range(p), "");

			p.error(err);
		}
	}

	let pat = pattern(p, true, false)?;
	let pat_range = pat.range(p);
	let mut kind = pat.kind();
	pat.undo_completion(p).abandon(p);

	let mut opt = None;

	if p.at(T![?]) {
		opt = Some(p.cur_tok().range);
		let range = p.cur_tok().range;
		match kind {
			SINGLE_PATTERN | ARRAY_PATTERN | OBJECT_PATTERN => {
				p.bump_any();
			}
			_ if p.state.in_declare => {
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
			}
			_ => {
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
				let err = p
					.err_builder("Binding patterns cannot be optional")
					.primary(pat_range, "");

				p.error(err);
			}
		}
		if !p.typescript() {
			let err = p
				.err_builder(
					"optional parameter syntax with `?` can only be used in TypeScript files",
				)
				.primary(range, "");

			p.error(err);
		}
	}
	maybe_ts_type_annotation(p);
	if p.at(T![=]) {
		let start = p.cur_tok().range.start;
		p.bump_any();

		let expr = assign_expr(p);
		let end = expr
			.map(|x| usize::from(x.range(p).end()))
			.unwrap_or_else(|| p.cur_tok().range.start);
		if let Some(range) = opt {
			let err = p
				.err_builder("optional parameters cannot have initializers")
				.primary(start..end, "")
				.secondary(range, "");

			p.error(err);
		}

		kind = ASSIGN_PATTERN;
	}

	Some(m.complete(p, kind))
}

fn access_modifier<'a>(p: &'a Parser) -> Option<&'a str> {
	if matches!(p.cur_src(), "public" | "private" | "protected") {
		Some(p.cur_src())
	} else {
		None
	}
}

fn constructor_param_pat(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let has_accessibility = if access_modifier(p).is_some() {
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

pub fn formal_parameters(p: &mut Parser) -> CompletedMarker {
	parameters_common(p, false)
}

pub fn constructor_params(p: &mut Parser) -> CompletedMarker {
	parameters_common(p, true)
}

fn parameters_common(p: &mut Parser, constructor_params: bool) -> CompletedMarker {
	let m = p.start();
	let mut first = true;

	p.state.allow_object_expr = p.expect(T!['(']);

	let parameters_list = p.start();

	while !p.at(EOF) && !p.at(T![')']) {
		if first {
			first = false;
		} else if p.nth_at(1, T![')']) {
			p.eat(T![,]);
			break;
		} else {
			p.expect(T![,]);
		}

		let decorator = if p.at(T![@]) {
			decorators(p).into_iter().next()
		} else {
			None
		};

		let marker = if p.at(T![...]) {
			let m = p.start();
			p.bump_any();
			pattern(p, true, false);

			// rest patterns cannot be optional: `...foo?: number[]`
			if p.at(T![?]) {
				let err = p
					.err_builder("rest patterns cannot be optional")
					.primary(p.cur_tok().range, "");

				p.error(err);
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
			}

			// type annotation `...foo: number[]`
			if p.eat(T![:]) {
				let complete = ts_type(p);
				if let Some(mut res) = complete {
					res.err_if_not_ts(p, "type annotations can only be used in TypeScript files");
				}
			}

			if p.at(T![=]) {
				let start = p.cur_tok().range.start;
				let m = p.start();
				p.bump_any();
				let expr = assign_expr(&mut *p);
				let end = expr
					.map(|x| usize::from(x.range(p).end()))
					.unwrap_or_else(|| p.cur_tok().range.start);
				let err = p
					.err_builder("rest elements may not have default initializers")
					.primary(start..end, "");

				p.error(err);
				m.complete(p, ERROR);
			}
			let complete = m.complete(p, REST_PATTERN);

			// FIXME: this should be handled better, we should keep trying to parse params but issue an error for each one
			// which would allow for better recovery from `foo, ...bar, foo`
			if p.at(T![,]) {
				let m = p.start();
				let range = p.cur_tok().range;
				p.bump_any();
				m.complete(p, ERROR);
				let err = p
					.err_builder("rest elements may not have trailing commas")
					.primary(range, "");

				p.error(err);
			}
			Some(complete)
		} else {
			let func = if constructor_params {
				constructor_param_pat
			} else {
				formal_param_pat
			};
			// test_err formal_params_no_binding_element
			// function foo(true) {}
			if let Some(res) = func(p) {
				if res.kind() == ASSIGN_PATTERN && p.state.in_binding_list_for_signature {
					let err = p
						.err_builder(
							"assignment patterns cannot be used in function/constructor types",
						)
						.primary(res.range(p), "");

					p.error(err);
				}
				Some(res)
			} else {
				p.err_recover_no_err(
					token_set![
						T![ident],
						T![await],
						T![yield],
						T![,],
						T!['['],
						T![...],
						T![')'],
					],
					true,
				);
				None
			}
		};

		if let (Some(res), Some(decorator)) = (marker, decorator) {
			let kind = res.kind();
			let m = decorator.precede(p);
			res.undo_completion(p).abandon(p);
			m.complete(p, kind);
		}
	}

	parameters_list.complete(p, LIST);
	p.state.allow_object_expr = true;
	p.expect(T![')']);
	m.complete(p, PARAMETER_LIST)
}

pub fn arrow_body(p: &mut Parser) -> Option<CompletedMarker> {
	let mut guard = p.with_state(ParserState {
		in_function: true,
		..p.state.clone()
	});
	if guard.at(T!['{']) {
		block_stmt(&mut *guard, true, None)
	} else {
		assign_expr(&mut *guard)
	}
}

// test class_decl
// class foo {}
// class foo extends bar {}
// class foo extends foo.bar {}
pub fn class_decl(p: &mut Parser, expr: bool) -> CompletedMarker {
	// test_err class_decl_err
	// class {}
	// class extends bar {}
	// class extends {}
	// class
	// class foo { set {} }
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

	if idt.is_none() && !expr && !guard.state.in_default {
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

	m.complete(&mut *guard, CLASS_DECL)
}

pub(crate) fn class_body(p: &mut Parser) -> CompletedMarker {
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
				inner.complete(p, EMPTY_STMT);
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

fn is_method(p: &Parser, offset: usize) -> bool {
	(p.at(T![?]) && is_method(p, offset + 1))
		|| (p.nth_at(offset, T!['(']) || p.nth_at(offset, T![<]))
}

pub(crate) fn is_semi(p: &Parser, offset: usize) -> bool {
	p.nth_at(offset, T![;])
		|| p.nth_at(offset, EOF)
		|| p.nth_at(offset, T!['}'])
		|| p.has_linebreak_before_n(offset)
}

fn is_prop(p: &Parser, offset: usize) -> bool {
	(p.at(T![?]) && is_prop(p, offset + 1))
		|| token_set![T![!], T![:], T![=], T!['}']].contains(p.nth(offset))
		|| is_semi(p, offset + 1)
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

fn class_member_no_semi(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	decorators(p);
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

			fn_body(p);

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

/// A method definition, this takes an optional markers for object props
pub fn method(
	p: &mut Parser,
	marker: impl Into<Option<Marker>>,
	recovery_set: impl Into<Option<TokenSet>>,
) -> Option<CompletedMarker> {
	let m = marker.into().unwrap_or_else(|| p.start());
	let old = p.state.to_owned();
	p.state.in_function = true;
	let complete = match p.cur() {
		T!['('] | T![<] => {
			args_body(p);
			m.complete(p, METHOD)
		}

		// test method_getter
		// class foo {
		//  get bar() {}
		// }

		// test_err method_getter_err
		// class foo {
		//  get {}
		// }
		T![ident] if p.cur_src() == "get" && p.nth(1) != T!['('] => {
			p.bump_remap(T![get]);
			object_prop_name(p, false);
			args_body(p);
			m.complete(p, GETTER)
		}
		// test method_setter
		// class foo {
		//  set bar() {}
		// }
		T![ident] if p.cur_src() == "set" && p.nth(1) != T!['('] => {
			p.bump_remap(T![set]);
			object_prop_name(p, false);
			args_body(p);
			m.complete(p, SETTER)
		}
		// test async_method
		// class foo {
		//  async foo() {}
		//  async *foo() {}
		// }
		T![ident] if p.cur_src() == "async" && !p.has_linebreak_before_n(1) => {
			p.bump_remap(T![async]);
			let in_generator = p.eat(T![*]);
			let mut guard = p.with_state(ParserState {
				in_async: true,
				in_generator,
				..p.state.clone()
			});
			object_prop_name(&mut *guard, false);
			args_body(&mut *guard);
			m.complete(&mut *guard, METHOD)
		}
		T![*] | STRING | NUMBER | T![await] | T![ident] | T![yield] | T!['['] => {
			let in_generator = p.eat(T![*]);
			let mut guard = p.with_state(ParserState {
				in_generator,
				..p.state.clone()
			});
			object_prop_name(&mut *guard, false);
			args_body(&mut *guard);
			drop(guard);
			m.complete(p, METHOD)
		}
		t if t.is_keyword() => {
			let in_generator = p.eat(T![*]);
			let mut guard = p.with_state(ParserState {
				in_generator,
				..p.state.clone()
			});
			object_prop_name(&mut *guard, false);
			args_body(&mut *guard);
			drop(guard);
			m.complete(p, METHOD)
		}
		_ => {
			let err = p
				.err_builder("expected a method definition, but found none")
				.primary(p.cur_tok().range, "");

			p.err_recover(
				err,
				recovery_set.into().unwrap_or(BASE_METHOD_RECOVERY_SET),
				false,
			);
			return None;
		}
	};
	p.state = old;
	Some(complete)
}
