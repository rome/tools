use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::{JsAnyClass, JsClassStatement};

impl ToFormatElement for JsClassStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyClass::from(self.clone()).to_format_element(formatter)
    }
}
