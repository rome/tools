use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use crate::FormatNodeFields;
use rome_js_syntax::TsTemplateElement;

impl FormatNodeFields<TsTemplateElement> for FormatNodeRule<TsTemplateElement> {
    fn fmt_fields(node: &TsTemplateElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_template_literal(TemplateElement::Ts(node.clone()), formatter)
    }
}
