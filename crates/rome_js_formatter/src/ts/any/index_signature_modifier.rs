//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyIndexSignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyIndexSignatureModifier;
impl FormatRule<TsAnyIndexSignatureModifier> for FormatTsAnyIndexSignatureModifier {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyIndexSignatureModifier,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyIndexSignatureModifier::JsStaticModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyIndexSignatureModifier::TsReadonlyModifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
