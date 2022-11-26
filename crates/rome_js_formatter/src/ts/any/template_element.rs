//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyTemplateElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyTemplateElement;
impl FormatRule<TsAnyTemplateElement> for FormatTsAnyTemplateElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyTemplateElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTemplateElement::TsTemplateChunkElement(node) => node.format().fmt(f),
            TsAnyTemplateElement::TsTemplateElement(node) => node.format().fmt(f),
        }
    }
}
