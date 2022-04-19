use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsBigIntLiteralExpression;
use rome_js_syntax::JsBigIntLiteralExpressionFields;

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionFields { value_token } = self.as_fields();
        let value_token = value_token?;
        Ok(crate::utils::format_big_int_literal_token(
            value_token,
            formatter,
        ))
    }
}
