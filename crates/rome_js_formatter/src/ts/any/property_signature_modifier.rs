//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureModifier;
impl FormatRule<TsAnyPropertySignatureModifier> for FormatTsAnyPropertySignatureModifier {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyPropertySignatureModifier,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyPropertySignatureModifier::TsDeclareModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureModifier::TsAccessibilityModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureModifier::JsStaticModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureModifier::TsReadonlyModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureModifier::TsOverrideModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyPropertySignatureModifier::TsAbstractModifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
