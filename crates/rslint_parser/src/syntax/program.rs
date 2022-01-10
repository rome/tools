//! Top level functions for parsing a script or module, also includes module specific items.

use super::expr::{parse_expr_or_assignment, parse_expression, parse_name};
use super::stmt::{parse_statements, semi, variable_declaration_statement};
use super::typescript::*;
use crate::parser::ParserProgress;
use crate::syntax::class::parse_class_statement;
use crate::syntax::expr::is_at_name;
use crate::syntax::function::parse_function_statement;
use crate::syntax::function::{is_at_async_function, LineBreak};
use crate::syntax::js_parse_error;
use crate::syntax::module::parse_module_body;
use crate::syntax::stmt::directives;
use crate::ParsedSyntax::Present;
use crate::{JsSyntaxKind::*, *};
use syntax::stmt::FOLLOWS_LET;

#[macro_export]
macro_rules! at_ident_name {
    ($p:expr) => {
        ($p.at_ts(token_set![T![ident], T![await], T![yield]]) || $p.cur().is_keyword())
    };
    ($p:expr, $offset:expr) => {
        (token_set![T![ident], T![await], T![yield]].contains($p.nth($offset)) || $p.nth($offset).is_keyword())
    }
}

// test_err unterminated_unicode_codepoint
// let s = "\u{200";

pub fn parse(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	p.eat(JS_SHEBANG);

	let old_parser_state = directives(p);

	let result = match p.syntax.file_kind {
		FileKind::Script => {
			parse_statements(p, false);
			m.complete(p, JS_SCRIPT)
		}
		FileKind::Module | FileKind::TypeScript => parse_module_body(p, m),
	};

	if let Some(old_parser_state) = old_parser_state {
		p.state = old_parser_state;
	}

	result
}

fn named_export_specifier(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
	if p.cur_src() == "as" {
		p.bump_remap(T![as]);
		parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
	}
	m.complete(p, SPECIFIER)
}

// test export
// export { foo } from "bla";
pub fn export_decl(p: &mut Parser) -> CompletedMarker {
	let start = p.cur_tok().start();
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
				let range = $p.cur_tok().range();
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
			parse_expression(p).or_add_diagnostic(p, js_parse_error::expected_expression);
			semi(p, start..p.cur_tok().start());
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
					.primary(p.cur_tok().range(), "");

				p.error(err);
			} else {
				p.bump_remap(T![namespace]);
			}

			// TODO(RDambrosio016): verify, is identifier_name correct here or should it just be ident?
			parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
			semi(p, start..p.cur_tok().start());
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
				.primary(p.cur_tok().range(), "");

			p.error(err);
			p.bump_any();
			m.complete(p, JS_UNKNOWN);
		} else {
			p.bump_remap(T![declare]);
		}
	}

	let only_ty = if p.typescript() && p.cur_src() == "type" {
		p.bump_remap(T![type]);
		true
	} else {
		false
	};

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
			parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
			exports_ns = true;
		}
	}

	let mut export_default = false;

	if !only_ty && !exports_ns && p.eat(T![default]) {
		if p.cur_src() == "abstract" && p.nth_at(1, T![class]) {
			let inner = p.start();
			if !p.typescript() {
				let err = p
					.err_builder("`abstract` modifiers can only be used in TypeScript files")
					.primary(p.cur_tok().range(), "");

				p.error(err);
				let m = p.start();
				p.bump_any();
				m.complete(p, JS_UNKNOWN);
			} else {
				p.bump_remap(T![abstract]);
			}
			let decl = parse_class_statement(&mut *p.with_state(ParserState {
				in_default: true,
				..p.state.clone()
			}))
			.unwrap();

			decl.undo_completion(p).abandon(p);
			inner.complete(p, JS_CLASS_STATEMENT);
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if p.cur_src() == "interface" {
			if let Some(ref mut compl) = ts_interface(p) {
				compl.err_if_not_ts(p, "interfaces can only be used in TypeScript files");
			}
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if p.at(T![class]) {
			let p = &mut *p.with_state(ParserState {
				in_default: true,
				..p.state.clone()
			});
			parse_class_statement(p).unwrap();
			return m.complete(p, EXPORT_DEFAULT_DECL);
		}

		if is_at_async_function(p, LineBreak::DoCheck) {
			if let Present(_) = parse_function_statement(p) {
				return m.complete(p, EXPORT_DEFAULT_DECL);
			}
		}

		if p.cur_src() == "from" || (p.at(T![,]) && p.nth_at(1, T!['{'])) {
			export_default = true;
		} else {
			parse_expr_or_assignment(p).or_add_diagnostic(p, js_parse_error::expected_expression);
			semi(p, start..p.cur_tok().start());
			return m.complete(p, EXPORT_DEFAULT_EXPR);
		}
	}

	if !only_ty && p.at(T![class]) {
		parse_class_statement(p).unwrap();
	} else if !only_ty
		// function ...
		&& (p.at(T![function])
			||
		is_at_async_function(p, LineBreak::DoCheck))
	{
		parse_function_statement(p).unwrap();
	} else if !only_ty && p.at(T![const]) && p.nth_src(1) == "enum" {
		ts_enum(p).err_if_not_ts(p, "enums can only be used in TypeScript files");
	} else if !only_ty
		&& (p.at(T![var])
			|| p.at(T![const])
			|| (p.cur_src() == "let" && FOLLOWS_LET.contains(p.nth(1))))
	{
		variable_declaration_statement(p).unwrap();
	} else {
		if p.cur_src() == "from" && exports_ns {
			from_clause_and_semi(p, start);
			return m.complete(p, EXPORT_WILDCARD);
		}

		if !export_default && is_at_name(p) {
			parse_name(p).unwrap();
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
		let mut progress = ParserProgress::default();

		while (!p.at(EOF) && p.at(T![,])) || crate::at_ident_name!(p) {
			progress.assert_progressing(p);

			if first {
				first = false;
			} else if p.eat(T![,]) && p.at(T!['}']) {
				break;
			}
			named_export_specifier(p);
		}

		specifiers.complete(p, EXPORT_NAMED_SPECIFIER_LIST);
		p.expect(T!['}']);

		if p.cur_src() == "from" {
			from_clause_and_semi(p, start);
		} else {
			semi(p, start..p.cur_tok().start());
			if export_default || exports_ns {
				let err = p
					.err_builder(
						"`export default` and `export as` declarations must have a `from` clause",
					)
					.primary(start..p.cur_tok().start(), "");

				p.error(err);
			}
		}

		return m.complete(p, EXPORT_NAMED);
	}
	m.complete(p, EXPORT_DECL)
}

fn from_clause_and_semi(p: &mut Parser, start: usize) {
	debug_assert_eq!(p.cur_src(), "from");
	p.bump_remap(T![from]);
	p.expect(JS_STRING_LITERAL);
	semi(p, start..p.cur_tok().start());
}

pub fn ts_import_equals_decl(p: &mut Parser, m: Marker) -> CompletedMarker {
	let start = p.cur_tok().start();
	parse_name(p).or_add_diagnostic(p, js_parse_error::expected_identifier);
	p.expect(T![=]);

	if p.cur_src() == "require" && p.nth_at(1, T!['(']) {
		ts_external_module_ref(p);
	} else {
		ts_entity_name(p, None, false);
	}
	semi(p, start..p.cur_tok().start());
	m.complete(p, TS_IMPORT_EQUALS_DECL)
}

pub fn ts_external_module_ref(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	if p.cur_src() != "require" {
		let err = p
			.err_builder("expected `require` for an external module reference, but found none")
			.primary(p.cur_tok().range(), "");

		p.error(err);
	} else {
		p.bump_remap(T![require]);
	}

	p.expect(T!['(']);
	p.expect(JS_STRING_LITERAL);
	p.expect(T![')']);
	m.complete(p, TS_EXTERNAL_MODULE_REF)
}
