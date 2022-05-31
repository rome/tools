//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExportClause;
use crate::prelude::*;
use rome_js_syntax::JsAnyExportClause;
impl FormatRule<JsAnyExportClause> for FormatJsAnyExportClause {
    type Context = JsFormatContext;
    fn format(node: &JsAnyExportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExportClause::JsExportDefaultDeclarationClause(node) => node.format().format(f),
            JsAnyExportClause::JsExportDefaultExpressionClause(node) => node.format().format(f),
            JsAnyExportClause::JsExportNamedClause(node) => node.format().format(f),
            JsAnyExportClause::JsExportFromClause(node) => node.format().format(f),
            JsAnyExportClause::JsExportNamedFromClause(node) => node.format().format(f),
            JsAnyExportClause::JsAnyDeclarationClause(node) => node.format().format(f),
            JsAnyExportClause::TsExportAsNamespaceClause(node) => node.format().format(f),
            JsAnyExportClause::TsExportAssignmentClause(node) => node.format().format(f),
            JsAnyExportClause::TsExportDeclareClause(node) => node.format().format(f),
        }
    }
}
