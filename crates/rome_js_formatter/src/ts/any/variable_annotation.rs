//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyVariableAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
impl FormatRule<TsAnyVariableAnnotation> for FormatTsAnyVariableAnnotation {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyVariableAnnotation,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyVariableAnnotation::TsTypeAnnotation(node) => node.format().format(f),
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node) => node.format().format(f),
        }
    }
}
