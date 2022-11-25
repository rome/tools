//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyTemplateElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyTemplateElement;
impl FormatRule<JsAnyTemplateElement> for FormatJsAnyTemplateElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyTemplateElement::JsTemplateChunkElement(node) => node.format().fmt(f),
            JsAnyTemplateElement::JsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
