use crate::formatter_traits::FormatTokenAndNode;

use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsPreUpdateExpression;
use rslint_syntax::JsPreUpdateExpressionFields;

impl ToFormatElement for JsPreUpdateExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPreUpdateExpressionFields { operator, operand } = self.as_fields();

        Ok(format_elements![
            operator.format(formatter)?,
            operand.format(formatter)?,
        ])
    }
}
