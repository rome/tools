//! Top level functions for parsing a script or module, also includes module specific items.

use super::module::parse_module_body;
use super::stmt::parse_statements;
use crate::state::{ChangeParserState, EnableStrictMode};
use crate::syntax::stmt::parse_directives;
use crate::{CompletedMarker, ModuleKind, Parser};
use rome_js_syntax::JsSyntaxKind::*;

// test_err unterminated_unicode_codepoint
// let s = "\u{200";

pub(crate) fn parse(p: &mut Parser) -> CompletedMarker {
    let m = p.start();
    p.eat(JS_SHEBANG);

    let (statement_list, strict_snapshot) = parse_directives(p);

    let result = match p.source_type.module_kind() {
        ModuleKind::Script => {
            parse_statements(p, false, statement_list);
            m.complete(p, JS_SCRIPT)
        }
        ModuleKind::Module => {
            parse_module_body(p, statement_list);
            m.complete(p, JS_MODULE)
        }
    };

    if let Some(strict_snapshot) = strict_snapshot {
        EnableStrictMode::restore(&mut p.state, strict_snapshot);
    }

    result
}
