//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExpression;
use crate::prelude::*;
use rome_js_syntax::JsAnyExpression;
impl FormatRule<JsAnyExpression> for FormatJsAnyExpression {
    fn format(node: &JsAnyExpression, formatter: &Formatter) -> FormatResult<FormatElement> {
        match node {
            JsAnyExpression::JsAnyLiteralExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::ImportMeta(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsArrayExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsArrowFunctionExpression(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyExpression::JsAssignmentExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsAwaitExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsBinaryExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsCallExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsClassExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsComputedMemberExpression(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyExpression::JsConditionalExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsFunctionExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsIdentifierExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsImportCallExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsInExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsInstanceofExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsLogicalExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsNewExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsObjectExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsParenthesizedExpression(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyExpression::JsPostUpdateExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsPreUpdateExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsSequenceExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsStaticMemberExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsSuperExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsThisExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsUnaryExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsUnknownExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsYieldExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::NewTarget(node) => formatted![formatter, node.format()],
            JsAnyExpression::JsTemplate(node) => formatted![formatter, node.format()],
            JsAnyExpression::TsTypeAssertionExpression(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyExpression::TsAsExpression(node) => formatted![formatter, node.format()],
            JsAnyExpression::TsNonNullAssertionExpression(node) => {
                formatted![formatter, node.format()]
            }
            JsAnyExpression::JsxTagExpression(node) => formatted![formatter, node.format()],
        }
    }
}
