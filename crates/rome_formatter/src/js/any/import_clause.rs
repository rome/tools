//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyImportClause;
impl ToFormatElement for JsAnyImportClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsImportBareClause(node) => node.format(formatter),
            Self::JsImportNamedClause(node) => node.format(formatter),
            Self::JsImportDefaultClause(node) => node.format(formatter),
            Self::JsImportNamespaceClause(node) => node.format(formatter),
        }
    }
}
