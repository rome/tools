//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
impl FormatRule<TsAnyPropertySignatureAnnotation> for FormatTsAnyPropertySignatureAnnotation {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyPropertySignatureAnnotation, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyPropertySignatureAnnotation::TsTypeAnnotation(node) => node.format().fmt(f),
            TsAnyPropertySignatureAnnotation::TsOptionalPropertyAnnotation(node) => {
                node.format().fmt(f)
            }
        }
    }
}
