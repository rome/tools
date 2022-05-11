//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyPropertyModifier;
use crate::prelude::*;
use rome_js_syntax::JsAnyPropertyModifier;
impl FormatRule<JsAnyPropertyModifier> for FormatJsAnyPropertyModifier {
    fn format(node: &JsAnyPropertyModifier, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyPropertyModifier::TsAccessibilityModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyPropertyModifier::JsStaticModifier(node) => formatted![formatter, [node.format()]],
            JsAnyPropertyModifier::TsReadonlyModifier(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyPropertyModifier::TsOverrideModifier(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
