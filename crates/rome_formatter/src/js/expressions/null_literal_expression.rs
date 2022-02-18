use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsNullLiteralExpression;
use rslint_parser::ast::JsNullLiteralExpressionFields;

impl ToFormatElement for JsNullLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNullLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
