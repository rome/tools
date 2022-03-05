use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::{JsAnyFunction, JsFunctionDeclaration};

impl ToFormatElement for JsFunctionDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        JsAnyFunction::from(self.clone()).to_format_element(formatter)
    }
}
