//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyFunctionBody;
use crate::prelude::*;
use rome_js_syntax::JsAnyFunctionBody;
impl FormatRule<JsAnyFunctionBody> for FormatJsAnyFunctionBody {
    fn format(node: &JsAnyFunctionBody, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyFunctionBody::JsAnyExpression(node) => formatted![formatter, node.format()],
            JsAnyFunctionBody::JsFunctionBody(node) => formatted![formatter, node.format()],
        }
    }
}
