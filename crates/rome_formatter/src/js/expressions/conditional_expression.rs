use crate::utils::{format_conditional, Conditional};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsConditionalExpression;

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(Conditional::Expression(self.clone()), formatter, false)
    }
}
