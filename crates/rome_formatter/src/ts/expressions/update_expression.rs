use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsPostUpdateExpression, JsPreUpdateExpression};

impl ToFormatElement for JsPreUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_token(&self.operator()?)?,
            formatter.format_node(self.operand()?)?,
        ])
    }
}

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            formatter.format_node(self.operand()?)?,
            formatter.format_token(&self.operator()?)?,
        ])
    }
}
