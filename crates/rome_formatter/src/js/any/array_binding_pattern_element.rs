//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyArrayBindingPatternElement;
impl ToFormatElement for JsAnyArrayBindingPatternElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsArrayHole(node) => node.format(formatter),
            Self::JsAnyBindingPattern(node) => node.format(formatter),
            Self::JsBindingPatternWithDefault(node) => node.format(formatter),
            Self::JsArrayBindingPatternRestElement(node) => node.format(formatter),
        }
    }
}
