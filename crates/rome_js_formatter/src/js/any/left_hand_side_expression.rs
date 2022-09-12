//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsLeftHandSideExpression;
#[derive(Debug, Clone, Default)]
pub struct FormatJsLeftHandSideExpression;
impl FormatRule<JsLeftHandSideExpression> for FormatJsLeftHandSideExpression {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsLeftHandSideExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsLeftHandSideExpression::JsStaticMemberExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsComputedMemberExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsNewExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsCallExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsxElement(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsxSelfClosingElement(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsxFragment(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsTemplate(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsArrayExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsParenthesizedExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsObjectExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsClassExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsFunctionExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsIdentifierExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsAnyLiteralExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::TsInstantiationExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsThisExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsPrivateName(node) => node.format().fmt(f),
            JsLeftHandSideExpression::TsNonNullAssertionExpression(node) => node.format().fmt(f),
        }
    }
}
