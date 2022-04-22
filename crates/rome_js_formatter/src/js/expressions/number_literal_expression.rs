use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsNumberLiteralExpression;
use rome_js_syntax::JsNumberLiteralExpressionFields;

impl FormatNode for JsNumberLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNumberLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
