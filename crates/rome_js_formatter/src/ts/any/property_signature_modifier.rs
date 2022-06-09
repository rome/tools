//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatTsAnyPropertySignatureModifier;
use crate::prelude::*;
use rome_js_syntax::TsAnyPropertySignatureModifier;
impl FormatRule<TsAnyPropertySignatureModifier> for FormatTsAnyPropertySignatureModifier {
    type Context = JsFormatContext;
    fn fmt(node: &TsAnyPropertySignatureModifier, f: &mut JsFormatter) -> FormatResult<()> {
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
