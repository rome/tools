use std::borrow::Cow;

use crate::prelude::*;
use crate::utils::string_utils::ToAsciiLowercaseCow;

use crate::FormatNodeFields;
use rome_js_syntax::JsBigIntLiteralExpression;
use rome_js_syntax::JsBigIntLiteralExpressionFields;

impl FormatNodeFields<JsBigIntLiteralExpression> for FormatNodeRule<JsBigIntLiteralExpression> {
    fn format_fields(
        node: &JsBigIntLiteralExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let original = value_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => formatted![formatter, [value_token.format()]],
            Cow::Owned(lowercase) => Ok(formatter.format_replaced(
                &value_token,
                Token::new_dynamic(lowercase, value_token.text_trimmed_range().start()).into(),
            )),
        }
    }
}
