//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyExportClause;
impl Format for JsAnyExportClause {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsExportDefaultDeclarationClause(node) => node.format(formatter),
            Self::JsExportDefaultExpressionClause(node) => node.format(formatter),
            Self::JsExportNamedClause(node) => node.format(formatter),
            Self::JsExportFromClause(node) => node.format(formatter),
            Self::JsExportNamedFromClause(node) => node.format(formatter),
            Self::JsAnyDeclarationClause(node) => node.format(formatter),
            Self::TsExportAsNamespaceClause(node) => node.format(formatter),
            Self::TsExportAssignmentClause(node) => node.format(formatter),
            Self::TsExportDeclareClause(node) => node.format(formatter),
        }
    }
}
