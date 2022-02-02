use crate::formatter_traits::FormatTokenAndNode;
use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{JsPostUpdateExpression, JsPreUpdateExpression};

impl ToFormatElement for JsPreUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.operator().format(formatter)?,
            self.operand().format(formatter)?,
        ])
    }
}

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            self.operand().format(formatter)?,
            self.operator().format(formatter)?,
        ])
    }
}
