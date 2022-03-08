use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsNullLiteralExpression;
use rome_js_syntax::JsNullLiteralExpressionFields;

impl ToFormatElement for JsNullLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsNullLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
