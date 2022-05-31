//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyForInOrOfInitializer;
use crate::prelude::*;
use rome_js_syntax::JsAnyForInOrOfInitializer;
impl FormatRule<JsAnyForInOrOfInitializer> for FormatJsAnyForInOrOfInitializer {
    type Context = JsFormatContext;
    fn format(node: &JsAnyForInOrOfInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(node) => node.format().format(f),
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(node) => node.format().format(f),
        }
    }
}
