//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyMethodModifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyMethodModifier;
impl FormatRule<JsAnyMethodModifier> for FormatJsAnyMethodModifier {
    fn format(node: &JsAnyMethodModifier, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyMethodModifier::TsAccessibilityModifier(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyMethodModifier::JsStaticModifier(node) => formatted![formatter, node.format()],
            JsAnyMethodModifier::TsOverrideModifier(node) => formatted![formatter, node.format()],
        }
    }
}
