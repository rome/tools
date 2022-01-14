use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyBinding;

impl ToFormatElement for JsAnyBinding {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyBinding::JsIdentifierBinding(single) => single.to_format_element(formatter),
            JsAnyBinding::JsUnknownBinding(_) => todo!(),
        }
    }
}
