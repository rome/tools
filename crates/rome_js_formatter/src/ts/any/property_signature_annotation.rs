//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureAnnotation;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureAnnotation;
impl FormatRule<TsAnyPropertySignatureAnnotation> for FormatTsAnyPropertySignatureAnnotation {
    type Options = JsFormatOptions;
    fn format(
        node: &TsAnyPropertySignatureAnnotation,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyPropertySignatureAnnotation::TsTypeAnnotation(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureAnnotation::TsOptionalPropertyAnnotation(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
