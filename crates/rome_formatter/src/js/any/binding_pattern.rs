//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{prelude::*, FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyBindingPattern;
impl ToFormatElement for JsAnyBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyBinding(node) => node.format(formatter),
            Self::JsArrayBindingPattern(node) => node.format(formatter),
            Self::JsObjectBindingPattern(node) => node.format(formatter),
        }
    }
}
