//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyIndexSignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyIndexSignatureModifier;
impl FormatRule<TsAnyIndexSignatureModifier> for FormatTsAnyIndexSignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyIndexSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyIndexSignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            TsAnyIndexSignatureModifier::TsReadonlyModifier(node) => node.format().fmt(f),
        }
    }
}
