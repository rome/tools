use crate::prelude::*;
use crate::utils::{
    binary_argument_needs_parens, is_simple_expression, FormatPrecedence,
    JsAnyBinaryLikeLeftExpression,
};
use rome_formatter::write;

use crate::utils::JsAnyBinaryLikeExpression;

use crate::parentheses::{ExpressionNode, NeedsParentheses};
use rome_js_syntax::{
    JsAnyExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields, JsSyntaxKind,
    JsSyntaxNode,
};
use rome_rowan::{AstNode, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub struct FormatJsParenthesizedExpression;

impl FormatNodeRule<JsParenthesizedExpression> for FormatJsParenthesizedExpression {
    fn fmt_fields(
        &self,
        node: &JsParenthesizedExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let expression = expression?;

        if is_expression_handling_parens(&expression) {
            return write!(
                f,
                [
                    format_removed(&l_paren_token?),
                    expression.format(),
                    format_removed(&r_paren_token?)
                ]
            );
        }

        let parenthesis_can_be_omitted = parenthesis_can_be_omitted(node, &expression)?;

        if parenthesis_can_be_omitted {
            write![
                f,
                [
                    format_removed(&l_paren_token?),
                    expression.format(),
                    format_removed(&r_paren_token?),
                ]
            ]
        } else if is_simple_parenthesized_expression(node)? {
            write![
                f,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format()
                ]
            ]
        } else {
            write![
                f,
                [
                    format_delimited(&l_paren_token?, &expression.format(), &r_paren_token?,)
                        .soft_block_indent()
                ]
            ]
        }
    }

    fn needs_parentheses(&self, item: &JsParenthesizedExpression) -> bool {
        item.needs_parentheses()
    }
}

fn is_simple_parenthesized_expression(node: &JsParenthesizedExpression) -> SyntaxResult<bool> {
    let JsParenthesizedExpressionFields {
        l_paren_token,
        expression,
        r_paren_token,
    } = node.as_fields();

    if l_paren_token?.has_trailing_comments() || r_paren_token?.has_leading_comments() {
        return Ok(false);
    }

    if !is_simple_expression(&expression?)? {
        return Ok(false);
    }

    Ok(true)
}

// Allow list of nodes that use the new `need_parens` formatting to determine if parentheses are necessary or not.
pub(crate) fn is_expression_handling_parens(expression: &JsAnyExpression) -> bool {
    use JsAnyExpression::*;

    if let JsAnyExpression::JsParenthesizedExpression(inner) = expression {
        if let Ok(inner) = inner.expression() {
            is_expression_handling_parens(&inner)
        } else {
            false
        }
    } else {
        !matches!(
            expression,
            JsInstanceofExpression(_)
                | JsBinaryExpression(_)
                | JsInExpression(_)
                | JsLogicalExpression(_)
        )
    }
}

fn parenthesis_can_be_omitted(
    node: &JsParenthesizedExpression,
    expression: &JsAnyExpression,
) -> SyntaxResult<bool> {
    let parent = node.syntax().parent();

    if let Some(parent) = &parent {
        match parent.kind() {
            // The formatting of the return or throw argument takes care of adding parentheses if necessary
            JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_THROW_STATEMENT => {
                return Ok(true)
            }
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => return Ok(true),
            _ => {
                // fall through
            }
        }
    }

    let expression = expression.resolve();

    let parent_precedence = FormatPrecedence::with_precedence_for_parenthesis(parent.as_ref());

    if parent_precedence > FormatPrecedence::Low {
        return Ok(false);
    }

    if let Some(parent) = parent {
        if JsAnyBinaryLikeExpression::can_cast(parent.kind())
            && !binary_argument_needs_parens(&JsAnyBinaryLikeLeftExpression::from(expression))
        {
            return Ok(true);
        }
    }

    Ok(false)
}

impl NeedsParentheses for JsParenthesizedExpression {
    #[inline(always)]
    fn needs_parentheses(&self) -> bool {
        false
    }

    #[inline(always)]
    fn needs_parentheses_with_parent(&self, _parent: &JsSyntaxNode) -> bool {
        false
    }
}

impl ExpressionNode for JsParenthesizedExpression {
    fn resolve(&self) -> JsAnyExpression {
        let inner = self.expression();

        inner.unwrap_or_else(|_| self.clone().into())
    }

    fn into_resolved(self) -> JsAnyExpression {
        let inner = self.expression();

        inner.unwrap_or_else(|_| self.into())
    }
}
