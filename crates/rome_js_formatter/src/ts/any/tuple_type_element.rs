//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyTupleTypeElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyTupleTypeElement;
impl FormatRule<TsAnyTupleTypeElement> for FormatTsAnyTupleTypeElement {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsType(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsRestTupleTypeElement(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(node) => node.format().fmt(f),
        }
    }
}
