use crate::utils::format_binary_like_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsAnyExpression, JsBinaryExpression};

impl ToFormatElement for JsBinaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            &JsAnyExpression::JsBinaryExpression(self.clone()),
            formatter,
        )
    }
}
