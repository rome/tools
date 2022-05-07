use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use rome_js_syntax::TsTemplateElement;

impl FormatNode for TsTemplateElement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Ts(self.clone()), formatter)
    }
}
