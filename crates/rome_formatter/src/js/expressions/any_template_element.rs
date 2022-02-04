use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyTemplateElement;

impl ToFormatElement for JsAnyTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyTemplateElement::JsTemplateChunkElement(node) => node.to_format_element(formatter),
            JsAnyTemplateElement::JsTemplateElement(node) => node.to_format_element(formatter),
        }
    }
}
