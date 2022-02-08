use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::{JsAnyName, JsName, JsPrivateName};

impl ToFormatElement for JsName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}
