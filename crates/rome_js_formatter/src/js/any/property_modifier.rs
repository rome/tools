//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyPropertyModifier;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyPropertyModifier;
impl FormatRule<JsAnyPropertyModifier> for FormatJsAnyPropertyModifier {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyPropertyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyPropertyModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            JsAnyPropertyModifier::JsStaticModifier(node) => node.format().fmt(f),
            JsAnyPropertyModifier::TsReadonlyModifier(node) => node.format().fmt(f),
            JsAnyPropertyModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}
