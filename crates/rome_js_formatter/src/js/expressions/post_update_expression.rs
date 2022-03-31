use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

impl ToFormatElement for JsPostUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionFields { operand, operator } = self.as_fields();

        Ok(format_elements![
            operand.format(formatter)?,
            operator.format(formatter)?,
        ])
    }
}
