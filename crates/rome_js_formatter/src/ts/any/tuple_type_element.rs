//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTupleTypeElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTupleTypeElement;
impl FormatRule<TsAnyTupleTypeElement> for FormatTsAnyTupleTypeElement {
    type Context = JsFormatContext;
    fn format(node: &TsAnyTupleTypeElement, f: &mut Formatter<Self::Context>) -> FormatResult<()> {
        match node {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(node) => node.format().format(f),
            TsAnyTupleTypeElement::TsType(node) => node.format().format(f),
            TsAnyTupleTypeElement::TsRestTupleTypeElement(node) => node.format().format(f),
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(node) => node.format().format(f),
        }
    }
}
