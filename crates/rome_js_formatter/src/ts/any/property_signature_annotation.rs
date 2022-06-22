//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyPropertySignatureAnnotation;
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
