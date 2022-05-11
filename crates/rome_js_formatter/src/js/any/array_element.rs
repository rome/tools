//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayElement;
impl FormatRule<JsAnyArrayElement> for FormatJsAnyArrayElement {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyArrayElement,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyArrayElement::JsAnyExpression(node) => formatted![formatter, [node.format()]],
            JsAnyArrayElement::JsSpread(node) => formatted![formatter, [node.format()]],
            JsAnyArrayElement::JsArrayHole(node) => formatted![formatter, [node.format()]],
        }
    }
}
