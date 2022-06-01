//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyPropertyModifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyPropertyModifier;
impl FormatRule<JsAnyPropertyModifier> for FormatJsAnyPropertyModifier {
    type Context = JsFormatContext;
    fn format(node: &JsAnyPropertyModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyPropertyModifier::TsAccessibilityModifier(node) => node.format().format(f),
            JsAnyPropertyModifier::JsStaticModifier(node) => node.format().format(f),
            JsAnyPropertyModifier::TsReadonlyModifier(node) => node.format().format(f),
            JsAnyPropertyModifier::TsOverrideModifier(node) => node.format().format(f),
        }
    }
}
