use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsNullLiteralExpression;
use rome_js_syntax::JsNullLiteralExpressionFields;

impl FormatNode for JsNullLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNullLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
