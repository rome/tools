//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyForInitializer;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyForInitializer;
impl FormatRule<JsAnyForInitializer> for FormatJsAnyForInitializer {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyForInitializer, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyForInitializer::JsVariableDeclaration(node) => node.format().fmt(f),
            JsAnyForInitializer::JsAnyExpression(node) => node.format().fmt(f),
        }
    }
}
