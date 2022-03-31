//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyTag;
impl ToFormatElement for JsxAnyTag {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxElement(node) => node.to_format_element(formatter),
            Self::JsxSelfClosingElement(node) => node.to_format_element(formatter),
            Self::JsxFragment(node) => node.to_format_element(formatter),
        }
    }
}
