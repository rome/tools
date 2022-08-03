use crate::prelude::*;
use crate::utils::{
    binary_argument_needs_parens, is_simple_expression, FormatPrecedence,
    JsAnyBinaryLikeLeftExpression,
};
use rome_formatter::write;

use crate::utils::JsAnyBinaryLikeExpression;

use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsParenthesizedExpression,
    JsParenthesizedExpressionFields, JsSyntaxKind,
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

        let parenthesis_can_be_omitted = parenthesis_can_be_omitted(node)?;

        let expression = expression?;

        if is_simple_parenthesized_expression(node)? {
            if parenthesis_can_be_omitted {
                write!(f, [format_removed(&l_paren_token?)])?;
            } else {
                write![f, [l_paren_token.format()]]?;
            };

            write![f, [expression.format()]]?;

            if parenthesis_can_be_omitted {
                write!(f, [format_removed(&r_paren_token?)])?;
            } else {
                write![f, [r_paren_token.format()]]?;
            }
        } else if parenthesis_can_be_omitted {
            // we mimic the format delimited utility function
            write![
                f,
                [
                    format_removed(&l_paren_token?),
                    expression.format(),
                    format_removed(&r_paren_token?),
                ]
            ]?;
        } else {
            match expression {
                // if the expression inside the parenthesis is a stringLiteralExpression, we should leave it as is rather than
                // add extra soft_block_indent, for example:
                // ```js
                // ("escaped carriage return \
                // ");
                // ```
                // if we add soft_block_indent, we will get:
                // ```js
                // (
                // "escaped carriage return \
                // "
                // );
                // ```
                // which will not match prettier's formatting behavior, if we add this extra branch to handle this case, it become:
                // ```js
                // ("escaped carriage return \
                // ");
                // ```
                // this is what we want
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsStringLiteralExpression(_),
                ) => {
                    write![
                        f,
                        [
                            l_paren_token.format(),
                            expression.format(),
                            r_paren_token.format(),
                        ]
                    ]
                }
                JsAnyExpression::JsxTagExpression(expression) => {
                    write![
                        f,
                        [
                            format_removed(&l_paren_token?),
                            expression.format(),
                            format_removed(&r_paren_token?),
                        ]
                    ]
                }
                _ => write![
                    f,
                    [
                        format_delimited(&l_paren_token?, &expression.format(), &r_paren_token?,)
                            .soft_block_indent()
                    ]
                ],
            }?;
        }

        Ok(())
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

fn parenthesis_can_be_omitted(node: &JsParenthesizedExpression) -> SyntaxResult<bool> {
    let expression = node.expression()?;
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

    // if expression is a StringLiteralExpression, we need to check it before precedence comparison, here is an example:
    // ```js
    // a[("test")]
    // ```
    // if we use precedence comparison, we will get:
    // parent_precedence should be `High` due to the parenthesized_expression's parent is ComputedMemberExpression,
    // and node_precedence should be `Low` due to expression is StringLiteralExpression. `parent_precedence > node_precedence` will return false,
    // the parenthesis will not be omitted.
    // But the expected behavior is that the parenthesis will be omitted. The code above should be formatted as:
    // ```js
    // a["test"]
    // ```
    // So we need to add extra branch to handle this case.
    if matches!(
        expression,
        JsAnyExpression::JsAnyLiteralExpression(JsAnyLiteralExpression::JsStringLiteralExpression(
            _
        ))
    ) {
        return Ok(!matches!(
            parent.map(|p| p.kind()),
            Some(JsSyntaxKind::JS_EXPRESSION_STATEMENT)
        ));
    }

    let parent_precedence = FormatPrecedence::with_precedence_for_parenthesis(parent.as_ref());
    let node_precedence = FormatPrecedence::with_precedence_for_parenthesis(Some(node.syntax()));

    if parent_precedence > node_precedence {
        return Ok(false);
    }

    if let Some(parent) = parent {
        if let Some(binary_like) = JsAnyBinaryLikeExpression::cast(parent) {
            let operator = binary_like.operator()?;

            if !binary_argument_needs_parens(
                operator,
                &JsAnyBinaryLikeLeftExpression::from(expression),
            )? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
