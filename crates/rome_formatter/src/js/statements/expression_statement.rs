use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

impl ToFormatElement for JsExpressionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(formatter, expression.format(formatter)?, semicolon_token)
    }
}
