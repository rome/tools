//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyImportClause;
impl ToFormatElement for JsAnyImportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportBareClause(node) => node.to_format_element(formatter),
            Self::JsImportNamedClause(node) => node.to_format_element(formatter),
            Self::JsImportDefaultClause(node) => node.to_format_element(formatter),
            Self::JsImportNamespaceClause(node) => node.to_format_element(formatter),
        }
    }
}
