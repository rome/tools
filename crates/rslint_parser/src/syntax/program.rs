//! Top level functions for parsing a script or module, also includes module specific items.

use super::expr::parse_name;
use super::stmt::{parse_statements, semi};
use super::typescript::*;
use crate::syntax::js_parse_error;
use crate::syntax::module::parse_module_body;
use crate::syntax::stmt::directives;
use crate::{JsSyntaxKind::*, *};

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
