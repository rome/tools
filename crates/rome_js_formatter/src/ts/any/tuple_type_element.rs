//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyTupleTypeElement;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyTupleTypeElement;
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
