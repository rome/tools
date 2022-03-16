//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyAttributeValue;
impl ToFormatElement for JsxAnyAttributeValue {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxAnyTag(node) => node.to_format_element(formatter),
            Self::JsxString(node) => node.to_format_element(formatter),
            Self::JsxExpressionAttributeValue(node) => node.to_format_element(formatter),
        }
    }
}
