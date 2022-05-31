//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExpression;
use crate::prelude::*;
use rome_js_syntax::JsAnyExpression;
impl FormatRule<JsAnyExpression> for FormatJsAnyExpression {
    type Context = JsFormatContext;
    fn format(node: &JsAnyExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExpression::JsAnyLiteralExpression(node) => node.format().format(f),
            JsAnyExpression::ImportMeta(node) => node.format().format(f),
            JsAnyExpression::JsArrayExpression(node) => node.format().format(f),
            JsAnyExpression::JsArrowFunctionExpression(node) => node.format().format(f),
            JsAnyExpression::JsAssignmentExpression(node) => node.format().format(f),
            JsAnyExpression::JsAwaitExpression(node) => node.format().format(f),
            JsAnyExpression::JsBinaryExpression(node) => node.format().format(f),
            JsAnyExpression::JsCallExpression(node) => node.format().format(f),
            JsAnyExpression::JsClassExpression(node) => node.format().format(f),
            JsAnyExpression::JsComputedMemberExpression(node) => node.format().format(f),
            JsAnyExpression::JsConditionalExpression(node) => node.format().format(f),
            JsAnyExpression::JsFunctionExpression(node) => node.format().format(f),
            JsAnyExpression::JsIdentifierExpression(node) => node.format().format(f),
            JsAnyExpression::JsImportCallExpression(node) => node.format().format(f),
            JsAnyExpression::JsInExpression(node) => node.format().format(f),
            JsAnyExpression::JsInstanceofExpression(node) => node.format().format(f),
            JsAnyExpression::JsLogicalExpression(node) => node.format().format(f),
            JsAnyExpression::JsNewExpression(node) => node.format().format(f),
            JsAnyExpression::JsObjectExpression(node) => node.format().format(f),
            JsAnyExpression::JsParenthesizedExpression(node) => node.format().format(f),
            JsAnyExpression::JsPostUpdateExpression(node) => node.format().format(f),
            JsAnyExpression::JsPreUpdateExpression(node) => node.format().format(f),
            JsAnyExpression::JsSequenceExpression(node) => node.format().format(f),
            JsAnyExpression::JsStaticMemberExpression(node) => node.format().format(f),
            JsAnyExpression::JsSuperExpression(node) => node.format().format(f),
            JsAnyExpression::JsThisExpression(node) => node.format().format(f),
            JsAnyExpression::JsUnaryExpression(node) => node.format().format(f),
            JsAnyExpression::JsUnknownExpression(node) => node.format().format(f),
            JsAnyExpression::JsYieldExpression(node) => node.format().format(f),
            JsAnyExpression::NewTarget(node) => node.format().format(f),
            JsAnyExpression::JsTemplate(node) => node.format().format(f),
            JsAnyExpression::TsTypeAssertionExpression(node) => node.format().format(f),
            JsAnyExpression::TsAsExpression(node) => node.format().format(f),
            JsAnyExpression::TsNonNullAssertionExpression(node) => node.format().format(f),
            JsAnyExpression::JsxTagExpression(node) => node.format().format(f),
        }
    }
}
