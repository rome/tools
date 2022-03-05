use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsBooleanLiteralExpression;
use rslint_syntax::JsBooleanLiteralExpressionFields;

impl ToFormatElement for JsBooleanLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBooleanLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
