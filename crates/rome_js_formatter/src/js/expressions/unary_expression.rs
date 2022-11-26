use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::parentheses::{unary_like_expression_needs_parentheses, NeedsParentheses};

use rome_js_syntax::JsSyntaxNode;
use rome_js_syntax::JsUnaryExpression;
use rome_js_syntax::{JsUnaryExpressionFields, JsUnaryOperator};
use rome_rowan::match_ast;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsUnaryExpression;

impl FormatNodeRule<JsUnaryExpression> for FormatJsUnaryExpression {
    fn fmt_fields(&self, node: &JsUnaryExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let JsUnaryExpressionFields {
            operator_token,
            argument,
        } = node.as_fields();

        let operation = node.operator()?;
        let operator_token = operator_token?;
        let argument = argument?;

        write!(f, [operator_token.format()])?;

        let is_keyword_operator = matches!(
            operation,
            JsUnaryOperator::Delete | JsUnaryOperator::Void | JsUnaryOperator::Typeof
        );

        if is_keyword_operator {
            write!(f, [space()])?;
        }

        if f.comments().has_comments(argument.syntax())
            && !f.comments().is_suppressed(argument.syntax())
        {
            write!(
                f,
                [group(&format_args![
                    text("("),
                    soft_block_indent(&argument.format()),
                    text(")")
                ])]
            )
        } else {
            write![f, [argument.format()]]
        }
    }

    fn needs_parentheses(&self, item: &JsUnaryExpression) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for JsUnaryExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match_ast! {
            match parent {
                JsUnaryExpression(parent_unary) => {
                    let parent_operator = parent_unary.operator();
                    let operator = self.operator();

                    matches!(operator, Ok(JsUnaryOperator::Plus | JsUnaryOperator::Minus))
                        && parent_operator == operator
                },
                _ => {
                    unary_like_expression_needs_parentheses(self.syntax(), parent)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use rome_js_syntax::JsUnaryExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("class A extends (!B) {}", JsUnaryExpression);

        assert_needs_parentheses!("(+a).b", JsUnaryExpression);
        assert_needs_parentheses!("(+a)[b]", JsUnaryExpression);
        assert_not_needs_parentheses!("a[+b]", JsUnaryExpression);

        assert_needs_parentheses!("(+a)`template`", JsUnaryExpression);

        assert_needs_parentheses!("(+a)()", JsUnaryExpression);
        assert_needs_parentheses!("new (+a)()", JsUnaryExpression);

        assert_needs_parentheses!("(+a)!", JsUnaryExpression);

        assert_needs_parentheses!("(+a) ** 3", JsUnaryExpression);
        assert_not_needs_parentheses!("(+a) + 3", JsUnaryExpression);
    }
}
