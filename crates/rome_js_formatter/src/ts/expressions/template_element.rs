use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use crate::FormatNodeFields;
use rome_js_syntax::TsTemplateElement;

impl FormatNodeFields<TsTemplateElement> for FormatNodeRule<TsTemplateElement> {
    fn format_fields(
        node: &TsTemplateElement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Ts(node.clone()), formatter)
    }
}
