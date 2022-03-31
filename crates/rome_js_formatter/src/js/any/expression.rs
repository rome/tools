//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsAnyExpression;
impl ToFormatElement for JsAnyExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyLiteralExpression(node) => node.to_format_element(formatter),
            Self::ImportMeta(node) => node.to_format_element(formatter),
            Self::JsArrayExpression(node) => node.to_format_element(formatter),
            Self::JsArrowFunctionExpression(node) => node.to_format_element(formatter),
            Self::JsAssignmentExpression(node) => node.to_format_element(formatter),
            Self::JsAwaitExpression(node) => node.to_format_element(formatter),
            Self::JsBinaryExpression(node) => node.to_format_element(formatter),
            Self::JsCallExpression(node) => node.to_format_element(formatter),
            Self::JsClassExpression(node) => node.to_format_element(formatter),
            Self::JsComputedMemberExpression(node) => node.to_format_element(formatter),
            Self::JsConditionalExpression(node) => node.to_format_element(formatter),
            Self::JsFunctionExpression(node) => node.to_format_element(formatter),
            Self::JsIdentifierExpression(node) => node.to_format_element(formatter),
            Self::JsImportCallExpression(node) => node.to_format_element(formatter),
            Self::JsInExpression(node) => node.to_format_element(formatter),
            Self::JsInstanceofExpression(node) => node.to_format_element(formatter),
            Self::JsLogicalExpression(node) => node.to_format_element(formatter),
            Self::JsNewExpression(node) => node.to_format_element(formatter),
            Self::JsObjectExpression(node) => node.to_format_element(formatter),
            Self::JsParenthesizedExpression(node) => node.to_format_element(formatter),
            Self::JsPostUpdateExpression(node) => node.to_format_element(formatter),
            Self::JsPreUpdateExpression(node) => node.to_format_element(formatter),
            Self::JsSequenceExpression(node) => node.to_format_element(formatter),
            Self::JsStaticMemberExpression(node) => node.to_format_element(formatter),
            Self::JsSuperExpression(node) => node.to_format_element(formatter),
            Self::JsThisExpression(node) => node.to_format_element(formatter),
            Self::JsUnaryExpression(node) => node.to_format_element(formatter),
            Self::JsUnknownExpression(node) => node.to_format_element(formatter),
            Self::JsYieldExpression(node) => node.to_format_element(formatter),
            Self::NewTarget(node) => node.to_format_element(formatter),
            Self::JsTemplate(node) => node.to_format_element(formatter),
            Self::TsTypeAssertionExpression(node) => node.to_format_element(formatter),
            Self::TsAsExpression(node) => node.to_format_element(formatter),
            Self::TsNonNullAssertionExpression(node) => node.to_format_element(formatter),
            Self::JsxTagExpression(node) => node.to_format_element(formatter),
        }
    }
}
