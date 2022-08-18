use crate::prelude::*;
use crate::utils::{
    format_binary_like_expression, needs_binary_like_parentheses, JsAnyBinaryLikeExpression,
};

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{JsAnyExpression, JsBinaryExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsBinaryExpression;

impl FormatNodeRule<JsBinaryExpression> for FormatJsBinaryExpression {
    fn fmt_fields(
        &self,
        node: &JsBinaryExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsBinaryExpression(node.clone()),
            formatter,
        )
    }
}

impl NeedsParentheses for JsBinaryExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        needs_binary_like_parentheses(&JsAnyBinaryLikeExpression::from(self.clone()), parent)
    }
}

impl ExpressionNode for JsBinaryExpression {
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
    use rome_js_syntax::{JsBinaryExpression, SourceType};

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class X extends (4 + 4) {}", JsBinaryExpression);

        assert_needs_parentheses!("(4 + 4) as number", JsBinaryExpression);
        assert_needs_parentheses!("<number>(4 + 4)", JsBinaryExpression);
        assert_needs_parentheses!("!(4 + 4)", JsBinaryExpression);
        assert_needs_parentheses!("await (4 + 4)", JsBinaryExpression);
        assert_needs_parentheses!("(4 + 4)!", JsBinaryExpression);

        assert_needs_parentheses!("(4 + 4)()", JsBinaryExpression);
        assert_needs_parentheses!("(4 + 4)?.()", JsBinaryExpression);
        assert_needs_parentheses!("new (4 + 4)()", JsBinaryExpression);
        assert_needs_parentheses!("(4 + 4)`template`", JsBinaryExpression);
        assert_needs_parentheses!("[...(4 + 4)]", JsBinaryExpression);
        assert_needs_parentheses!("({...(4 + 4)})", JsBinaryExpression);
        assert_needs_parentheses!(
            "<test {...(4 + 4)} />",
            JsBinaryExpression,
            SourceType::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(4 + 4)}</test>",
            JsBinaryExpression,
            SourceType::tsx()
        );

        assert_needs_parentheses!("(4 + 4).member", JsBinaryExpression);
        assert_needs_parentheses!("(4 + 4)[member]", JsBinaryExpression);
        assert_not_needs_parentheses!("object[4 + 4]", JsBinaryExpression);

        assert_needs_parentheses!("(4 + 4) * 3", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("(4 + 4) * 3", JsBinaryExpression[0]);

        assert_needs_parentheses!("a ** b ** c", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("a ** b ** c", JsBinaryExpression[0]);

        assert_needs_parentheses!("a * r >> 5", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("a * r >> 5", JsBinaryExpression[0]);

        assert_needs_parentheses!("a * r | 4", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("a * r | 5", JsBinaryExpression[0]);

        assert_needs_parentheses!("a % 4 + 4", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("a % 4 + 4", JsBinaryExpression[0]);

        assert_needs_parentheses!("a == b == c", JsBinaryExpression[1]);
        assert_not_needs_parentheses!("a == b == c", JsBinaryExpression[0]);
    }
}
