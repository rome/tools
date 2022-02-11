//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::AnyJsImportClause;
impl ToFormatElement for AnyJsImportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportBareClause(node) => node.to_format_element(formatter),
            Self::JsImportNamedClause(node) => node.to_format_element(formatter),
            Self::JsImportDefaultClause(node) => node.to_format_element(formatter),
            Self::JsImportNamespaceClause(node) => node.to_format_element(formatter),
        }
    }
}
