use crate::utils::format_binaryish_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsAnyExpression, JsBinaryExpression};

impl ToFormatElement for JsBinaryExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binaryish_expression(
            &JsAnyExpression::JsBinaryExpression(self.clone()),
            formatter,
        )
    }
}
