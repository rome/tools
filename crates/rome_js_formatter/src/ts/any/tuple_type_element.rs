//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTupleTypeElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTupleTypeElement;
impl FormatRule<TsAnyTupleTypeElement> for FormatTsAnyTupleTypeElement {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyTupleTypeElement, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsType(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsRestTupleTypeElement(node) => node.format().fmt(f),
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(node) => node.format().fmt(f),
        }
    }
}
