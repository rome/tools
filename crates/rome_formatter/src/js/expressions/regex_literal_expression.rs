use crate::formatter_traits::FormatTokenAndNode;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsRegexLiteralExpression;
use rome_js_syntax::JsRegexLiteralExpressionFields;

impl ToFormatElement for JsRegexLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRegexLiteralExpressionFields { value_token } = self.as_fields();

        value_token.format(formatter)
    }
}
