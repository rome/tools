//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsAnyFunctionBody;
#[derive(Debug, Clone, Default)]
pub struct FormatJsAnyFunctionBody;
impl FormatRule<JsAnyFunctionBody> for FormatJsAnyFunctionBody {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFunctionBody::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyFunctionBody::JsFunctionBody(node) => node.format().fmt(f),
        }
    }
}
