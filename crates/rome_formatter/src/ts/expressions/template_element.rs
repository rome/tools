use crate::utils::{format_template_literal, TemplateElement};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsTemplateElement;

impl ToFormatElement for TsTemplateElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Ts(self.clone()), formatter)
    }
}
