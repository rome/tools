//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTemplateElement;
impl FormatRule<TsAnyTemplateElement> for FormatTsAnyTemplateElement {
    type Context = JsFormatContext;
    fn format(node: &TsAnyTemplateElement, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match node {
            TsAnyTemplateElement::TsTemplateChunkElement(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTemplateElement::TsTemplateElement(node) => formatted![formatter, [node.format()]],
        }
    }
}
