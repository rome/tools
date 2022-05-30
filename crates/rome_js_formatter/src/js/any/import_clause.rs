//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyImportClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyImportClause;
impl FormatRule<JsAnyImportClause> for FormatJsAnyImportClause {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyImportClause,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyImportClause::JsImportBareClause(node) => formatted![formatter, [node.format()]],
            JsAnyImportClause::JsImportNamedClause(node) => formatted![formatter, [node.format()]],
            JsAnyImportClause::JsImportDefaultClause(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyImportClause::JsImportNamespaceClause(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
