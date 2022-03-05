use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsBigIntLiteralExpression;
use rslint_syntax::JsBigIntLiteralExpressionFields;

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
