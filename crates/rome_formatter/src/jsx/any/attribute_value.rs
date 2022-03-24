//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsxAnyAttributeValue;
impl ToFormatElement for JsxAnyAttributeValue {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsxAnyTag(node) => node.format(formatter),
            Self::JsxString(node) => node.format(formatter),
            Self::JsxExpressionAttributeValue(node) => node.format(formatter),
        }
    }
}
