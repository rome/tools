//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyForInitializer;
use crate::prelude::*;
use rome_js_syntax::JsAnyForInitializer;
impl FormatRule<JsAnyForInitializer> for FormatJsAnyForInitializer {
    type Context = JsFormatContext;
    fn format(node: &JsAnyForInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyForInitializer::JsVariableDeclaration(node) => node.format().format(f),
            JsAnyForInitializer::JsAnyExpression(node) => node.format().format(f),
        }
    }
}
