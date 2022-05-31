//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyLiteralExpression;
use crate::prelude::*;
use rome_js_syntax::JsAnyLiteralExpression;
impl FormatRule<JsAnyLiteralExpression> for FormatJsAnyLiteralExpression {
    type Context = JsFormatContext;
    fn format(
        node: &JsAnyLiteralExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        match node {
            JsAnyLiteralExpression::JsStringLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyLiteralExpression::JsNumberLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyLiteralExpression::JsBigIntLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyLiteralExpression::JsBooleanLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyLiteralExpression::JsNullLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
            JsAnyLiteralExpression::JsRegexLiteralExpression(node) => {
                formatted![formatter, [node.format()]]
            }
        }
    }
}
