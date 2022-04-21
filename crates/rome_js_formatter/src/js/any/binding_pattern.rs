//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyBindingPattern;
impl Format for JsAnyBindingPattern {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyBinding(node) => node.format(formatter),
            Self::JsArrayBindingPattern(node) => node.format(formatter),
            Self::JsObjectBindingPattern(node) => node.format(formatter),
        }
    }
}
