use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyBindingPattern;

impl ToFormatElement for JsAnyBindingPattern {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyBindingPattern::JsAnyBinding(binding) => binding.to_format_element(formatter),
            JsAnyBindingPattern::JsArrayBindingPattern(array) => array.to_format_element(formatter),
            JsAnyBindingPattern::JsObjectBindingPattern(binding) => {
                binding.to_format_element(formatter)
            }
        }
    }
}
