//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyLiteralExpression;
use crate::prelude::*;
use rome_js_syntax::JsAnyLiteralExpression;
impl FormatRule<JsAnyLiteralExpression> for FormatJsAnyLiteralExpression {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyLiteralExpression::JsStringLiteralExpression(node) => node.format().fmt(f),
            JsAnyLiteralExpression::JsNumberLiteralExpression(node) => node.format().fmt(f),
            JsAnyLiteralExpression::JsBigIntLiteralExpression(node) => node.format().fmt(f),
            JsAnyLiteralExpression::JsBooleanLiteralExpression(node) => node.format().fmt(f),
            JsAnyLiteralExpression::JsNullLiteralExpression(node) => node.format().fmt(f),
            JsAnyLiteralExpression::JsRegexLiteralExpression(node) => node.format().fmt(f),
        }
    }
}
