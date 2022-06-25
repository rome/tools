//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::TsAnyIndexSignatureModifier;
#[derive(Debug, Clone, Default)]
pub struct FormatTsAnyIndexSignatureModifier;
impl FormatRule<TsAnyIndexSignatureModifier> for FormatTsAnyIndexSignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyIndexSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyIndexSignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            TsAnyIndexSignatureModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}
