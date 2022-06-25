//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyImportClause;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyImportClause;
impl FormatRule<JsAnyImportClause> for FormatJsAnyImportClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyImportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyImportClause::JsImportBareClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportNamedClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportDefaultClause(node) => node.format().fmt(f),
            JsAnyImportClause::JsImportNamespaceClause(node) => node.format().fmt(f),
        }
    }
}
