use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsPostUpdateExpression;
use rome_js_syntax::JsPostUpdateExpressionFields;

impl FormatNode for JsPostUpdateExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsPostUpdateExpressionFields {
            operand,
            operator_token,
        } = self.as_fields();

        formatted![
            formatter,
            operand.format(formatter)?,
            operator_token.format(formatter)?,
        ]
    }
}
