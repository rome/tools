use crate::utils::format_binary_like_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsAnyExpression, JsLogicalExpression};

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            &JsAnyExpression::JsLogicalExpression(self.clone()),
            formatter,
        )
    }
}
