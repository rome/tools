//! Top level functions for parsing a script or module, also includes module specific items.

use syntax::stmt::FOLLOWS_LET;

use super::decl::{class_decl, function_decl};
use super::expr::{assign_expr, expr, identifier_name, literal, object_expr, primary_expr};
use super::pat::binding_identifier;
use super::stmt::{block_items, semi, var_decl};
use super::typescript::*;
use crate::{SyntaxKind::*, *};

#[macro_export]
macro_rules! at_ident_name {
    ($p:expr) => {
        ($p.at_ts(token_set![T![ident], T![await], T![yield]]) || $p.cur().is_keyword())
    };
    ($p:expr, $offset:expr) => {
        (token_set![T![ident], T![await], T![yield]].contains($p.nth($offset)) || $p.nth($offset).is_keyword())
    }
}

pub fn parse(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.eat(T![shebang]);
	block_items(p, true, true, false, None);
	m.complete(
		p,
		if p.syntax.file_kind == FileKind::Script {
			SyntaxKind::SCRIPT
		} else {
			SyntaxKind::MODULE
		},
	)
}

fn named_list(p: &mut Parser) -> Marker {
	let m = p.start();
	p.expect(T!['{']);
	let mut first = true;
	let specifiers_list = p.start();
	while !p.at(EOF) && !p.at(T!['}']) {
		if first {
			first = false;
		} else if p.at(T![,]) && p.nth_at(1, T!['}']) {
			p.bump_any();
			break;
		} else {
			p.expect(T![,]);
		}

		specifier(p);
	}
	specifiers_list.complete(p, LIST);
	p.expect(T!['}']);
	m
}

fn specifier(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	identifier_name(p);
	if p.cur_src() == "as" {
		p.bump_remap(T![as]);
		identifier_name(p);
	}
	m.complete(p, SPECIFIER)
}

fn named_export_specifier(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	identifier_name(p);
	if p.cur_src() == "as" {
		p.bump_remap(T![as]);
		identifier_name(p);
	}
	m.complete(p, SPECIFIER)
}

/// An import declaration
///
/// # Panics
/// Panics if the current syntax kind is not IMPORT_KW
pub fn import_decl(p: &mut Parser) -> CompletedMarker {
	assert_eq!(p.cur(), T![import]);
	let m = p.start();
	let start = p.cur_tok().range.start;

	// import.meta and import(foo)
	if p.nth_at(1, T![.]) || p.nth_at(1, T!['(']) {
		primary_expr(p).expect(
			"returned value from primary_expr should not be None because
                the current token is guaranteed to be `IMPORT_KW`",
		);
		semi(p, start..p.cur_tok().range.start);
		return m.complete(p, EXPR_STMT);
	}

	let p = &mut *p.with_state(ParserState {
		is_module: true,
		strict: Some(StrictMode::Module),
		..p.state.clone()
	});

	p.expect(T![import]);

	if p.at_ts(token_set![T![ident], T![async], T![yield]]) && p.nth_at(1, T![=]) {
		let mut complete = ts_import_equals_decl(p, m);
		complete.err_if_not_ts(
			p,
			"import equals declarations can only be used in TypeScript files",
		);
		return complete;
	}

	let list = p.start();

	if p.at(STRING) {
		let inner = p.start();
		p.bump_any();
		inner.complete(p, IMPORT_STRING_SPECIFIER);
		semi(p, start..p.cur_tok().range.start);

		list.complete(p, LIST);
		return m.complete(p, IMPORT_DECL);
	}

	let ty_only = p.cur_src() == "type"
		&& (p.nth_at(1, T!['{']) || p.nth_src(1) != "from" && !p.nth_at(1, T![,]));

	if ty_only {
		if !p.typescript() {
			let err = p
				.err_builder("type imports can only be used in TypeScript files")
				.primary(p.cur_tok().range, "");

			p.error(err);
			let m = p.start();
			p.bump_any();
			m.complete(p, ERROR);
		} else {
			p.bump_remap(T![type]);
		}
	}

	if p.at_ts(token_set![T![async], T![yield], T![ident]]) {
		imported_binding(p);
		if p.cur_src() != "from" {
			p.expect(T![,]);
		}
	}

	if p.at(T![*]) {
		let m = p.start();
		p.bump_any();
		if p.cur_src() != "as" {
			let err = p
				.err_builder("expected `as` for a namespace specifier, but found none")
				.primary(p.cur_tok().range, "");

			p.error(err);
		} else {
			p.bump_remap(T![as]);
		}
		imported_binding(p);
		m.complete(p, WILDCARD_IMPORT);
	} else if p.at(T!['{']) {
		named_list(p).complete(p, NAMED_IMPORTS);
	}

	list.complete(p, LIST);

	if p.cur_src() != "from" {
		let err = p
			.err_builder("expected a `from` clause for an import, but found none")
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		p.bump_remap(T![from]);
	}

	if !p.at(STRING) {
		let err = p
			.err_builder(
				"expected a source for a `from` clause in an import statement, but found none",
			)
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		literal(p);
	}

	if p.cur_src() == "assert" {
		p.bump_remap(T![assert]);
		if !p.at(T!['{']) {
			let err = p
				.err_builder("assert clauses in import declarations require an object expression")
				.primary(p.cur_tok().range, "");

			p.error(err);
		} else {
			object_expr(p);
		}
	}

	semi(p, start..p.cur_tok().range.start);
	m.complete(p, IMPORT_DECL)
}

fn imported_binding(p: &mut Parser) {
	let p = &mut *p.with_state(ParserState {
		in_async: false,
		in_generator: false,
		..p.state.clone()
	});
	binding_identifier(p);
}

pub fn export_decl(p: &mut Parser) -> CompletedMarker {
	let start = p.cur_tok().range.start;
	let m = p.start();
	p.expect(T![export]);

	let declare = p.typescript() && p.cur_src() == "declare";

	if declare {
		if let Some(mut res) = try_parse_ts(p, ts_declare) {
			res.err_if_not_ts(
				p,
				"TypeScript declarations can only be used in TypeScript files",
			);
			return m.complete(p, EXPORT_DECL);
		}
	}

	let offset = declare as usize;

	if p.typescript() && at_ident_name!(p, offset) {
		if let Some(mut res) = try_parse_ts(p, ts_decl) {
			res.err_if_not_ts(
				p,
				"TypeScript declarations can only be used in TypeScript files",
			);
			return m.complete(p, EXPORT_DECL);
		}
	}

	macro_rules! err_if_declare {
		($p:expr, $declare:expr, $msg:literal) => {
			if $declare {
				let range = $p.cur_tok().range;
				$p.bump_remap(T![declare]);
				let err = $p.err_builder($msg).primary(range, "");

				$p.error(err);
			}
		};
	}

	match p.nth(offset) {
		T![import] => {
			err_if_declare!(
				p,
				declare,
				"`declare` modifiers cannot be applied to import declarations"
			);
			p.bump_any();
			let mut complete = ts_import_equals_decl(p, m);
			complete.err_if_not_ts(
				p,
				"import equals declarations can only be used in TypeScript files",
			);
			return complete;
		}
		T![=] => {
			err_if_declare!(
				p,
				declare,
				"`declare` modifiers cannot be applied to export equals declarations"
			);
			p.bump_any();
			expr(p);
			semi(p, start..p.cur_tok().range.start);
			let mut complete = m.complete(p, TS_EXPORT_ASSIGNMENT);
			complete.err_if_not_ts(
				p,
				"export equals declarations can only be used in TypeScript files",
			);
			return complete;
		}
		_ if p.nth_src(offset) == "as" => {
			err_if_declare!(
				p,
				declare,
				"`declare` modifiers cannot be applied to export as namespace declarations"
			);
			p.bump_remap(T![as]);
			if p.cur_src() != "namespace" {
				let err = p
					.err_builder("expected `namespace`, but found none")
					.primary(p.cur_tok().range, "");

				p.error(err);
			} else {
				p.bump_remap(T![namespace]);
			}

			// TODO(RDambrosio016): verify, is identifier_name correct here or should it just be ident?
			identifier_name(p);
			semi(p, start..p.cur_tok().range.start);
			let mut complete = m.complete(p, TS_NAMESPACE_EXPORT_DECL);
			complete.err_if_not_ts(
				p,
				"export as namespace declarations can only be used in TypeScript files",
			);
			return complete;
		}
		_ => {}
	}

	// TODO: Is this logic correct? declare seems to not be always allowed but
	// considering ts has no spec (D:) it's kind of hard to know where it is allowed.
	// even swc and babel seem to get this wrong
	if declare {
		if !p.typescript() {
			let m = p.start();
			let err = p
				.err_builder("declare modifiers can only be used in TypeScript files")
				.primary(p.cur_tok().range, "");

			p.error(err);
			p.bump_any();
			m.complete(p, ERROR);
		} else {
			p.bump_remap(T![declare]);
		}
	}

	let only_ty = p.typescript()
		&& (p.cur_src() == "type" && {
			p.bump_remap(T![type]);
			true
		});

	let mut exports_ns = false;
	let mut has_star = false;

	if p.eat(T![*]) {
		has_star = true;
		if p.cur_src() == "from" {
			from_clause_and_semi(p, start);
			return m.complete(p, EXPORT_WILDCARD);
		}
		if p.cur_src() == "as" {
			p.bump_remap(T![as]);
			identifier_name(p);
			exports_ns = true;
		}
	}

	let mut export_default = false;

	if !only_ty && !exports_ns && p.eat(T![default]) {
		if p.cur_src() == "abstract" && p.nth_at(1, T![class]) {
			p.state.decorators_were_valid = true;
			let inner = p.start();
			if !p.typescript() {
				let err = p
					.err_builder("`abstract` modifiers can only be used in TypeScript files")
					.primary(p.cur_tok().range, "");

				p.error(err);
				let m = p.start();
				p.bump_any();
				m.complete(p, ERROR);
			} else {
				p.bump_remap(T![abstract]);
			}
			let decl = class_decl(
				&mut *p.with_state(ParserState {
					in_default: true,
					..p.state.clone()
				}),
				false,
			);
			decl.undo_completion(p).abandon(p);
			inner.complete(p, CLASS_DECL);
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if p.cur_src() == "interface" {
			if let Some(ref mut compl) = ts_interface(p) {
				compl.err_if_not_ts(p, "interfaces can only be used in TypeScript files");
			}
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if p.at(T![class]) {
			p.state.decorators_were_valid = true;
			class_decl(
				&mut *p.with_state(ParserState {
					in_default: true,
					..p.state.clone()
				}),
				false,
			);
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if p.cur_src() == "async" && p.nth_at(1, T![function]) && !p.has_linebreak_before_n(1) {
			p.state.decorators_were_valid = true;
			let mut guard = p.with_state(ParserState {
				in_async: true,
				..p.state.clone()
			});
			let inner = guard.start();
			guard.bump_any();
			function_decl(&mut *guard, inner, false);
			return m.complete(&mut *guard, EXPORT_DEFAULT_DECL);
		}

		if p.cur_src() == "from" || (p.at(T![,]) && p.nth_at(1, T!['{'])) {
			export_default = true;
		} else {
			assign_expr(p);
			semi(p, start..p.cur_tok().range.start);
			return m.complete(p, EXPORT_DEFAULT_EXPR);
		}
	}

	if !only_ty && p.at(T![class]) {
		p.state.decorators_were_valid = true;
		class_decl(p, false);
	} else if !only_ty
		&& p.cur_src() == "async"
		&& p.nth_at(1, T![function])
		&& !p.has_linebreak_before_n(1)
	{
		p.state.decorators_were_valid = true;
		let mut guard = p.with_state(ParserState {
			in_async: true,
			..p.state.clone()
		});
		let inner = guard.start();
		guard.bump_any();
		function_decl(&mut *guard, inner, false);
	} else if !only_ty && p.at(T![function]) {
		p.state.decorators_were_valid = true;
		let m = p.start();
		function_decl(p, m, false);
	} else if !only_ty && p.at(T![const]) && p.nth_src(1) == "enum" {
		ts_enum(p).err_if_not_ts(p, "enums can only be used in TypeScript files");
	} else if !only_ty
		&& (p.at(T![var])
			|| p.at(T![const])
			|| (p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1))))
	{
		var_decl(p, false);
	} else {
		let m = p.start();

		if p.cur_src() == "from" && exports_ns {
			from_clause_and_semi(p, start);
			return m.complete(p, EXPORT_WILDCARD);
		}

		if !export_default
			&& (token_set![T![async], T![yield], T![yield]].contains(p.cur())
				|| p.cur().is_keyword())
		{
			identifier_name(p);
			export_default = true;
		}

		if p.cur_src() == "from" && export_default {
			from_clause_and_semi(p, start);
			return m.complete(p, EXPORT_NAMED);
		}

		if has_star && !exports_ns {
			from_clause_and_semi(p, start);
			return m.complete(p, EXPORT_WILDCARD);
		}

		if exports_ns || export_default {
			p.expect(T![,]);
		}

		p.expect(T!['{']);

		let mut first = true;
		let specifiers = p.start();

		while (!p.at(EOF) && p.at(T![,])) || crate::at_ident_name!(p) {
			if first {
				first = false;
			} else if p.eat(T![,]) && p.at(T!['}']) {
				break;
			}
			named_export_specifier(p);
		}

		specifiers.complete(p, LIST);
		p.expect(T!['}']);

		if p.cur_src() == "from" {
			from_clause_and_semi(p, start);
		} else {
			semi(p, start..p.cur_tok().range.start);
			if export_default || exports_ns {
				let err = p
					.err_builder(
						"`export default` and `export as` declarations must have a `from` clause",
					)
					.primary(start..p.cur_tok().range.start, "");

				p.error(err);
			}
		}

		m.complete(p, EXPORT_NAMED);
	}
	m.complete(p, EXPORT_DECL)
}

fn from_clause_and_semi(p: &mut Parser, start: usize) {
	debug_assert_eq!(p.cur_src(), "from");
	p.bump_remap(T![from]);
	literal(p);
	semi(p, start..p.cur_tok().range.start);
}

pub fn ts_import_equals_decl(p: &mut Parser, m: Marker) -> CompletedMarker {
	let start = p.cur_tok().range.start;
	identifier_name(p);
	p.expect(T![=]);

	if p.cur_src() == "require" && p.nth_at(1, T!['(']) {
		ts_external_module_ref(p);
	} else {
		ts_entity_name(p, None, false);
	}
	semi(p, start..p.cur_tok().range.start);
	m.complete(p, TS_IMPORT_EQUALS_DECL)
}

pub fn ts_external_module_ref(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	if p.cur_src() != "require" {
		let err = p
			.err_builder("expected `require` for an external module reference, but found none")
			.primary(p.cur_tok().range, "");

		p.error(err);
	} else {
		p.bump_remap(T![require]);
	}

	p.expect(T!['(']);
	p.expect(STRING);
	p.expect(T![')']);
	m.complete(p, TS_EXTERNAL_MODULE_REF)
}
