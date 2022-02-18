use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsParenthesizedExpression;
use rslint_parser::ast::JsParenthesizedExpressionFields;

impl ToFormatElement for JsParenthesizedExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsParenthesizedExpressionFields {
            l_paren_token,
            expression,
            r_paren_token,
        } = self.as_fields();

        Ok(format_elements![
            l_paren_token.format(formatter)?,
            expression.format(formatter)?,
            r_paren_token.format(formatter)?,
        ])
    }
}
