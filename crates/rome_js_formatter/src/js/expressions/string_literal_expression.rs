use crate::{FormatElement, FormatNode, FormatResult, Formatter};

use crate::utils::format_string_literal_token;
use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;

impl FormatNode for JsStringLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = self.as_fields();

        let value_token = value_token?;

        Ok(format_string_literal_token(value_token, formatter))
    }
}
