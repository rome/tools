//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyForInOrOfInitializer;
use crate::prelude::*;
use rome_js_syntax::JsAnyForInOrOfInitializer;
impl FormatRule<JsAnyForInOrOfInitializer> for FormatJsAnyForInOrOfInitializer {
    fn format(
        node: &JsAnyForInOrOfInitializer,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(node) => {
                formatted![formatter, node.format()]
            }
        }
    }
}
