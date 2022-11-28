//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::AnyTsVariableAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyTsVariableAnnotation;
impl FormatRule<AnyTsVariableAnnotation> for FormatAnyTsVariableAnnotation {
    type Context = JsFormatContext;
    fn fmt(&self, node: &AnyTsVariableAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            AnyTsVariableAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            AnyTsVariableAnnotation::TsDefiniteVariableAnnotation(node) => node.format().fmt(f),
        }
    }
}
