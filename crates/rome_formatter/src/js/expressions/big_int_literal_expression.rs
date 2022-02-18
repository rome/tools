use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsBigIntLiteralExpression;
use rslint_parser::ast::JsBigIntLiteralExpressionFields;

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
