//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::JsAnyTemplateElement;
impl FormatRule<JsAnyTemplateElement> for FormatJsAnyTemplateElement {
    fn format(node: &JsAnyTemplateElement, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyTemplateElement::JsTemplateChunkElement(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyTemplateElement::JsTemplateElement(node) => formatted![formatter, node.format()],
        }
    }
}
