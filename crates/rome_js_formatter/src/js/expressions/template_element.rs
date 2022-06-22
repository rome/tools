use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};

use rome_js_syntax::JsTemplateElement;

#[derive(Debug, Clone, Default)]
pub struct FormatJsTemplateElement;

impl FormatNodeRule<JsTemplateElement> for FormatJsTemplateElement {
    fn fmt_fields(
        &self,
        node: &JsTemplateElement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_template_literal(TemplateElement::Js(node.clone()), formatter)
    }
}
