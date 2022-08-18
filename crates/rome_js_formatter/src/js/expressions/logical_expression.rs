use crate::prelude::*;
use crate::utils::{
    format_binary_like_expression, needs_binary_like_parentheses, JsAnyBinaryLikeExpression,
};

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsLogicalExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsLogicalExpression;

impl FormatNodeRule<JsLogicalExpression> for FormatJsLogicalExpression {
    fn fmt_fields(
        &self,
        node: &JsLogicalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsLogicalExpression(node.clone()),
            formatter,
        )
    }
}

impl NeedsParentheses for JsLogicalExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if let Some(parent) = JsLogicalExpression::cast(parent.clone()) {
            return parent.operator() != self.operator();
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
