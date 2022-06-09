//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyFunctionBody;
use crate::prelude::*;
use rome_js_syntax::JsAnyFunctionBody;
impl FormatRule<JsAnyFunctionBody> for FormatJsAnyFunctionBody {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyFunctionBody, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyFunctionBody::JsAnyExpression(node) => node.format().fmt(f),
            JsAnyFunctionBody::JsFunctionBody(node) => node.format().fmt(f),
        }
    }
}
