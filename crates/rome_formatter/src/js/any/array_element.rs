//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrayElement;
impl ToFormatElement for JsAnyArrayElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyExpression(node) => node.to_format_element(formatter),
            Self::JsSpread(node) => node.to_format_element(formatter),
            Self::JsArrayHole(node) => node.to_format_element(formatter),
        }
    }
}
