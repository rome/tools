use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{JsAnyFunction, JsArrowFunctionExpression};

impl ToFormatElement for JsArrowFunctionExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).to_format_element(formatter)
    }
}
