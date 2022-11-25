//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use rome_js_syntax::JsAnyFunctionBody;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyFunctionBody;
impl FormatRule<JsAnyFunctionBody> for FormatJsAnyFunctionBody {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFunctionBody::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyFunctionBody::JsFunctionBody(node) => node.format().fmt(f),
        }
    }
}
