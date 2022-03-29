use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;

impl ToFormatElement for JsStringLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = self.as_fields();

        let value_token = value_token?;

        Ok(format_string_literal_token(value_token, formatter))
    }
}
