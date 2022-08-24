use crate::prelude::*;
use crate::utils::{needs_binary_like_parentheses, JsAnyBinaryLikeExpression};

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsLogicalExpression, JsSyntaxNode};
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsLogicalExpression;

impl FormatNodeRule<JsLogicalExpression> for FormatJsLogicalExpression {
    fn fmt_fields(
        &self,
        node: &JsLogicalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        JsAnyBinaryLikeExpression::JsLogicalExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsLogicalExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsLogicalExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if let Some(parent) = JsLogicalExpression::cast(parent.clone()) {
            return if parent.operator() != self.operator() {
                true
            } else {
                // TODO: Parentheses should never be needed for the same operators BUT this is causing a re-formatting
                // issue if a logical expression has an in-balanced tree. See issue-7024.js for a test case..
                // The way prettier solves this is by re-balancing the tree before formatting, something, Rome' doesn't yet support.
                Ok(self.syntax()) != parent.left().map(AstNode::into_syntax).as_ref()
            };
        }

        needs_binary_like_parentheses(&JsAnyBinaryLikeExpression::from(self.clone()), parent)
    }
}

impl ExpressionNode for JsLogicalExpression {
    #[inline]
    fn resolve(&self) -> JsAnyExpression {
        self.clone().into()
    }

    #[inline]
    fn into_resolved(self) -> JsAnyExpression {
        self.into()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{JsLogicalExpression, SourceType};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class X extends (a && b) {}", JsLogicalExpression);

        assert_needs_parentheses!("(a && b) as number", JsLogicalExpression);
        assert_needs_parentheses!("<number>(a && b)", JsLogicalExpression);
        assert_needs_parentheses!("!(a && b)", JsLogicalExpression);
        assert_needs_parentheses!("await (a && b)", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)!", JsLogicalExpression);

        assert_needs_parentheses!("(a && b)()", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)?.()", JsLogicalExpression);
        assert_needs_parentheses!("new (a && b)()", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)`template`", JsLogicalExpression);
        assert_needs_parentheses!("[...(a && b)]", JsLogicalExpression);
        assert_needs_parentheses!("({...(a && b)})", JsLogicalExpression);
        assert_needs_parentheses!(
            "<test {...(a && b)} />",
            JsLogicalExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(a && b)}</test>",
            JsLogicalExpression,
            SourceType::tsx()
        );

        assert_needs_parentheses!("(a && b).member", JsLogicalExpression);
        assert_needs_parentheses!("(a && b)[member]", JsLogicalExpression);
        assert_not_needs_parentheses!("object[a && b]", JsLogicalExpression);

        assert_needs_parentheses!("(a && b) || c", JsLogicalExpression[1]);
        assert_needs_parentheses!("(a && b) in c", JsLogicalExpression);
        assert_needs_parentheses!("(a && b) instanceof c", JsLogicalExpression);
        assert_needs_parentheses!("(a && b) + c", JsLogicalExpression);

        assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[0]);
        assert_not_needs_parentheses!("a && b && c", JsLogicalExpression[1]);
    }
}
