//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, FormatElement, FormatResult, Formatter};
use rome_js_syntax::JsAnyArrayElement;
impl Format for JsAnyArrayElement {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyExpression(node) => node.format(formatter),
            Self::JsSpread(node) => node.format(formatter),
            Self::JsArrayHole(node) => node.format(formatter),
        }
    }
}
