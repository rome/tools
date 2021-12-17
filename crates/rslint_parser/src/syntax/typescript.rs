//! TypeScript specific functions.

use super::decl::*;
use super::expr::{
	parse_expr_or_assignment, parse_identifier_name, parse_lhs_expr, parse_literal_expression,
};
use super::stmt::{parse_statements, semi, variable_declaration_statement};
use crate::parser::ParserProgress;
#[allow(deprecated)]
use crate::parser::SingleTokenParseRecovery;
use crate::syntax::binding::parse_binding;
use crate::syntax::class::parse_class_declaration;
use crate::syntax::expr::parse_any_name;
use crate::syntax::function::parse_function_declaration;
use crate::syntax::js_parse_error;
use crate::{JsSyntaxKind::*, *};

pub const BASE_TS_RECOVERY_SET: TokenSet = token_set![
	T![void],
	T![ident],
	T![ident],
	T![await],
	T![null],
	T![break],
	T!['['],
];

#[rustfmt::skip]
pub const DISALLOWED_TYPE_NAMES: &[&str] = &[
    "string",
    "null",
    "number",
    "object",
    "any",
    "unknown",
    "boolean",
    "bigint",
    "symbol",
    "void",
    "never",
];

// ambiguity is fun!
macro_rules! no_recover {
	($p:expr, $res:expr) => {
		if $res.is_none() && $p.state.no_recovery {
			return None;
		}
	};
	($p:expr, $m:expr, $t:expr, $res:expr) => {
		if $res.is_none() && $p.state.no_recovery {
			$m.abandon($p);
			$p.rewind($t);
			return None;
		}
	};
}

pub fn ts_modifier(p: &mut Parser, modifiers: &[&'static str]) -> Option<Range<usize>> {
	if !modifiers.contains(&p.cur_src()) {
		return None;
	}

	let range = p.cur_tok().range;

	if p.has_linebreak_before_n(1)
		|| token_set![T!['('], T![')'], T![:], T![=], T![?]].contains(p.nth(1))
	{
		return None;
	}

	let kind = match p.cur_src() {
		"abstract" => T![abstract],
		"readonly" => T![readonly],
		_ => unreachable!("unknown modifier"),
	};
	p.bump_remap(kind);

	Some(range)
}

pub(crate) fn maybe_ts_type_annotation(p: &mut Parser) -> Option<Range<usize>> {
	if p.at(T![:]) {
		let m = p.start();

		let start = p.cur_tok().range.start;
		p.bump_any();
		let compl = ts_type(p);

		let end = compl
			.map(|x| usize::from(x.range(p).end()))
			.unwrap_or_else(|| p.cur_tok().range.start);

		if !p.typescript() {
			let err = p
				.err_builder("type annotations can only be used in TypeScript files")
				.primary(start..end, "");

			p.error(err);
			m.complete(p, ERROR);
		} else {
			m.complete(p, TS_TYPE_ANNOTATION);
		}
		Some(start..end)
	} else {
		None
	}
}

pub(crate) fn ts_expr_stmt(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur_src() {
		"declare" => ts_declare(p),
		"global" => {
			if p.nth_at(1, T!['{']) {
				ts_ambient_external_module_decl(p, false)
			} else {
				None
			}
		}
		_ => ts_decl(p),
	}
}

pub(crate) fn ts_declare(p: &mut Parser) -> Option<CompletedMarker> {
	debug_assert_eq!(p.cur_src(), "declare");
	let p = &mut *p.with_state(ParserState {
		in_declare: true,
		..p.state.clone()
	});
	Some(match p.nth(1) {
		T![function] => parse_function_declaration(p).unwrap(),
		T![class] => {
			let m = p.start();
			p.bump_remap(T![declare]);
			parse_class_declaration(p)
				.unwrap()
				.undo_completion(p)
				.abandon(p);
			m.complete(p, JS_LITERAL_MEMBER_NAME)
		}
		t if (t == T![const] && p.nth_at(2, T![enum])) || t == T![enum] => {
			let m = p.start();
			p.bump_remap(T![declare]);
			ts_enum(p).undo_completion(p).abandon(p);
			m.complete(p, TS_ENUM)
		}
		T![const] | T![var] => {
			let m = p.start();
			p.bump_remap(T![declare]);
			// unwrap the marker so its children go to `m`
			variable_declaration_statement(p)
				.unwrap()
				.undo_completion(p)
				.abandon(p);
			m.complete(p, JS_VARIABLE_STATEMENT)
		}
		_ if p.nth_src(1) == "let" => {
			let m = p.start();
			p.bump_remap(T![declare]);
			variable_declaration_statement(p)
				.unwrap()
				.undo_completion(p)
				.abandon(p);
			m.complete(p, JS_VARIABLE_STATEMENT)
		}
		_ if p.nth_src(1) == "global" => {
			let m = p.start();
			p.bump_remap(T![declare]);
			if let Some(complete) = ts_ambient_external_module_decl(p, false) {
				complete.undo_completion(p).abandon(p);
			}
			m.complete(p, TS_MODULE_DECL)
		}
		_ => {
			let checkpoint = p.checkpoint();
			let m = p.start();
			p.bump_remap(T![declare]);
			let res = ts_decl(p);
			return if let Some(res) = res {
				let kind = res.kind();
				res.undo_completion(p).abandon(p);
				Some(m.complete(p, kind))
			} else {
				m.abandon(p);
				p.rewind(checkpoint);
				None
			};
		}
	})
}

pub(crate) fn ts_decl(p: &mut Parser) -> Option<CompletedMarker> {
	if p.cur_src() == "abstract" {
		let m = p.start();
		let range = p.cur_tok().range;
		p.bump_remap(T![abstract]);
		if !p.at(T![class]) {
			let err = p.err_builder("abstract modifiers can only be applied to classes, methods, or property definitions")
                .primary(range, "");

			p.error(err);
			m.abandon(p);
			return None;
		}
		parse_class_declaration(p)
			.unwrap()
			.undo_completion(p)
			.abandon(p);
		return Some(m.complete(p, JS_LITERAL_MEMBER_NAME));
	}

	if p.at(T![enum]) {
		return Some(ts_enum(p));
	}

	if p.cur_src() == "interface" {
		return ts_interface(p);
	}

	if p.cur_src() == "module" {
		if p.nth_at(1, JS_STRING_LITERAL) {
			return ts_ambient_external_module_decl(p, true);
		} else if token_set![T![ident], T![yield], T![await]].contains(p.nth(1)) {
			p.bump_remap(T![module]);
			return ts_module_or_namespace_decl(p, false, false);
		}
	}

	if p.cur_src() == "namespace" {
		let m = p.start();
		p.bump_any();
		ts_module_or_namespace_decl(p, true, false)?
			.undo_completion(p)
			.abandon(p);
		return Some(m.complete(p, TS_NAMESPACE_DECL));
	}

	if p.cur_src() == "type" {
		return ts_type_alias_decl(p);
	}

	None
}

pub fn ts_type_alias_decl(p: &mut Parser) -> Option<CompletedMarker> {
	let t = p.checkpoint();

	let m = p.start();
	let start = p.cur_tok().range.start;
	p.bump_any();
	let identifier =
		parse_identifier_name(p).or_syntax_error(p, js_parse_error::expected_identifier);

	if let Some(marker) = identifier {
		no_recover!(p, m, t, Some(marker));
	};
	if p.at(T![<]) {
		no_recover!(p, ts_type_params(p));
	}

	let end = p.cur_tok().range.end;

	if p.expect_no_recover(T![=]).is_none() {
		m.abandon(p);
		p.rewind(t);
		return None;
	}

	no_recover!(p, m, t, ts_type(p));

	semi(p, start..end);
	Some(m.complete(p, TS_TYPE_ALIAS_DECL))
}

pub(crate) fn ts_module_or_namespace_decl(
	p: &mut Parser,
	namespace: bool,
	eat_dot: bool,
) -> Option<CompletedMarker> {
	let m = p.start();
	if eat_dot {
		p.eat(T![.]);
	}

	if parse_identifier_name(p).is_absent() && p.state.no_recovery {
		return None;
	}

	if p.at(T![.]) {
		ts_module_or_namespace_decl(p, namespace, true)?;
	} else {
		ts_module_block(p);
	}

	Some(m.complete(
		p,
		if namespace {
			TS_NAMESPACE_DECL
		} else {
			TS_MODULE_DECL
		},
	))
}

pub fn ts_ambient_external_module_decl(
	p: &mut Parser,
	check_for_module: bool,
) -> Option<CompletedMarker> {
	let m = p.start();
	let start = p.cur_tok().range.start;
	if check_for_module && p.cur_src() != "module" {
		let err = p
			.err_builder(&format!(
				"expected keyword `module`, but instead found `{}`",
				p.cur_src()
			))
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else if check_for_module {
		p.bump_remap(T![module]);
	}

	let end = p.cur_tok().range.end;
	if p.cur_src() == "global" {
		p.bump_any();
	} else {
		p.expect_required(JS_STRING_LITERAL);
	}
	if p.at(T!['{']) {
		ts_module_block(p);
	} else {
		semi(p, start..end);
	}
	Some(m.complete(p, TS_MODULE_DECL))
}

pub fn ts_module_block(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	p.expect_no_recover(T!['{'])?;
	// module blocks are considered top level
	parse_statements(p, true);
	p.expect_no_recover(T!['}'])?;
	Some(m.complete(p, TS_MODULE_BLOCK))
}

pub fn ts_interface(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	if p.cur_src() != "interface" {
		let err = p
			.err_builder(&format!(
				"expected keyword `interface`, but instead found `{}`",
				p.cur_src()
			))
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		p.bump_any();
	}

	if DISALLOWED_TYPE_NAMES.contains(&p.cur_src()) || p.cur_src() == "intrinsic" {
		let err = p
			.err_builder(&format!(
				"`{}` cannot be used as the name of an interface",
				p.cur_src()
			))
			.primary(p.cur_tok().range, "")
			.footer_note(format!("`{}` is already reserved as a type", p.cur_src()));

		p.error(err);
	}
	parse_identifier_name(p).or_syntax_error(p, js_parse_error::expected_identifier);
	if p.at(T![<]) {
		ts_type_params(p);
	}

	if p.cur_src() == "extends" {
		p.bump_any();
		ts_heritage_clause(p, false);
	};

	let mut progress = ParserProgress::default();
	while p.cur_src() == "extends" {
		progress.assert_progressing(p);

		let m = p.start();
		p.bump_any();
		let mut complete = ts_heritage_clause(p, false);
		for elem in &mut complete {
			elem.change_kind(p, ERROR);
		}

		let err = p
			.err_builder("interfaces cannot contain multiple `extends` clauses")
			.primary(p.marker_vec_range(&complete), "");

		p.error(err);
		m.complete(p, ERROR);
	}

	p.expect_required(T!['{']);

	let members_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) && !p.at(T!['}']) {
		progress.assert_progressing(p);
		ts_type_member(p);
	}
	members_list.complete(p, TS_OBJECT_MEMBER_LIST);

	p.expect_required(T!['}']);
	Some(m.complete(p, TS_INTERFACE_DECL))
}

// FIXME: ts allows trailing commas but this doesnt, we need to figure out a way
// to peek at the next token and see if its the end of the heritage clause
pub(crate) fn ts_heritage_clause(p: &mut Parser, exprs: bool) -> Vec<CompletedMarker> {
	let mut elems = Vec::with_capacity(1);
	let m = p.start();
	if exprs {
		parse_lhs_expr(p).or_syntax_error(p, js_parse_error::expected_expression);
	} else {
		ts_entity_name(p, None, false);
	}

	if p.at(T![<]) {
		ts_type_args(p);
	}

	// it doesnt matter if we complete as ts_expr_with_type_args even if its an lhs expr
	// because exprs: true will only be used with `class extends foo, bar`, in which case
	// the first expr will be "unwrapped" to go to the class' node and the rest are errors
	elems.push(m.complete(p, TS_EXPR_WITH_TYPE_ARGS));

	let mut progress = ParserProgress::default();
	while p.eat(T![,]) {
		progress.assert_progressing(p);
		let m = p.start();
		if exprs {
			parse_lhs_expr(p).or_syntax_error(p, js_parse_error::expected_expression);
		} else {
			ts_entity_name(p, None, false);
		}
		if p.at(T![<]) {
			ts_type_args(p);
		}

		elems.push(m.complete(p, TS_EXPR_WITH_TYPE_ARGS));
	}
	elems
}

pub fn ts_type_member(p: &mut Parser) -> Option<CompletedMarker> {
	if p.at(T!['(']) || p.at(T![<]) {
		return ts_signature_member(p, false);
	}

	if p.at(T![new]) && token_set!(T![<], T!['(']).contains(p.nth(1)) {
		return ts_signature_member(p, true);
	}

	let (m, readonly) = if p.cur_src() == "readonly" {
		let m = p.start();
		p.bump_remap(T![readonly]);
		(m, true)
	} else {
		(p.start(), false)
	};

	match try_parse_index_signature(p, m) {
		Ok(idx) => Some(idx),
		Err(m) => ts_property_or_method_sig(p, m, readonly),
	}
}

fn ts_property_or_method_sig(p: &mut Parser, m: Marker, readonly: bool) -> Option<CompletedMarker> {
	if p.eat(T!['[']) {
		parse_expr_or_assignment(p)
			.or_syntax_error(p, js_parse_error::expected_expression_assignment);
		p.expect_no_recover(T![']'])?;
	} else {
		match p.cur() {
			JS_STRING_LITERAL | JS_NUMBER_LITERAL => {
				parse_literal_expression(p).ok();
			}
			_ => {
				let mut complete = parse_any_name(p).ok()?;
				if complete.kind() == JS_PRIVATE_CLASS_MEMBER_NAME {
					let err = p
						.err_builder("private names are not allowed outside of class bodies")
						.primary(complete.range(p), "");

					p.error(err);
					complete.change_kind(p, ERROR);
				}
			}
		}
	};

	p.eat(T![?]);
	Some(if !readonly && p.at_ts(token_set![T!['('], T![<]]) {
		if p.at(T![<]) {
			no_recover!(p, ts_type_params(p));
		}
		parse_parameter_list(p).or_syntax_error(p, js_parse_error::expected_parameters);
		if p.at(T![:]) {
			ts_type_or_type_predicate_ann(p, T![:]);
		}
		type_member_semi(p);
		m.complete(p, TS_METHOD_SIGNATURE)
	} else {
		if p.eat(T![:]) {
			ts_type(p);
		}
		type_member_semi(p);
		m.complete(p, TS_PROPERTY_SIGNATURE)
	})
}

pub(crate) fn try_parse_index_signature(
	p: &mut Parser,
	m: Marker,
) -> Result<CompletedMarker, Marker> {
	if !p.at(T!['['])
		|| !(token_set![T![ident], T![await], T![yield]].contains(p.nth(1))
			|| p.nth(1).is_keyword())
		|| !token_set![T![:], T![,]].contains(p.nth(2))
	{
		return Err(m);
	}

	if p.expect_no_recover(T!['[']).is_none() {
		return Err(m);
	}

	let pat_m = parse_binding(p).unwrap().undo_completion(p);

	if p.expect_no_recover(T![:]).is_none() {
		return Err(m);
	}

	if ts_type(p).is_none() && p.state.no_recovery {
		return Err(m);
	}

	pat_m.complete(p, JS_IDENTIFIER_BINDING);

	if p.expect_no_recover(T![']']).is_none() {
		return Err(m);
	}

	if p.eat(T![:]) && ts_type(p).is_none() && p.state.no_recovery {
		return Err(m);
	}

	type_member_semi(p);
	Ok(m.complete(p, TS_INDEX_SIGNATURE))
}

pub fn ts_signature_member(p: &mut Parser, construct_sig: bool) -> Option<CompletedMarker> {
	let m = p.start();
	if construct_sig {
		p.expect_required(T![new]);
	}

	if p.at(T![<]) {
		no_recover!(p, ts_type_params(p));
	}

	{
		let guard = &mut *p.with_state(ParserState {
			in_binding_list_for_signature: true,
			..p.state.clone()
		});
		parse_parameter_list(guard).ok();
	}
	if p.at(T![:]) {
		no_recover!(p, ts_type_or_type_predicate_ann(p, T![:]));
	}
	type_member_semi(p);

	Some(m.complete(
		p,
		if construct_sig {
			TS_CONSTRUCT_SIGNATURE_DECL
		} else {
			TS_CALL_SIGNATURE_DECL
		},
	))
}

// TODO(RDambrosio016): is this logic correct?
fn type_member_semi(p: &mut Parser) {
	if p.at_ts(token_set![T![,], T![;]]) {
		p.bump_any();
	}
}

pub fn ts_enum(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.eat(T![const]);
	p.expect_required(T![enum]);
	parse_identifier_name(p).or_syntax_error(p, js_parse_error::expected_identifier);
	p.expect_required(T!['{']);
	let mut first = true;

	let members_list = p.start();
	let mut progress = ParserProgress::default();
	while !p.at(EOF) && !p.at(T!['}']) {
		progress.assert_progressing(p);
		if first {
			first = false;
		} else if p.at(T![,]) && p.nth_at(1, T!['}']) {
			p.eat(T![,]);
			break;
		} else {
			p.expect_required(T![,]);
		}

		let member = p.start();
		let err_occured = if !p.at_ts(token_set![T![ident], T![yield], T![await]])
			&& !p.cur().is_keyword()
			&& !p.at(JS_STRING_LITERAL)
		{
			let err = p
				.err_builder("expected an identifier or string for an enum variant, but found none")
				.primary(p.cur_tok().range, "");

			#[allow(deprecated)]
			SingleTokenParseRecovery::with_error(
				token_set![T!['}'], T![ident], T![yield], T![await], T![=], T![,]],
				ERROR,
				err,
			)
			.recover(p);
			true
		} else {
			if !p.eat(JS_STRING_LITERAL) {
				parse_identifier_name(p)
					.unwrap()
					.undo_completion(p)
					.abandon(p);
			}
			false
		};

		if p.eat(T![=]) {
			parse_expr_or_assignment(p)
				.or_syntax_error(p, js_parse_error::expected_expression_assignment);
			member.complete(p, TS_ENUM_MEMBER);
		} else if err_occured {
			member.abandon(p);
		} else {
			member.complete(p, TS_ENUM_MEMBER);
		}
	}

	members_list.complete(p, TS_ENUM_MEMBER_LIST);

	p.expect_required(T!['}']);
	m.complete(p, TS_ENUM)
}

pub fn try_parse_ts(
	p: &mut Parser,
	func: impl FnOnce(&mut Parser) -> Option<CompletedMarker>,
) -> Option<CompletedMarker> {
	let checkpoint = p.checkpoint();

	let res = if p.state.no_recovery {
		func(p)
	} else {
		func(&mut *p.with_state(ParserState {
			no_recovery: true,
			..p.state.clone()
		}))
	};

	if res.is_none() {
		p.rewind(checkpoint);
	}
	res
}

pub fn ts_type(p: &mut Parser) -> Option<CompletedMarker> {
	let ty = ts_non_conditional_type(p);
	if p.has_linebreak_before_n(0) || !p.at(T![extends]) {
		return ty;
	}

	let m = ty.map(|x| x.precede(p)).unwrap_or_else(|| p.start());
	if p.at(T![extends]) {
		let m = p.start();
		p.bump_any();
		no_recover!(p, ts_non_conditional_type(p));
		m.complete(p, TS_EXTENDS);
	}
	p.expect_no_recover(T![?])?;
	no_recover!(p, ts_type(p));
	p.expect_no_recover(T![:])?;
	no_recover!(p, ts_type(p));
	Some(m.complete(p, TS_CONDITIONAL_TYPE))
}

pub fn ts_fn_or_constructor_type(p: &mut Parser, fn_type: bool) -> Option<CompletedMarker> {
	let m = p.start();
	if !fn_type && p.expect_no_recover(T![new]).is_none() {
		m.abandon(p);
		return None;
	}

	if p.at(T![<]) {
		ts_type_params(p);
	}
	parse_parameter_list(p).or_syntax_error(p, js_parse_error::expected_parameters);
	if ts_type_or_type_predicate_ann(p, T![=>]).is_none() && p.state.no_recovery {
		m.abandon(p);
		return None;
	}

	Some(m.complete(
		p,
		if fn_type {
			TS_FN_TYPE
		} else {
			TS_CONSTRUCTOR_TYPE
		},
	))
}

pub(crate) fn ts_type_or_type_predicate_ann(
	p: &mut Parser,
	return_token: JsSyntaxKind,
) -> Option<CompletedMarker> {
	let ident_ref_set = token_set![T![await], T![yield], T![ident]];
	p.expect_no_recover(return_token)?;

	let type_pred = (p.cur_src() == "asserts" && ident_ref_set.contains(p.nth(1)))
		|| (p.at_ts(ident_ref_set) && p.nth_src(1) == "is" && !p.has_linebreak_before_n(1));

	if type_pred {
		ts_predicate(p)
	} else {
		ts_type(p)
	}
}

pub fn ts_non_conditional_type(p: &mut Parser) -> Option<CompletedMarker> {
	if is_start_of_fn_type(p) {
		return ts_fn_or_constructor_type(p, true);
	}

	if p.at(T![new]) {
		return ts_fn_or_constructor_type(p, false);
	}

	intersection_or_union(p, false, ts_intersection_type_or_higher, T![|])
}

fn ts_intersection_type_or_higher(p: &mut Parser) -> Option<CompletedMarker> {
	intersection_or_union(p, true, ts_type_operator_or_higher, T![&])
}

fn look_ahead(p: &mut Parser, func: impl FnOnce(&mut Parser) -> bool) -> bool {
	let checkpoint = p.checkpoint();
	let res = func(p);
	p.rewind(checkpoint);
	res
}

fn is_start_of_fn_type(p: &mut Parser) -> bool {
	p.at(T![<]) || (p.at(T!['(']) && look_ahead(p, is_unambiguously_start_of_fn_type))
}

fn is_unambiguously_start_of_fn_type(p: &mut Parser) -> bool {
	p.eat(T!['(']);
	if p.at(T![')']) || p.at(T![...]) {
		return true;
	}

	if skip_parameter_start(p) {
		if p.at_ts(token_set![T![:], T![,], T![?], T![=]]) {
			return true;
		}
		if p.at(T![')']) && p.nth_at(1, T![=>]) {
			return true;
		}
	}
	false
}

fn skip_parameter_start(p: &mut Parser) -> bool {
	maybe_eat_incorrect_modifier(p);
	if p.at_ts(token_set![T![this], T![yield], T![ident], T![await]]) {
		p.bump_any();
		return true;
	}

	if p.eat(T!['{']) {
		let mut counter = 1;

		while counter > 0 {
			if p.eat(T!['{']) {
				counter += 1;
			} else if p.eat(T!['}']) {
				counter -= 1;
			} else {
				p.bump_any();
			}
		}
		return true;
	}

	if p.eat(T!['[']) {
		let mut counter = 1;

		while counter > 0 {
			if p.eat(T!['[']) {
				counter += 1;
			} else if p.eat(T![']']) {
				counter -= 1;
			} else {
				p.bump_any();
			}
		}
		return true;
	}
	false
}

fn intersection_or_union(
	p: &mut Parser,
	intersection: bool,
	mut constituent: impl FnMut(&mut Parser) -> Option<CompletedMarker>,
	op: JsSyntaxKind,
) -> Option<CompletedMarker> {
	let kind = if intersection {
		TS_INTERSECTION
	} else {
		TS_UNION
	};
	let m = p.start();
	let types_list = p.start();
	let saw_op = p.eat(op);
	let ty = constituent(p);
	if p.at(op) {
		while p.eat(op) {
			constituent(p);
		}

		types_list.complete(p, TS_TYPE_LIST);
		Some(m.complete(p, kind))
	} else if !saw_op && ty.is_none() {
		types_list.abandon(p);
		m.abandon(p);
		None
	} else if !saw_op {
		types_list.abandon(p);
		m.abandon(p);
		ty
	} else {
		types_list.complete(p, TS_TYPE_LIST);
		Some(m.complete(p, kind))
	}
}

pub fn ts_type_operator_or_higher(p: &mut Parser) -> Option<CompletedMarker> {
	if matches!(p.cur_src(), "keyof" | "unique" | "readonly") {
		let m = p.start();
		let kind = match p.cur_src() {
			"keyof" => KEYOF_KW,
			"unique" => UNIQUE_KW,
			"readonly" => READONLY_KW,
			_ => unreachable!(),
		};
		p.bump_remap(kind);
		no_recover!(p, ts_type_operator_or_higher(p));
		Some(m.complete(p, TS_TYPE_OPERATOR))
	} else if p.cur_src() == "infer" {
		let m = p.start();
		p.bump_remap(T![infer]);
		parse_identifier_name(p).or_syntax_error(p, js_parse_error::expected_identifier);
		Some(m.complete(p, TS_INFER))
	} else {
		// FIXME: readonly should apparently be handled here?
		// but the previous matches should have accounted for it 🤔
		ts_array_type_or_higher(p)
	}
}

pub fn ts_array_type_or_higher(p: &mut Parser) -> Option<CompletedMarker> {
	let t = p.checkpoint();

	let mut ty = ts_non_array_type(p);
	let mut progress = ParserProgress::default();

	while !p.has_linebreak_before_n(0) && p.at(T!['[']) {
		progress.assert_progressing(p);
		let m = ty.map(|x| x.precede(p)).unwrap_or_else(|| p.start());
		p.bump_any();
		if p.eat(T![']']) {
			ty = Some(m.complete(p, TS_ARRAY));
		} else {
			no_recover!(p, m, t, ts_type(p));
			if p.expect_no_recover(T![']']).is_none() {
				m.abandon(p);
				p.rewind(t);
				return None;
			}
			ty = Some(m.complete(p, TS_INDEXED_ARRAY));
		}
	}
	ty
}

pub fn ts_tuple(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	p.expect_no_recover(T!['['])?;
	let mut progress = ParserProgress::default();
	while !p.at(EOF) && !p.at(T![']']) {
		progress.assert_progressing(p);
		let m = p.start();
		let rest_range = p.cur_tok().range;
		let rest = p.eat(T![...]);
		let name = if crate::at_ident_name!(p)
			&& !DISALLOWED_TYPE_NAMES.contains(&p.cur_src())
			&& (p.nth_at(1, T![:]) || (p.nth_at(1, T![?]) && p.nth_at(2, T![:])))
		{
			parse_identifier_name(p).or_syntax_error(p, js_parse_error::expected_identifier);
			true
		} else {
			false
		};

		let opt_range = p.cur_tok().range;
		let is_opt = name && p.eat(T![?]);
		if name {
			p.expect_required(T![:]);
		}
		no_recover!(p, ts_type(p));
		if !name && p.at(T![?]) {
			p.eat(T![?]);
		}
		m.complete(p, TS_TUPLE_ELEMENT);
		if is_opt && rest {
			let err = p
				.err_builder("a tuple element cannot be both rest and optional")
				.secondary(rest_range, "")
				.primary(opt_range, "");

			p.error(err);
		}
		p.eat(T![,]);
	}

	p.expect_no_recover(T![']'])?;
	Some(m.complete(p, TS_TUPLE))
}

pub fn ts_non_array_type(p: &mut Parser) -> Option<CompletedMarker> {
	match p.cur() {
		T![ident] | T![void] | T![yield] | T![null] | T![await] | T![break] => {
			if p.cur_src() == "asserts" && p.nth_at(1, T![this]) {
				p.bump_any();
				return ts_predicate(p);
			}

			let kind = match p.cur_src() {
				"void" => TS_VOID,
				"null" => TS_NULL,
				"any" => TS_ANY,
				"boolean" => TS_BOOLEAN,
				"bigint" => TS_BIGINT,
				"never" => TS_NEVER,
				"number" => TS_NUMBER,
				"object" => TS_OBJECT,
				"string" => TS_STRING,
				"symbol" => TS_SYMBOL,
				"unknown" => TS_UNKNOWN,
				"undefined" => TS_UNDEFINED,
				_ =>
				/* dummy value */
				{
					ERROR
				}
			};

			if kind != ERROR && !p.nth_at(1, T![.]) {
				let m = p.start();
				p.bump_any();
				Some(m.complete(p, kind))
			} else {
				ts_type_ref(p, None)
			}
		}
		JS_NUMBER_LITERAL | JS_STRING_LITERAL | TRUE_KW | FALSE_KW | JS_REGEX_LITERAL => Some(
			parse_literal_expression(p)
				.precede(p)
				.complete(p, TS_LITERAL),
		),
		BACKTICK => {
			let m = p.start();
			p.bump_any();

			let elements_list = p.start();
			while !p.at(EOF) && !p.at(BACKTICK) {
				match p.cur() {
                    TEMPLATE_CHUNK => {
											let m = p.start();
											p.bump_any();
											m.complete(p, TEMPLATE_CHUNK_ELEMENT);
										},
										DOLLAR_CURLY => {
                        let e = p.start();
                        p.bump_any();
                        ts_type(p);
                        p.expect_required(T!['}']);
                        e.complete(p, TS_TEMPLATE_ELEMENT);
                    },
                    t => unreachable!("Anything not template chunk or dollar_curly should have been eaten by the lexer, but {:?} was found", t),
                }
			}

			elements_list.complete(p, TEMPLATE_ELEMENT_LIST);
			p.eat(BACKTICK);
			Some(m.complete(p, TS_TEMPLATE))
		}
		T![-] => {
			let t = p.checkpoint();
			let m = p.start();
			p.bump_any();
			if p.at(JS_NUMBER_LITERAL) {
				let _m = p.start();
				p.bump_any();
				_m.complete(p, JS_NUMBER_LITERAL_EXPRESSION);
			} else if p.expect_no_recover(JS_NUMBER_LITERAL).is_none() {
				m.abandon(p);
				p.rewind(t);
				return None;
			}
			Some(m.complete(p, TS_LITERAL))
		}
		T![import] => ts_import(p),
		T![this] => {
			if p.nth_src(1) == "is" {
				ts_predicate(p)
			} else {
				let m = p.start();
				p.bump_any();
				Some(m.complete(p, TS_THIS))
			}
		}
		T![typeof] => ts_type_query(p),
		T!['{'] => {
			if is_mapped_type_start(p) {
				ts_mapped_type(p)
			} else {
				let m = p.start();
				p.bump_any();
				let members_list = p.start();
				let mut progress = ParserProgress::default();
				while !p.at(EOF) && !p.at(T!['}']) {
					progress.assert_progressing(p);
					ts_type_member(p);
					type_member_semi(p);
				}
				members_list.complete(p, TS_OBJECT_MEMBER_LIST);
				p.expect_required(T!['}']);
				Some(m.complete(p, TS_OBJECT_TYPE))
			}
		}
		T!['['] => ts_tuple(p),
		T!['('] => {
			let t = p.checkpoint();
			let m = p.start();
			p.bump_any();
			no_recover!(p, m, t, ts_type(p));
			if p.expect_no_recover(T![')']).is_none() {
				m.abandon(p);
				p.rewind(t);
				return None;
			};
			Some(m.complete(p, TS_PAREN))
		}
		_ => {
			let err = p
				.err_builder("expected a type")
				.primary(p.cur_tok().range, "");

			#[allow(deprecated)]
			SingleTokenParseRecovery::with_error(
				BASE_TS_RECOVERY_SET.union(token_set![
					T![typeof],
					T!['{'],
					T!['['],
					T!['('],
					T![this],
					T![import],
					T![-],
					JS_NUMBER_LITERAL,
					JS_STRING_LITERAL,
					TRUE_KW,
					FALSE_KW,
					JS_REGEX_LITERAL,
					BACKTICK,
					T![&],
					T![|]
				]),
				ERROR,
				err,
			)
			.recover(p);
			None
		}
	}
}

pub fn ts_type_args(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	if p.expect_no_recover(T![<]).is_none() {
		m.abandon(p);
		return None;
	}

	let mut first = true;

	let args_list = p.start();
	let mut progress = ParserProgress::default();

	while !p.at(EOF) && !p.at(T![>]) {
		progress.assert_progressing(p);
		if first {
			first = false;
		} else if p.at(T![,]) && p.nth_at(1, T![>]) {
			let m = p.start();
			let range = p.cur_tok().range;
			p.bump_any();
			m.complete(p, ERROR);
			let err = p
				.err_builder("type arguments may not contain trailing commas")
				.primary(range, "help: remove this comma");

			p.error(err);
		} else if p.expect_no_recover(T![,]).is_none() {
			args_list.abandon(p);
			m.abandon(p);
			return None;
		}

		if ts_type(p).is_none() && p.state.no_recovery {
			args_list.abandon(p);
			m.abandon(p);
			return None;
		}
	}
	args_list.complete(p, TS_TYPE_ARG_LIST);

	if p.expect_no_recover(T![>]).is_none() {
		m.abandon(p);
		None
	} else {
		Some(m.complete(p, TS_TYPE_ARGS))
	}
}

// FIXME: `<T() => {}` causes infinite recursion if the parser isnt being run with `no_recovery`
pub fn ts_type_params(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	if p.expect_no_recover(T![<]).is_none() {
		m.abandon(p);
		return None;
	}

	let mut first = true;

	let params_list = p.start();
	let mut progress = ParserProgress::default();
	while !p.at(EOF) && !p.at(T![>]) {
		progress.assert_progressing(p);
		if first {
			first = false;
		} else {
			if p.at(T![,]) && p.nth_at(1, T![>]) {
				p.bump_any();
				break;
			}
			if p.expect_no_recover(T![,]).is_none() {
				break;
			}
		}

		no_recover!(p, type_param(p));
	}
	params_list.complete(p, TS_TYPE_PARAM_LIST);

	p.expect_required(T![>]);
	Some(m.complete(p, TS_TYPE_PARAMS))
}

fn type_param(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let mut should_complete =
		if p.at_ts(token_set![T![ident], T![await], T![yield]]) || p.cur().is_keyword() {
			p.bump_remap(T![ident]);
			true
		} else {
			false
		};
	if p.cur_src() == "extends" {
		should_complete = true;
		let _m = p.start();
		p.bump_remap(T![extends]);
		no_recover!(p, ts_type(p));
		_m.complete(p, TS_CONSTRAINT);
	}
	if p.at(T![=]) {
		should_complete = true;
		let _m = p.start();
		p.bump_any();
		no_recover!(p, ts_type(p));
		_m.complete(p, TS_DEFAULT);
	}
	if should_complete {
		Some(m.complete(p, TS_TYPE_PARAM))
	} else {
		m.abandon(p);
		let err = p
			.err_builder("expected a type parameter, but found none")
			.primary(p.cur_tok().range, "");

		#[allow(deprecated)]
		SingleTokenParseRecovery::with_error(
			token_set![T![ident], T![yield], T![await], T![>], T![=]],
			ERROR,
			err,
		)
		.recover(p);
		None
	}
}

pub fn ts_import(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	p.expect_no_recover(T![import])?;
	p.expect_no_recover(T!['('])?;
	p.expect_no_recover(JS_STRING_LITERAL)?;
	p.expect_no_recover(T![')'])?;
	if p.eat(T![.]) {
		ts_entity_name(p, None, false);
	}
	if p.at(T![<]) && !p.has_linebreak_before_n(0) {
		ts_type_args(p);
	}

	Some(m.complete(p, TS_IMPORT))
}

pub fn ts_type_query(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	p.expect_no_recover(T![typeof])?;

	if p.at(T![import]) {
		no_recover!(p, ts_import(p));
	} else {
		no_recover!(p, ts_entity_name(p, None, true));
	}
	Some(m.complete(p, TS_TYPE_QUERY))
}

pub fn ts_mapped_type(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	p.expect_no_recover(T!['{'])?;
	let tok = p.cur_tok().range;
	let _m = p.start();
	if p.eat(T![+]) || p.eat(T![-]) {
		if p.cur_src() != "readonly" {
			let err = p
				.err_builder("`+` and `-` modifiers in mapped types must be followed by `readonly`")
				.primary(tok, "");

			p.error(err);
		} else {
			p.bump_remap(T![readonly]);
		}
		_m.complete(p, TS_MAPPED_TYPE_READONLY);
	} else if p.cur_src() == "readonly" {
		p.bump_remap(T![readonly]);
		_m.complete(p, TS_MAPPED_TYPE_READONLY);
	} else {
		_m.abandon(p);
	}

	let param = p.start();
	p.expect_no_recover(T!['['])?;
	// This is basically to unwrap the marker from a node to a single token
	if let Present(marker) = parse_identifier_name(p) {
		marker.undo_completion(p).abandon(p)
	}
	if p.cur_src() != "in" {
		let err = p
			.err_builder("expected `in` after a mapped type parameter name")
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		p.bump_any();
	}
	no_recover!(p, ts_type(p));
	if p.cur_src() == "as" {
		p.bump_any();
		ts_type(p);
	}
	p.expect_no_recover(T![']'])?;
	param.complete(p, TS_MAPPED_TYPE_PARAM);
	let tok = p.cur_tok().range;
	if p.eat(T![+]) || p.eat(T![-]) {
		if !p.at(T![?]) {
			// TODO: Im not sure of the proper terminology for this, someone should clarify this error
			let err = p
				.err_builder("`+` and `-` modifiers in mapped types must be followed by `?`")
				.primary(tok, "");

			p.error(err);
		} else {
			p.bump_any();
		}
	} else if p.at(T![?]) {
		p.bump_any();
	}

	p.expect_no_recover(T![:])?;
	no_recover!(p, ts_type(p));
	// FIXME: This should issue an error for no semi and no ASI, but the fact that a `}` is expected
	// after should make this case kind of rare
	p.eat(T![;]);
	p.expect_no_recover(T!['}'])?;
	Some(m.complete(p, TS_MAPPED_TYPE))
}

fn is_mapped_type_start(p: &Parser) -> bool {
	if (p.nth_at(1, T![+]) || p.nth_at(1, T![-])) && p.nth_src(2) == "readonly" {
		return true;
	}
	let mut cur = 1;
	if p.cur_src() == "readonly" {
		cur += 1;
	}
	if !p.nth_at(cur, T!['[']) {
		return false;
	}
	cur += 1;
	if !matches!(p.nth(cur), T![yield] | T![await] | T![ident]) {
		return false;
	}
	cur += 1;
	p.nth_at(cur, T![in])
}

pub fn ts_predicate(p: &mut Parser) -> Option<CompletedMarker> {
	let m = p.start();
	let mut advanced = false;

	if p.cur_src() == "asserts" {
		p.bump_any();
		advanced = true;
	}

	if p.at(T![this]) {
		let _m = p.start();
		p.bump_any();
		_m.complete(p, TS_THIS);
		advanced = true;
	} else if p.at_ts(token_set![T![await], T![yield], T![ident]]) {
		let _m = p.start();
		p.bump_any();
		_m.complete(p, TS_TYPE_NAME);
		advanced = true;
	}

	if p.cur_src() == "is" {
		p.bump_any();
		no_recover!(p, ts_type(p));
		advanced = true;
	}

	if !advanced {
		m.abandon(p);
		None
	} else {
		Some(m.complete(p, TS_PREDICATE))
	}
}

pub(crate) fn maybe_eat_incorrect_modifier(p: &mut Parser) -> Option<CompletedMarker> {
	let maybe_err = p.start();
	if matches!(p.cur_src(), "public" | "private" | "protected") {
		let m = p.start();
		p.bump_any();
		Some(m.complete(p, ERROR))
	} else if ts_modifier(p, &["readonly"]).is_some() {
		Some(maybe_err.complete(p, ERROR))
	} else {
		maybe_err.abandon(p);
		None
	}
}

pub fn ts_type_ref(
	p: &mut Parser,
	recovery_set: impl Into<Option<TokenSet>> + Clone,
) -> Option<CompletedMarker> {
	let t = p.checkpoint();

	let m = p.start();
	if let Some(err_m) = maybe_eat_incorrect_modifier(p) {
		let err = p
			.err_builder("a parameter property is only allowed in a constructor implementation")
			.primary(err_m.range(p), "");

		p.error(err);
	}

	if ts_entity_name(p, recovery_set, true).is_none() {
		m.abandon(p);
		p.rewind(t);
		return None;
	}

	if !p.has_linebreak_before_n(0) && p.at(T![<]) {
		no_recover!(p, m, t, ts_type_args(p));
	}

	Some(m.complete(p, TS_TYPE_REF))
}

pub fn ts_entity_name(
	p: &mut Parser,
	recovery_set: impl Into<Option<TokenSet>> + Clone,
	allow_reserved: bool,
) -> Option<CompletedMarker> {
	let init = ts_type_name(p, recovery_set.clone(), false)?;
	// TODO: maybe we should recover if no init at this point?

	let mut lhs = init;
	let set = recovery_set
		.into()
		.unwrap_or(BASE_TS_RECOVERY_SET)
		.union(token_set![T![.]]);

	while p.at(T![.]) {
		let m = lhs.precede(p);
		p.bump_any();
		// TODO: we should maybe move recovery out of ts_type_name since we dont need recovery here
		no_recover!(p, ts_type_name(p, set, allow_reserved));
		lhs = m.complete(p, TS_QUALIFIED_PATH);
	}
	Some(lhs)
}

pub fn ts_type_name(
	p: &mut Parser,
	recovery_set: impl Into<Option<TokenSet>>,
	allow_reserved: bool,
) -> Option<CompletedMarker> {
	if p.at(T![ident]) || (p.cur().is_keyword() && allow_reserved) {
		let m = p.start();
		p.bump_remap(T![ident]);
		return Some(m.complete(p, TS_TYPE_NAME));
	}

	// FIXME: move the recovery job out of this method
	let set = recovery_set.into().unwrap_or(BASE_TS_RECOVERY_SET);
	let err = p
		.err_builder(&format!(
			"expected a TypeScript type name, but instead found `{}`",
			p.cur_src()
		))
		.primary(p.cur_tok().range, "");

	#[allow(deprecated)]
	SingleTokenParseRecovery::with_error(set, ERROR, err).recover(p);
	None
}
