//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportClause;
impl FormatRule<JsAnyImportClause> for FormatJsAnyImportClause {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyImportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportClause::JsImportBareClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportNamedClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportDefaultClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportNamespaceClause(node) => node.format().fmt(f),
        }
    }
}
