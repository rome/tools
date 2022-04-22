use crate::utils::{format_conditional, Conditional};
use crate::{FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::JsConditionalExpression;

impl FormatNode for JsConditionalExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(Conditional::Expression(self.clone()), formatter, false)
    }
}
