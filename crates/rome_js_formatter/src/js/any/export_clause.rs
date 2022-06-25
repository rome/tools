//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyExportClause;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyExportClause;
impl FormatRule<JsAnyExportClause> for FormatJsAnyExportClause {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyExportClause, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExportClause::JsExportDefaultDeclarationClause(node) => node.format().fmt(f),
            JsAnyExportClause::JsExportDefaultExpressionClause(node) => node.format().fmt(f),
            JsAnyExportClause::JsExportNamedClause(node) => node.format().fmt(f),
            JsAnyExportClause::JsExportFromClause(node) => node.format().fmt(f),
            JsAnyExportClause::JsExportNamedFromClause(node) => node.format().fmt(f),
            JsAnyExportClause::JsAnyDeclarationClause(node) => node.format().fmt(f),
            JsAnyExportClause::TsExportAsNamespaceClause(node) => node.format().fmt(f),
            JsAnyExportClause::TsExportAssignmentClause(node) => node.format().fmt(f),
            JsAnyExportClause::TsExportDeclareClause(node) => node.format().fmt(f),
        }
    }
}
