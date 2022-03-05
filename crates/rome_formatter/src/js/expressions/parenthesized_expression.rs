use crate::formatter_traits::FormatTokenAndNode;

use crate::utils::is_simple_expression;
use crate::{
    format_elements, hard_group_elements, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_syntax::{JsParenthesizedExpression, JsParenthesizedExpressionFields, SyntaxResult};

impl ToFormatElement for JsParenthesizedExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = self.as_fields();

        if is_simple_parenthesized_expression(self)? {
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
