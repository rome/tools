use crate::utils::format_binary_like_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsAnyExpression, JsInstanceofExpression};

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            &JsAnyExpression::JsInstanceofExpression(self.clone()),
            formatter,
        )
    }
}
