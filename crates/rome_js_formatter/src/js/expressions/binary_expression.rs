use crate::prelude::*;
use crate::utils::{needs_binary_like_parentheses, AnyJsBinaryLikeExpression};

use crate::parentheses::NeedsParentheses;
use rome_js_syntax::{JsBinaryExpression, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsBinaryExpression;

impl FormatNodeRule<JsBinaryExpression> for FormatJsBinaryExpression {
    fn fmt_fields(
        &self,
        node: &JsBinaryExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsBinaryLikeExpression::JsBinaryExpression(node.clone()).fmt(formatter)
    }

    fn needs_parentheses(&self, item: &JsBinaryExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsBinaryExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        needs_binary_like_parentheses(&AnyJsBinaryLikeExpression::from(self.clone()), parent)
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::{JsBinaryExpression, JsFileSource};

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
            JsFileSource::tsx()
        );
        assert_needs_parentheses!(
            "<test>{...(4 + 4)}</test>",
            JsBinaryExpression,
            JsFileSource::tsx()
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
