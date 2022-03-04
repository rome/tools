use crate::utils::{format_template_literal, TemplateElement};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsTemplateElement;

impl ToFormatElement for JsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Js(self.clone()), formatter)
    }
}
