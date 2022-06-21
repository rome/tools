//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyForInOrOfInitializer;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyForInOrOfInitializer;
impl FormatRule<JsAnyForInOrOfInitializer> for FormatJsAnyForInOrOfInitializer {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyForInOrOfInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyForInOrOfInitializer::JsAnyAssignmentPattern(node) => node.format().fmt(f),
            JsAnyForInOrOfInitializer::JsForVariableDeclaration(node) => node.format().fmt(f),
        }
    }
}
