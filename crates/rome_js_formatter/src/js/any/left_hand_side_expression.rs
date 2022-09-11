//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::prelude::*;
use rome_js_syntax::JsLeftHandSideExpression;
#[derive(Debug, Clone, Default)]
pub struct FormatJsLeftHandSideExpression;
impl FormatRule<JsLeftHandSideExpression> for FormatJsLeftHandSideExpression {
    type Context = JsFormatContext;
    fn fmt(&self, node: &JsLeftHandSideExpression, f: &mut JsFormatter) -> FormatResult<()> {
        match node {
            JsLeftHandSideExpression::JsParenthesizedExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsStaticMemberExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsComputedMemberExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::JsIdentifierExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::TsNonNullAssertionExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::TsTypeAssertionExpression(node) => node.format().fmt(f),
            JsLeftHandSideExpression::TsAsExpression(node) => node.format().fmt(f),
        }
    }
}
