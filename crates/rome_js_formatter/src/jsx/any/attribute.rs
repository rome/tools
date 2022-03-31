//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyAttribute;
impl ToFormatElement for JsxAnyAttribute {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxAttribute(node) => node.to_format_element(formatter),
            Self::JsxSpreadAttribute(node) => node.to_format_element(formatter),
        }
    }
}
