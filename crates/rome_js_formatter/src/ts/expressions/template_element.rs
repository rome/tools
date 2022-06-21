use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};

use rome_js_syntax::TsTemplateElement;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTemplateElement;

impl FormatNodeRule<TsTemplateElement> for FormatTsTemplateElement {
    fn fmt_fields(node: &TsTemplateElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_template_literal(TemplateElement::Ts(node.clone()), formatter)
    }
}
