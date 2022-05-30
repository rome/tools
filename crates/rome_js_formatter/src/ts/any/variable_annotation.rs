//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyVariableAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
impl FormatRule<TsAnyVariableAnnotation> for FormatTsAnyVariableAnnotation {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyVariableAnnotation,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyVariableAnnotation::TsTypeAnnotation(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
