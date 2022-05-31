//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureModifier;
impl FormatRule<TsAnyPropertySignatureModifier> for FormatTsAnyPropertySignatureModifier {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyPropertySignatureModifier,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        match node {
            TsAnyPropertySignatureModifier::TsDeclareModifier(node) => node.format().format(f),
            TsAnyPropertySignatureModifier::TsAccessibilityModifier(node) => {
                node.format().format(f)
            }
            TsAnyPropertySignatureModifier::JsStaticModifier(node) => node.format().format(f),
            TsAnyPropertySignatureModifier::TsReadonlyModifier(node) => node.format().format(f),
            TsAnyPropertySignatureModifier::TsOverrideModifier(node) => node.format().format(f),
            TsAnyPropertySignatureModifier::TsAbstractModifier(node) => node.format().format(f),
        }
    }
}
