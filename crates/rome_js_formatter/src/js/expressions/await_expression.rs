use crate::prelude::*;
use rome_formatter::write;

use crate::parentheses::{
    is_binary_like_left_or_right, is_conditional_test, unary_expression_needs_parentheses,
    NeedsParentheses,
};

use rome_js_syntax::{JsAwaitExpression, JsSyntaxNode};
use rome_js_syntax::{JsAwaitExpressionFields, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsAwaitExpression;

impl FormatNodeRule<JsAwaitExpression> for FormatJsAwaitExpression {
    fn fmt_fields(&self, node: &JsAwaitExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsAwaitExpressionFields {
            await_token,
            argument,
        } = node.as_fields();

        write![f, [await_token.format(), space(), argument.format(),]]
    }

    fn needs_parentheses(&self, item: &JsAwaitExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsAwaitExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        await_or_yield_needs_parens(parent, self.syntax())
    }
}

pub(super) fn await_or_yield_needs_parens(parent: &JsSyntaxNode, node: &JsSyntaxNode) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_UNARY_EXPRESSION | JsSyntaxKind::TS_AS_EXPRESSION => true,

        _ => {
            is_conditional_test(node, parent)
                || unary_expression_needs_parentheses(node, parent)
                || is_binary_like_left_or_right(node, parent)
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsAwaitExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(await a)`template`", JsAwaitExpression);
        assert_needs_parentheses!("+(await a)", JsAwaitExpression);

        assert_needs_parentheses!("(await a).b", JsAwaitExpression);
        assert_needs_parentheses!("(await a)[b]", JsAwaitExpression);
        assert_not_needs_parentheses!("a[await b]", JsAwaitExpression);

        assert_needs_parentheses!("(await a)()", JsAwaitExpression);
        assert_needs_parentheses!("new (await a)()", JsAwaitExpression);

        assert_needs_parentheses!("(await a) && b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) + b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) instanceof b", JsAwaitExpression);
        assert_needs_parentheses!("(await a) in b", JsAwaitExpression);

        assert_needs_parentheses!("[...(await a)]", JsAwaitExpression);
        assert_needs_parentheses!("({...(await b)})", JsAwaitExpression);
        assert_needs_parentheses!("call(...(await b))", JsAwaitExpression);

        assert_needs_parentheses!("class A extends (await b) {}", JsAwaitExpression);

        assert_needs_parentheses!("(await b) as number", JsAwaitExpression);
        assert_needs_parentheses!("(await b)!", JsAwaitExpression);

        assert_needs_parentheses!("(await b) ? b : c", JsAwaitExpression);
        assert_not_needs_parentheses!("a ? await b : c", JsAwaitExpression);
        assert_not_needs_parentheses!("a ? b : await c", JsAwaitExpression);
    }
}
