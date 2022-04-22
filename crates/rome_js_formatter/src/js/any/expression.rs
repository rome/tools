//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{Format, Formatter};
use rome_formatter::{FormatElement, FormatResult};
use rome_js_syntax::JsAnyExpression;
impl Format for JsAnyExpression {
    fn format(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            Self::JsAnyLiteralExpression(node) => node.format(formatter),
            Self::ImportMeta(node) => node.format(formatter),
            Self::JsArrayExpression(node) => node.format(formatter),
            Self::JsArrowFunctionExpression(node) => node.format(formatter),
            Self::JsAssignmentExpression(node) => node.format(formatter),
            Self::JsAwaitExpression(node) => node.format(formatter),
            Self::JsBinaryExpression(node) => node.format(formatter),
            Self::JsCallExpression(node) => node.format(formatter),
            Self::JsClassExpression(node) => node.format(formatter),
            Self::JsComputedMemberExpression(node) => node.format(formatter),
            Self::JsConditionalExpression(node) => node.format(formatter),
            Self::JsFunctionExpression(node) => node.format(formatter),
            Self::JsIdentifierExpression(node) => node.format(formatter),
            Self::JsImportCallExpression(node) => node.format(formatter),
            Self::JsInExpression(node) => node.format(formatter),
            Self::JsInstanceofExpression(node) => node.format(formatter),
            Self::JsLogicalExpression(node) => node.format(formatter),
            Self::JsNewExpression(node) => node.format(formatter),
            Self::JsObjectExpression(node) => node.format(formatter),
            Self::JsParenthesizedExpression(node) => node.format(formatter),
            Self::JsPostUpdateExpression(node) => node.format(formatter),
            Self::JsPreUpdateExpression(node) => node.format(formatter),
            Self::JsSequenceExpression(node) => node.format(formatter),
            Self::JsStaticMemberExpression(node) => node.format(formatter),
            Self::JsSuperExpression(node) => node.format(formatter),
            Self::JsThisExpression(node) => node.format(formatter),
            Self::JsUnaryExpression(node) => node.format(formatter),
            Self::JsUnknownExpression(node) => node.format(formatter),
            Self::JsYieldExpression(node) => node.format(formatter),
            Self::NewTarget(node) => node.format(formatter),
            Self::JsTemplate(node) => node.format(formatter),
            Self::TsTypeAssertionExpression(node) => node.format(formatter),
            Self::TsAsExpression(node) => node.format(formatter),
            Self::TsNonNullAssertionExpression(node) => node.format(formatter),
            Self::JsxTagExpression(node) => node.format(formatter),
        }
    }
}
