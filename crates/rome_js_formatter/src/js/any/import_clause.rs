//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportClause;
impl FormatRule<JsAnyImportClause> for FormatJsAnyImportClause {
    type Context = JsFormatContext;
    fn format(node: &JsAnyImportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportClause::JsImportBareClause(node) => node.format().format(f),
            JsAnyImportClause::JsImportNamedClause(node) => node.format().format(f),
            JsAnyImportClause::JsImportDefaultClause(node) => node.format().format(f),
            JsAnyImportClause::JsImportNamespaceClause(node) => node.format().format(f),
        }
    }
}
