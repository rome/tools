//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayBindingPatternElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayBindingPatternElement;
impl FormatRule<JsAnyArrayBindingPatternElement> for FormatJsAnyArrayBindingPatternElement {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyArrayBindingPatternElement,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyArrayBindingPatternElement::JsArrayHole(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
