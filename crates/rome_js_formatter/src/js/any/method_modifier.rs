//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyMethodModifier;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyMethodModifier;
impl FormatRule<JsAnyMethodModifier> for FormatJsAnyMethodModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyMethodModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyMethodModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            JsAnyMethodModifier::JsStaticModifier(node) => node.format().fmt(f),
            JsAnyMethodModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}
