use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::utils::get_member_chain;
use rome_js_syntax::{JsCallExpression, JsSyntaxKind, JsSyntaxNode};

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallExpression;

impl FormatNodeRule<JsCallExpression> for FormatJsCallExpression {
    fn fmt_fields(&self, node: &JsCallExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let member_chain = get_member_chain(node, f)?;

        member_chain.fmt(f)
    }

    fn needs_parentheses(&self, item: &JsCallExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsCallExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        matches!(parent.kind(), JsSyntaxKind::JS_NEW_EXPRESSION)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsCallExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (call())()", JsCallExpression);

        assert_not_needs_parentheses!("a?.()!.c", JsCallExpression);
        assert_not_needs_parentheses!("(a?.())!.c", JsCallExpression);

        assert_not_needs_parentheses!("(call())()", JsCallExpression[1]);
        assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[0]);
        assert_not_needs_parentheses!("getLogger().error(err);", JsCallExpression[1]);
    }
}
