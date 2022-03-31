//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyExportClause;
impl ToFormatElement for JsAnyExportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsExportDefaultDeclarationClause(node) => node.to_format_element(formatter),
            Self::JsExportDefaultExpressionClause(node) => node.to_format_element(formatter),
            Self::JsExportNamedClause(node) => node.to_format_element(formatter),
            Self::JsExportFromClause(node) => node.to_format_element(formatter),
            Self::JsExportNamedFromClause(node) => node.to_format_element(formatter),
            Self::JsAnyDeclarationClause(node) => node.to_format_element(formatter),
            Self::TsExportAsNamespaceClause(node) => node.to_format_element(formatter),
            Self::TsExportAssignmentClause(node) => node.to_format_element(formatter),
            Self::TsExportDeclareClause(node) => node.to_format_element(formatter),
        }
    }
}
