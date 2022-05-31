//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyTemplateElement;
impl FormatRule<JsAnyTemplateElement> for FormatJsAnyTemplateElement {
    type Context = JsFormatContext;
    fn format(node: &JsAnyTemplateElement, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            JsAnyTemplateElement::JsTemplateChunkElement(node) => node.format().format(f),
            JsAnyTemplateElement::JsTemplateElement(node) => node.format().format(f),
        }
    }
}
