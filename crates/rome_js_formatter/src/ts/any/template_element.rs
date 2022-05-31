//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTemplateElement;
impl FormatRule<TsAnyTemplateElement> for FormatTsAnyTemplateElement {
    type Context = JsFormatContext;
    fn format(node: &TsAnyTemplateElement, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            TsAnyTemplateElement::TsTemplateChunkElement(node) => node.format().format(f),
            TsAnyTemplateElement::TsTemplateElement(node) => node.format().format(f),
        }
    }
}
