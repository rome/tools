use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::{JsAnyFunction, JsFunctionStatement};

impl ToFormatElement for JsFunctionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).to_format_element(formatter)
    }
}
