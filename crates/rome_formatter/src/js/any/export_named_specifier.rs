//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyExportNamedSpecifier;
impl ToFormatElement for JsAnyExportNamedSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsExportNamedShorthandSpecifier(node) => node.format(formatter),
            Self::JsExportNamedSpecifier(node) => node.format(formatter),
        }
    }
}
