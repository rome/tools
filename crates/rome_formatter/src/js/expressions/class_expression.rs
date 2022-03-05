use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::{JsAnyClass, JsClassExpression};

impl ToFormatElement for JsClassExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).to_format_element(formatter)
    }
}
