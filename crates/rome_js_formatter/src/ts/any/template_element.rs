//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTemplateElement;
impl FormatRule<TsAnyTemplateElement> for FormatTsAnyTemplateElement {
    fn format(node: &TsAnyTemplateElement, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            TsAnyTemplateElement::TsTemplateChunkElement(node) => {
                formatted![formatter, node.format()]
            }
            TsAnyTemplateElement::TsTemplateElement(node) => formatted![formatter, node.format()],
        }
    }
}
