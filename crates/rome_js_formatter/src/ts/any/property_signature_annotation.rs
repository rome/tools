//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyPropertySignatureAnnotation;
impl FormatRule<TsAnyPropertySignatureAnnotation> for FormatTsAnyPropertySignatureAnnotation {
    type Context = JsFormatContext;
    fn fmt(
        &self,
        node: &TsAnyPropertySignatureAnnotation,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        match node {
            TsAnyPropertySignatureAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyPropertySignatureAnnotation::TsOptionalPropertyAnnotation(node) => {
                node.format().fmt(f)
            }
        }
    }
}
