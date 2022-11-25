//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file. 

use crate::prelude::*;
use rome_js_syntax::JsAnyLiteralExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsAnyLiteralExpression;
impl FormatRule<JsAnyLiteralExpression> for FormatJsAnyLiteralExpression {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsAnyLiteralExpression, f: &mut JsFormatter) -> FormatResult<()> {
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
