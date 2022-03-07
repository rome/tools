use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsNumberLiteralExpression;
use rome_js_syntax::JsNumberLiteralExpressionFields;

impl ToFormatElement for JsNumberLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNumberLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
