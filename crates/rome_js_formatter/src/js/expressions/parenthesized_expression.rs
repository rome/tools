use crate::prelude::*;
use crate::utils::{is_simple_expression, FormatPrecedence};

use crate::FormatNodeFields;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsParenthesizedExpression,
    JsParenthesizedExpressionFields, JsSyntaxKind, JsSyntaxNode,
};
use rome_rowan::{AstNode, SyntaxResult};

impl FormatNodeFields<JsParenthesizedExpression> for FormatNodeRule<JsParenthesizedExpression> {
    fn format_fields(
        node: &JsParenthesizedExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = node.as_fields();

        let parenthesis_can_be_omitted = parenthesis_can_be_omitted(node)?;

        let expression = expression?;

        if is_simple_parenthesized_expression(node)? {
            formatted![
                formatter,
                [
                    if parenthesis_can_be_omitted {
                        formatter.format_replaced(&l_paren_token?, empty_element())
                    } else {
                        formatted![formatter, [l_paren_token.format()]]?
                    },
                    expression.format(),
                    if parenthesis_can_be_omitted {
                        formatter.format_replaced(&r_paren_token?, empty_element())
                    } else {
                        formatted![formatter, [r_paren_token.format()]]?
                    },
                ]
            ]
        } else if parenthesis_can_be_omitted {
            // we mimic the format delimited utility function
            formatted![
                formatter,
                [
                    formatter.format_replaced(&l_paren_token?, empty_element()),
                    group_elements(formatted![formatter, [expression.format()]]?),
                    formatter.format_replaced(&r_paren_token?, empty_element()),
                ]
            ]
        } else if !requires_soft_block_indent(node)? {
            formatted![
                formatter,
                [
                    l_paren_token.format(),
                    expression.format(),
                    r_paren_token.format(),
                ]
            ]
        } else {
            formatter
                .delimited(
                    &l_paren_token?,
                    formatted![formatter, [expression.format()]]?,
                    &r_paren_token?,
                )
                .soft_block_indent()
                .finish()
        }
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

            Ok(not_binaryish_expression(left.syntax()) && not_binaryish_expression(right.syntax()))
        }

        JsAnyExpression::JsLogicalExpression(expression) => {
            let left = expression.left()?;
            let right = expression.right()?;

            Ok(not_binaryish_expression(left.syntax()) && not_binaryish_expression(right.syntax()))
        }
        _ => Ok(false),
    }
}

fn not_binaryish_expression(node: &JsSyntaxNode) -> bool {
    !matches!(
        node.kind(),
        JsSyntaxKind::JS_BINARY_EXPRESSION | JsSyntaxKind::JS_LOGICAL_EXPRESSION
    )
}

fn requires_soft_block_indent(node: &JsParenthesizedExpression) -> SyntaxResult<bool> {
    let expression_kind = node.expression()?.syntax().kind();

    let requires_indent = match expression_kind {
        // Never block indent a parenthesized multiline string literal
        JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION => false,
        // Only soft block ident a parenthesized sequence expression when it's
        // the child of specific node types
        JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
            let parent_kind = node.syntax().parent().map(|p| p.kind());

            matches!(
                parent_kind,
                Some(JsSyntaxKind::JS_RETURN_STATEMENT)
                    | Some(JsSyntaxKind::JS_UNARY_EXPRESSION)
                    | Some(JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
            )
        }
        _ => true,
    };
    Ok(requires_indent)
}
