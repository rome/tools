//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::generated::FormatJsAnyExpression;
use crate::prelude::*;
use rome_js_syntax::JsAnyExpression;
impl FormatRule<JsAnyExpression> for FormatJsAnyExpression {
    type Context = JsFormatContext;
    fn fmt(node: &JsAnyExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsAnyExpression::JsAnyLiteralExpression(node) => node.format().fmt(f),
            JsAnyExpression::ImportMeta(node) => node.format().fmt(f),
            JsAnyExpression::JsArrayExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsArrowFunctionExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsAssignmentExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsAwaitExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsBinaryExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsCallExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsClassExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsComputedMemberExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsConditionalExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsFunctionExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsIdentifierExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsImportCallExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsInExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsInstanceofExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsLogicalExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsNewExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsObjectExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsParenthesizedExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsPostUpdateExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsPreUpdateExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsSequenceExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsStaticMemberExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsSuperExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsThisExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsUnaryExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsUnknownExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsYieldExpression(node) => node.format().fmt(f),
            JsAnyExpression::NewTarget(node) => node.format().fmt(f),
            JsAnyExpression::JsTemplate(node) => node.format().fmt(f),
            JsAnyExpression::TsTypeAssertionExpression(node) => node.format().fmt(f),
            JsAnyExpression::TsAsExpression(node) => node.format().fmt(f),
            JsAnyExpression::TsNonNullAssertionExpression(node) => node.format().fmt(f),
            JsAnyExpression::JsxTagExpression(node) => node.format().fmt(f),
        }
    }
}
