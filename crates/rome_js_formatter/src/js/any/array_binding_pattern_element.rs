//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyJsArrayBindingPatternElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyJsArrayBindingPatternElement;
impl FormatRule<AnyJsArrayBindingPatternElement> for FormatAnyJsArrayBindingPatternElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyJsArrayBindingPatternElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyJsArrayBindingPatternElement::JsArrayHole(node) => node.format().fmt(f),
            AnyJsArrayBindingPatternElement::AnyJsBindingPattern(node) => node.format().fmt(f),
            AnyJsArrayBindingPatternElement::JsBindingPatternWithDefault(node) => {
                node.format().fmt(f)
            }
            AnyJsArrayBindingPatternElement::JsArrayBindingPatternRestElement(node) => {
                node.format().fmt(f)
            }
        }
    }
}
