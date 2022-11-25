//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyVariableAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyVariableAnnotation;
impl FormatRule<TsAnyVariableAnnotation> for FormatTsAnyVariableAnnotation {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyVariableAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyVariableAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyVariableAnnotation::TsDefiniteVariableAnnotation(node) => node.format().fmt(f),
        }
    }
}
