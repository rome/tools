//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyTupleTypeElement;
use crate::prelude::*;
use rome_js_syntax::TsAnyTupleTypeElement;
impl FormatRule<TsAnyTupleTypeElement> for FormatTsAnyTupleTypeElement {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyTupleTypeElement,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyTupleTypeElement::TsNamedTupleTypeElement(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTupleTypeElement::TsType(node) => formatted![formatter, [node.format()]],
            TsAnyTupleTypeElement::TsRestTupleTypeElement(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyTupleTypeElement::TsOptionalTupleTypeElement(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
