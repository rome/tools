use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use crate::FormatNodeFields;
use rome_js_syntax::JsTemplateElement;

impl FormatNodeFields<JsTemplateElement> for FormatNodeRule<JsTemplateElement> {
    fn format_fields(
        node: &JsTemplateElement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_template_literal(TemplateElement::Js(node.clone()), formatter)
    }
}
