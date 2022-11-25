//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyMethodSignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyMethodSignatureModifier;
impl FormatRule<TsAnyMethodSignatureModifier> for FormatTsAnyMethodSignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyMethodSignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyMethodSignatureModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            TsAnyMethodSignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            TsAnyMethodSignatureModifier::TsOverrideModifier(node) => node.format().fmt(f),
            TsAnyMethodSignatureModifier::TsAbstractModifier(node) => node.format().fmt(f),
        }
    }
}
