use crate::prelude::*;
use crate::utils::{format_template_literal, TemplateElement};
use crate::FormatNodeFields;
use rome_js_syntax::JsTemplateElement;

impl FormatNodeFields<JsTemplateElement> for FormatNodeRule<JsTemplateElement> {
    fn fmt_fields(node: &JsTemplateElement, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_template_literal(TemplateElement::Js(node.clone()), formatter)
    }
}
