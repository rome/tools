//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyMethodModifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyMethodModifier;
impl FormatRule<JsAnyMethodModifier> for FormatJsAnyMethodModifier {
    type Context = JsFormatContext;
    fn format(node: &JsAnyMethodModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyMethodModifier::TsAccessibilityModifier(node) => node.format().format(f),
            JsAnyMethodModifier::JsStaticModifier(node) => node.format().format(f),
            JsAnyMethodModifier::TsOverrideModifier(node) => node.format().format(f),
        }
    }
}
