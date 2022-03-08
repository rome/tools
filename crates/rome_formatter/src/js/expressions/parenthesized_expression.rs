use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::is_simple_expression;
use crate::{
    empty_element, format_elements, hard_group_elements, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{
    AstNode, JsAnyExpression, JsParenthesizedExpression, JsParenthesizedExpressionFields,
    JsSyntaxKind, SyntaxNode, SyntaxResult,
};

impl ToFormatElement for JsParenthesizedExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = self.as_fields();

        if parenthesis_can_be_omitted(self)? {
            Ok(hard_group_elements(format_elements![
                formatter.format_replaced(&l_paren_token?, empty_element())?,
                expression.format(formatter)?,
                formatter.format_replaced(&r_paren_token?, empty_element())?,
            ]))
        } else if is_simple_parenthesized_expression(self)? {
            Ok(hard_group_elements(format_elements![
                l_paren_token.format(formatter)?,
                expression.format(formatter)?,
                r_paren_token.format(formatter)?,
            ]))
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

    if let Some(parent) = parent {
        if matches!(
            parent.kind(),
            JsSyntaxKind::TS_AS_EXPRESSION
                | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
                | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                | JsSyntaxKind::JS_UNARY_EXPRESSION
                | JsSyntaxKind::JS_LOGICAL_EXPRESSION
                | JsSyntaxKind::JS_BINARY_EXPRESSION
                | JsSyntaxKind::JS_TEMPLATE
                | JsSyntaxKind::JS_SPREAD
                | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                | JsSyntaxKind::JS_CALL_EXPRESSION
                | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                | JsSyntaxKind::JS_NEW_EXPRESSION
                | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
                | JsSyntaxKind::JS_EXTENDS_CLAUSE
                | JsSyntaxKind::TS_IMPLEMENTS_CLAUSE
                | JsSyntaxKind::JS_AWAIT_EXPRESSION
                | JsSyntaxKind::JS_YIELD_ARGUMENT
        ) {
            return Ok(false);
        }
    } else {
        return Ok(true);
    }

    match expression {
        JsAnyExpression::JsBinaryExpression(expression) => {
            let left = expression.left()?;
            let right = expression.right()?;

            let simple =
                not_binaryish_expression(left.syntax()) && not_binaryish_expression(right.syntax());

            Ok(simple)
        }

        JsAnyExpression::JsLogicalExpression(expression) => {
            let left = expression.left()?;
            let right = expression.right()?;

            let simple =
                not_binaryish_expression(left.syntax()) && not_binaryish_expression(right.syntax());

            Ok(simple)
        }
        _ => Ok(false),
    }
}

fn not_binaryish_expression(node: &SyntaxNode) -> bool {
    !matches!(
        node.kind(),
        JsSyntaxKind::JS_BINARY_EXPRESSION | JsSyntaxKind::JS_LOGICAL_EXPRESSION
    )
}
