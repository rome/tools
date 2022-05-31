//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyMethodSignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyMethodSignatureModifier;
impl FormatRule<TsAnyMethodSignatureModifier> for FormatTsAnyMethodSignatureModifier {
    type Context = JsFormatContext;
    fn format(
        node: &TsAnyMethodSignatureModifier,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            TsAnyMethodSignatureModifier::TsAccessibilityModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyMethodSignatureModifier::JsStaticModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyMethodSignatureModifier::TsOverrideModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            TsAnyMethodSignatureModifier::TsAbstractModifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
