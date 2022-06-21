//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyForInitializer;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyForInitializer;
impl FormatRule<JsAnyForInitializer> for FormatJsAnyForInitializer {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyForInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyForInitializer::JsVariableDeclaration(node) => node.format().fmt(f),
            JsAnyForInitializer::JsAnyExpression(node) => node.format().fmt(f),
        }
    }
}
