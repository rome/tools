//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyArrayBindingPatternElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyArrayBindingPatternElement;
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
