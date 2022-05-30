//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyForInitializer;
use crate::prelude::*;
use rome_js_syntax::JsAnyForInitializer;
impl FormatRule<JsAnyForInitializer> for FormatJsAnyForInitializer {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyForInitializer,
        formatter: &Formatter<Self::Context>,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyForInitializer::JsVariableDeclaration(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyForInitializer::JsAnyExpression(node) => formatted![formatter, [node.format()]],
        }
    }
}
