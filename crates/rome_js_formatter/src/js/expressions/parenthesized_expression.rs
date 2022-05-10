use crate::utils::{is_simple_expression, FormatPrecedence};
use crate::{
    empty_element, format_elements, group_elements, hard_group_elements, Format, FormatElement,
    FormatNode, Formatter, JsFormatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::{
    JsAnyExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields,
    JsStringLiteralExpression, JsSyntaxKind, JsSyntaxNode,
};
use rome_rowan::{AstNode, SyntaxResult};

impl FormatNode for JsParenthesizedExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = self.as_fields();

        let parenthesis_can_be_omitted = parenthesis_can_be_omitted(self)?;

        let expression = expression?;

        if is_simple_parenthesized_expression(self)? {
            Ok(hard_group_elements(format_elements![
                if parenthesis_can_be_omitted {
                    formatter.format_replaced(&l_paren_token?, empty_element())
                } else {
                    l_paren_token.format(formatter)?
                },
                expression.format(formatter)?,
                if parenthesis_can_be_omitted {
                    formatter.format_replaced(&r_paren_token?, empty_element())
                } else {
                    r_paren_token.format(formatter)?
                },
            ]))
        } else if parenthesis_can_be_omitted {
            // we mimic the format delimited utility function
            Ok(format_elements![
                formatter.format_replaced(&l_paren_token?, empty_element()),
                group_elements(expression.format(formatter)?),
                formatter.format_replaced(&r_paren_token?, empty_element()),
            ])
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
        } else if JsStringLiteralExpression::can_cast(expression.syntax().kind()) {
            Ok(format_elements![
                l_paren_token.format(formatter)?,
                expression.format(formatter)?,
                r_paren_token.format(formatter)?,
            ])
        } else {
            formatter.format_delimited_soft_block_indent(
                &l_paren_token?,
                expression.format(formatter)?,
                &r_paren_token?,
            )
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

    if !is_simple_expression(expression?)? {
        return Ok(false);
    }

    Ok(true)
}

fn parenthesis_can_be_omitted(node: &JsParenthesizedExpression) -> SyntaxResult<bool> {
    let expression = node.expression()?;
    let parent = node.syntax().parent();
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
