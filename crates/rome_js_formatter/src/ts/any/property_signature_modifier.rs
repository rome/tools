//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureModifier;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsAnyPropertySignatureModifier;
impl FormatRule<TsAnyPropertySignatureModifier> for FormatTsAnyPropertySignatureModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &TsAnyPropertySignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            TsAnyPropertySignatureModifier::TsDeclareModifier(node) => node.format().fmt(f),
            TsAnyPropertySignatureModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            TsAnyPropertySignatureModifier::JsStaticModifier(node) => node.format().fmt(f),
            TsAnyPropertySignatureModifier::TsReadonlyModifier(node) => node.format().fmt(f),
            TsAnyPropertySignatureModifier::TsOverrideModifier(node) => node.format().fmt(f),
            TsAnyPropertySignatureModifier::TsAbstractModifier(node) => node.format().fmt(f),
        }
    }
}
