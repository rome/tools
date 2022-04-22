use crate::utils::{format_template_literal, TemplateElement};
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsTemplateElement;

impl FormatNode for JsTemplateElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Js(self.clone()), formatter)
    }
}
