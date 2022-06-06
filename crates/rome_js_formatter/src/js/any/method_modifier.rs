//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyMethodModifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyMethodModifier;
impl FormatRule<JsAnyMethodModifier> for FormatJsAnyMethodModifier {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyMethodModifier, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyMethodModifier::TsAccessibilityModifier(node) => node.format().fmt(f),
            JsAnyMethodModifier::JsStaticModifier(node) => node.format().fmt(f),
            JsAnyMethodModifier::TsOverrideModifier(node) => node.format().fmt(f),
        }
    }
}
