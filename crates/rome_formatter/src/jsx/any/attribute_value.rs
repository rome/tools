//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyAttributeValue;
impl ToFormatElement for JsxAnyAttributeValue {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxElement(node) => node.to_format_element(formatter),
            Self::JsxStringLiteral(node) => node.to_format_element(formatter),
        }
    }
}
