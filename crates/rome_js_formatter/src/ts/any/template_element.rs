//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTemplateElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTemplateElement;
impl FormatRule<TsAnyTemplateElement> for FormatTsAnyTemplateElement {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTemplateElement::TsTemplateChunkElement(node) => node.format().fmt(f),
            TsAnyTemplateElement::TsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
