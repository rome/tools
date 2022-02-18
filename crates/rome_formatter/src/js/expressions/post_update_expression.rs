use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsPostUpdateExpression;
use rslint_parser::ast::JsPostUpdateExpressionFields;

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionFields { operand, operator } = self.as_fields();

        Ok(format_elements![
            operand.format(formatter)?,
            operator.format(formatter)?,
        ])
    }
}
