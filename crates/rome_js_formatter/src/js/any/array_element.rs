//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyArrayElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyArrayElement;
impl FormatRule<JsAnyArrayElement> for FormatJsAnyArrayElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyArrayElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyArrayElement::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyArrayElement::JsSpread(node) => node.format().fmt(f),
            JsAnyArrayElement::JsArrayHole(node) => node.format().fmt(f),
        }
    }
}
