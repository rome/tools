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
        formatter.format_token(&self.value_token()?)
    }
}

impl ToFormatElement for JsPrivateName {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.hash_token()?)?,
            formatter.format_token(&self.value_token()?)?
        ])
    }
}
