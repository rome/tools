//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyBindingPattern;
impl ToFormatElement for JsAnyBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyBinding(node) => node.to_format_element(formatter),
            Self::JsArrayBindingPattern(node) => node.to_format_element(formatter),
            Self::JsObjectBindingPattern(node) => node.to_format_element(formatter),
        }
    }
}
