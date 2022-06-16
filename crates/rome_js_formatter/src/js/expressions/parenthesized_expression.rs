use crate::prelude::*;
use crate::utils::{is_simple_expression, FormatPrecedence};
use rome_formatter::write;

use crate::utils::JsAnyBinaryLikeExpression;
use crate::FormatNodeFields;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsParenthesizedExpression,
    JsParenthesizedExpressionFields, JsStringLiteralExpression, JsSyntaxKind,
};
use rome_rowan::{AstNode, SyntaxResult};

impl FormatNodeFields<JsParenthesizedExpression> for FormatNodeRule<JsParenthesizedExpression> {
    fn fmt_fields(node: &JsParenthesizedExpression, f: &mut JsFormatter) -> FormatResult<()> {
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

            write![f, [expression.format(),]]?;

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
                    group_elements(&expression.format()),
                    format_removed(&r_paren_token?),
                ]
            ]?;
        }
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
        else if JsStringLiteralExpression::can_cast(expression.syntax().kind()) {
            write![
                f,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format(),
                ]
            ]?;
        } else {
            write!(
                f,
                [
                    format_delimited(&l_paren_token?, &expression.format(), &r_paren_token?,)
                        .soft_block_indent()
                ]
            )?;
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
    // Here we handle cases where we have binary/logical expressions.
    // We want to remove the parenthesis only in cases where `left` and `right` are not other
    // binary/logical expressions.
    //
    // From another point of view, logical/binary expressions with the same operator can stay without
    // parenthesis.
    match expression {
        JsAnyExpression::JsBinaryExpression(expression) => {
            let left = expression.left()?;
            let right = expression.right()?;

            Ok(!JsAnyBinaryLikeExpression::can_cast(left.syntax().kind())
                && !JsAnyBinaryLikeExpression::can_cast(right.syntax().kind()))
        }

        JsAnyExpression::JsLogicalExpression(expression) => {
            let left = expression.left()?;
            let right = expression.right()?;

            Ok(!JsAnyBinaryLikeExpression::can_cast(left.syntax().kind())
                && !JsAnyBinaryLikeExpression::can_cast(right.syntax().kind()))
        }
        _ => Ok(false),
    }
}
