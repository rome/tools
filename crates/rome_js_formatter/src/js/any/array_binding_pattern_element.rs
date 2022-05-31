//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyArrayBindingPatternElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyArrayBindingPatternElement;
impl FormatRule<JsAnyArrayBindingPatternElement> for FormatJsAnyArrayBindingPatternElement {
    type Context = JsFormatContext;
    fn format(node: &JsAnyArrayBindingPatternElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrayBindingPatternElement::JsArrayHole(node) => node.format().format(f),
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(node) => node.format().format(f),
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(node) => {
                node.format().format(f)
            }
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                node.format().format(f)
            }
        }
    }
}
