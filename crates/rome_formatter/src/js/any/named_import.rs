//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyNamedImport;
impl ToFormatElement for JsAnyNamedImport {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsNamedImportSpecifiers(node) => node.format(formatter),
            Self::JsNamespaceImportSpecifier(node) => node.format(formatter),
        }
    }
}
