use crate::prelude::*;
use crate::utils::number_utils::CleanedNumberLiteralText;

use crate::parentheses::{is_member_object, NeedsParentheses};
use rome_js_syntax::JsNumberLiteralExpression;
use rome_js_syntax::{JsNumberLiteralExpressionFields, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsNumberLiteralExpression;

impl FormatNodeRule<JsNumberLiteralExpression> for FormatJsNumberLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsNumberLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsNumberLiteralExpressionFields { value_token } = node.as_fields();
        CleanedNumberLiteralText::from_number_literal_token(&value_token?).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsNumberLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsNumberLiteralExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        is_member_object(self.syntax(), parent)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsNumberLiteralExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(5).test", JsNumberLiteralExpression);
        assert_needs_parentheses!("(5)[test]", JsNumberLiteralExpression);
        assert_not_needs_parentheses!("test[5]", JsNumberLiteralExpression);
    }
}
