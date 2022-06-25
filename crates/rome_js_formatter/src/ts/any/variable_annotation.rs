//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyVariableAnnotation;
impl FormatRule<TsAnyVariableAnnotation> for FormatTsAnyVariableAnnotation {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyVariableAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyVariableAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node) => node.format().fmt(f),
        }
    }
}
