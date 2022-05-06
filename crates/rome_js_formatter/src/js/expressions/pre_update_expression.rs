use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsPreUpdateExpression;
use rome_js_syntax::JsPreUpdateExpressionFields;

impl FormatNode for JsPreUpdateExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPreUpdateExpressionFields {
            operator_token,
            operand,
        } = self.as_fields();

        formatted![
            formatter,
            operator_token.format(formatter)?,
            operand.format(formatter)?,
        ]
    }
}
