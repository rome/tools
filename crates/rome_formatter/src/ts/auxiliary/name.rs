use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsAnyName, JsName, JsPrivateName};

impl ToFormatElement for JsAnyName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyName::JsName(name) => name.to_format_element(formatter),
            JsAnyName::JsPrivateName(name) => name.to_format_element(formatter),
        }
    }
}

impl ToFormatElement for JsName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        self.value_token().format(formatter)
    }
}

impl ToFormatElement for JsPrivateName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.hash_token().format(formatter)?,
            self.value_token().format(formatter)?
        ])
    }
}
