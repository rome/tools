//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyIndexSignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyIndexSignatureModifier;
impl FormatRule<TsAnyIndexSignatureModifier> for FormatTsAnyIndexSignatureModifier {
    type Context = JsFormatContext;
    fn format(node: &TsAnyIndexSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyIndexSignatureModifier::JsStaticModifier(node) => node.format().format(f),
            TsAnyIndexSignatureModifier::TsReadonlyModifier(node) => node.format().format(f),
        }
    }
}
