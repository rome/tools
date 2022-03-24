//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrayBindingPatternElement;
impl ToFormatElement for JsAnyArrayBindingPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsArrayHole(node) => node.to_format_element(formatter),
            Self::JsAnyBindingPattern(node) => node.to_format_element(formatter),
            Self::JsBindingPatternWithDefault(node) => node.to_format_element(formatter),
            Self::JsArrayBindingPatternRestElement(node) => node.to_format_element(formatter),
        }
    }
}
