//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyNamedImportSpecifier;
impl ToFormatElement for JsAnyNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsShorthandNamedImportSpecifier(node) => node.format(formatter),
            Self::JsNamedImportSpecifier(node) => node.format(formatter),
            Self::JsUnknownNamedImportSpecifier(node) => node.format(formatter),
        }
    }
}
