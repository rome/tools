use crate::parser::{expected_any, expected_node, ParserProgress, ToDiagnostic};
use crate::syntax::binding::parse_binding;
use crate::syntax::js_parse_error::{expected_binding, expected_statement};
use crate::syntax::program::export_decl;
use crate::syntax::stmt::{parse_statement, semi, STMT_RECOVERY_SET};
use crate::{
	Absent, CompletedMarker, Marker, ParseRecovery, ParsedSyntax, Parser, Present, TokenSet,
};
use rslint_errors::Diagnostic;
use rslint_syntax::SyntaxKind::*;
use rslint_syntax::{SyntaxKind, T};
use std::ops::Range;

///! Implements the parsing logic for ES Module syntax

// test module
// import a from "b";
// export { a };
// c();
// import { c } from "c";
pub(crate) fn parse_module_body(p: &mut Parser, m: Marker) -> CompletedMarker {
	parse_module_items(p);

	m.complete(p, JS_MODULE)
}

fn parse_module_items(p: &mut Parser) {
	let list_marker = p.start();
	let mut progress = ParserProgress::default();
	let mut empty = true;

	while !p.at(EOF) {
		progress.assert_progressing(p);

		let module_item = parse_module_item(p);

		let recovered = module_item.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_STATEMENT, STMT_RECOVERY_SET),
			expected_statement,
		);

		if recovered.is_err() {
			break;
		}

		empty = false;
	}

	if empty {
		list_marker.abandon(p);
		p.missing();
	} else {
		list_marker.complete(p, LIST);
	}
}

fn parse_module_item(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		T![import] if !token_set![T![.], T!['(']].contains(p.nth(1)) => parse_import(p),
		T![export] => export_decl(p).into(),
		_ => {
			let checkpoint = p.checkpoint();

			if let Some(statement) = parse_statement(p, None) {
				Present(statement)
			} else {
				// TODO remove once error recovery is removed from `parse_statement`
				p.rewind(checkpoint);
				Absent
			}
		}
	}
}

// test_err import_err
// import;
// import *;
// import * as b, { a, b } from "c";
// import { a + b, d } from "c";
// import { a, a } from "c";
// import { default } from "c";
pub(crate) fn parse_import(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![import]) {
		return Absent;
	}

	let start = p.cur_tok().range.start;
	let import = p.start();
	p.bump_any();

	p.state.duplicate_binding_parent = Some("import");

	parse_import_clause(p).or_missing_with_error(p, |p, range| {
		expected_any(
			&["default import", "namespace import", "named import"],
			range,
		)
		.to_diagnostic(p)
	});

	p.state.duplicate_binding_parent = None;
	p.state.name_map.clear();

	let end = p.cur_tok().range.start;

	semi(p, start..end);

	Present(import.complete(p, JS_IMPORT))
}

// test import_default_clause
// import foo from "test";
fn parse_import_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		JS_STRING_LITERAL => parse_import_bare_clause(p),
		T![*] => parse_import_namespace_clause(p),
		T!['{'] => parse_import_named_clause(p),
		_ => match parse_binding(p) {
			Absent => Absent,
			Present(binding) => {
				let m = binding.precede(p);

				if matches!(p.cur(), T![,] | T!['{']) {
					p.expect_required(T![,]);

					let default_specifier = m.complete(p, JS_DEFAULT_IMPORT_SPECIFIER);
					let named_clause = default_specifier.precede(p);

					parse_named_import(p).or_missing_with_error(p, expected_named_import);

					expect_keyword(p, "from", T![from]);
					parse_module_source(p).or_missing_with_error(p, expected_module_source);

					Present(named_clause.complete(p, JS_IMPORT_NAMED_CLAUSE))
				} else {
					expect_keyword(p, "from", T![from]);
					parse_module_source(p).or_missing_with_error(p, expected_module_source);

					Present(m.complete(p, JS_IMPORT_DEFAULT_CLAUSE))
				}
			}
		},
	}
}

// test import_bare_clause
// import "test";
// import "no_semicolon"
fn parse_import_bare_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	parse_module_source(p)
		.map(|module_source| module_source.precede(p).complete(p, JS_IMPORT_BARE_CLAUSE))
}

fn parse_import_namespace_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![*]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any();

	expect_keyword(p, "as", T![as]);

	parse_binding(p).or_missing_with_error(p, expected_binding);

	expect_keyword(p, "from", T![from]);
	parse_module_source(p).or_missing_with_error(p, expected_module_source);

	Present(m.complete(p, JS_IMPORT_NAMESPACE_CLAUSE))
}

// test import_named_clause
// import {} from "a";
// import { a, b, c, } from "b";
// import b, { a } from "b";
// import a, * as b from "c";
// import { a as b, default as c, "a-b-c" as d } from "b";
fn parse_import_named_clause(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['{']) {
		return Absent;
	}

	let m = p.start();
	parse_default_import_specifier(p).or_missing(p);

	parse_named_import(p).or_missing_with_error(p, expected_named_import);

	expect_keyword(p, "from", T![from]);
	parse_module_source(p).or_missing_with_error(p, expected_module_source);

	Present(m.complete(p, JS_IMPORT_NAMED_CLAUSE))
}

fn parse_default_import_specifier(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	parse_binding(p).map(|binding| {
		let m = binding.precede(p);
		p.expect_required(T![,]);
		m.complete(p, JS_DEFAULT_IMPORT_SPECIFIER)
	})
}

fn parse_named_import(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		T![*] => parse_namespace_import_specifier(p),
		_ => parse_named_import_specifier_list(p),
	}
}

fn parse_namespace_import_specifier(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T![*]) {
		return Absent;
	}

	let m = p.start();
	p.bump_any();
	expect_keyword(p, "as", T![as]);
	parse_binding(p).or_missing_with_error(p, expected_binding);

	Present(m.complete(p, JS_NAMESPACE_IMPORT_SPECIFIER))
}

fn parse_named_import_specifier_list(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(T!['{']) {
		return Absent;
	}

	let m = p.start();
	p.bump_any();

	let list = p.start();
	let mut progress = ParserProgress::default();
	let mut first = true;

	while !matches!(p.cur(), EOF | T!['}'] | T![;]) {
		progress.assert_progressing(p);

		if first {
			first = false;
		} else {
			p.expect_required(T![,]);

			// Has this been a trailing comma and the parser is now at the end of the specifiers list?
			if p.at(T!['}']) {
				break;
			}
		}

		let specifier = parse_any_named_import_specifier(p);
		let recovered = specifier.or_recover(
			p,
			&ParseRecovery::new(JS_UNKNOWN_SPECIFIER, token_set![T![,], T!['}'], T![;]]),
			expected_named_import_specifier,
		);

		if recovered.is_err() {
			break;
		}
	}

	if first {
		list.abandon(p);
		p.missing();
	} else {
		list.complete(p, LIST);
	}

	p.expect_required(T!['}']);

	Present(m.complete(p, JS_NAMED_IMPORT_SPECIFIER_LIST))
}

fn parse_any_named_import_specifier(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if p.nth_src(1) == "as" {
		Present(parse_named_import_specifier(p))
	} else {
		parse_shorthand_named_import_specifier(p)
	}
}

fn parse_named_import_specifier(p: &mut Parser) -> CompletedMarker {
	let m = p.start();
	parse_export_name(p).or_missing_with_error(p, expected_export_name);

	expect_keyword(p, "as", T![as]);
	parse_binding(p).or_missing_with_error(p, expected_binding);

	m.complete(p, JS_NAMED_IMPORT_SPECIFIER)
}

fn parse_shorthand_named_import_specifier(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	parse_binding(p).map(|binding| {
		binding
			.precede(p)
			.complete(p, JS_SHORTHAND_NAMED_IMPORT_SPECIFIER)
	})
}

fn parse_export_name(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	match p.cur() {
		JS_STRING_LITERAL | T![ident] => {
			let m = p.start();
			p.bump_any();
			Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
		}
		t if t.is_keyword() => {
			let m = p.start();
			p.bump_remap(T![ident]);
			Present(m.complete(p, JS_LITERAL_EXPORT_NAME))
		}
		_ => Absent,
	}
}

fn parse_module_source(p: &mut Parser) -> ParsedSyntax<CompletedMarker> {
	if !p.at(JS_STRING_LITERAL) {
		Absent
	} else {
		let m = p.start();
		p.bump_any();
		Present(m.complete(p, JS_MODULE_SOURCE))
	}
}

fn expect_keyword(p: &mut Parser, keyword_name: &str, kind: SyntaxKind) {
	if p.at(T![ident]) && p.cur_src() == keyword_name {
		p.bump_remap(kind);
	} else {
		let err = if p.cur() == SyntaxKind::EOF {
			p.err_builder(&format!(
				"expected `{}` but instead the file ends",
				keyword_name
			))
			.primary(p.cur_tok().range, "the file ends here")
		} else {
			p.err_builder(&format!(
				"expected `{}` but instead found `{}`",
				keyword_name,
				p.cur_src()
			))
			.primary(p.cur_tok().range, "unexpected")
		};

		p.error(err);
	}
}

fn expected_module_source(p: &Parser, range: Range<usize>) -> Diagnostic {
	expected_node("string literal", range).to_diagnostic(p)
}

fn expected_named_import(p: &Parser, range: Range<usize>) -> Diagnostic {
	expected_any(&["namespace import", "named imports"], range).to_diagnostic(p)
}

fn expected_export_name(p: &Parser, range: Range<usize>) -> Diagnostic {
	expected_any(&["string literal", "identifier"], range).to_diagnostic(p)
}

fn expected_named_import_specifier(p: &Parser, range: Range<usize>) -> Diagnostic {
	expected_any(&["identifier", "string literal"], range).to_diagnostic(p)
}
