use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use rome_js_syntax::JsTemplateElement;

impl FormatNode for JsTemplateElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Js(self.clone()), formatter)
    }
}
