//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyVariableAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
impl FormatRule<TsAnyVariableAnnotation> for FormatTsAnyVariableAnnotation {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyVariableAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyVariableAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node) => node.format().fmt(f),
        }
    }
}
