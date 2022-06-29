//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyArrayBindingPatternElement;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyArrayBindingPatternElement;
impl FormatRule<JsAnyArrayBindingPatternElement> for FormatJsAnyArrayBindingPatternElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyArrayBindingPatternElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrayBindingPatternElement::JsArrayHole(node) => node.format().fmt(f),
            JsAnyArrayBindingPatternElement::JsAnyBindingPattern(node) => node.format().fmt(f),
            JsAnyArrayBindingPatternElement::JsBindingPatternWithDefault(node) => {
                node.format().fmt(f)
            }
            JsAnyArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                node.format().fmt(f)
            }
        }
    }
}
