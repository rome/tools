//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyForInOrOfInitializer;
use crate::prelude::*;
use rome_js_syntax::JsAnyForInOrOfInitializer;
impl FormatRule<JsAnyForInOrOfInitializer> for FormatJsAnyForInOrOfInitializer {
    type Options = JsFormatOptions;
    fn format(
        node: &JsAnyForInOrOfInitializer,
        formatter: &Formatter<Self::Options>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
