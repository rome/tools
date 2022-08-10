use crate::prelude::*;
use crate::utils::{
    binary_argument_needs_parens, is_simple_expression, resolve_expression, FormatPrecedence,
    JsAnyBinaryLikeLeftExpression,
};
use rome_formatter::write;

use crate::utils::JsAnyBinaryLikeExpression;

use rome_js_syntax::{
    JsAnyExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields, JsSyntaxKind,
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
                JsAnyExpression::JsObjectExpression(_) | JsAnyExpression::JsCallExpression(_) => {
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

// Allow list of nodes that manually handle inserting parens if needed
fn is_expression_handling_parens(expression: &JsAnyExpression) -> bool {
    use JsAnyExpression::*;

    if let JsAnyExpression::JsParenthesizedExpression(inner) = expression {
        if let Ok(inner) = inner.expression() {
            is_expression_handling_parens(&inner)
        } else {
            false
        }
    } else {
        matches!(
            expression,
            JsConditionalExpression(_)
                | JsArrayExpression(_)
                | JsUnaryExpression(_)
                | JsPreUpdateExpression(_)
                | JsPostUpdateExpression(_)
                | JsObjectExpression(_)
                | JsFunctionExpression(_)
                | JsClassExpression(_)
                | JsAwaitExpression(_)
                | JsYieldExpression(_)
                | JsIdentifierExpression(_)
                | JsThisExpression(_)
                | JsAnyLiteralExpression(_)
                | JsSequenceExpression(_)
                | JsSuperExpression(_)
                | JsAssignmentExpression(_)
                | JsArrowFunctionExpression(_)
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

    let expression = resolve_expression(expression.clone());

    match expression {
        JsAnyExpression::JsConditionalExpression(_) => {
            panic!("Reached conditional expression when it should have not, parent is:\n{parent:#?}\nexpression:\n{expression:#?}")
        }

        _ => {
            // fall through
        }
    }

    let parent_precedence = FormatPrecedence::with_precedence_for_parenthesis(parent.as_ref());

    if parent_precedence > FormatPrecedence::Low {
        return Ok(false);
    }

    if let Some(parent) = parent {
        if let Some(binary_like) = JsAnyBinaryLikeExpression::cast(parent) {
            let operator = binary_like.operator()?;
            let is_right = expression.syntax() == binary_like.right()?.syntax();

            if !binary_argument_needs_parens(
                operator,
                &JsAnyBinaryLikeLeftExpression::from(expression.clone()),
                is_right,
            )? {
                return Ok(true);
            }
        }
    }

    Ok(false)
}
