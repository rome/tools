use std::borrow::Cow;

use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::string_utils::ToAsciiLowercaseCow;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_formatter::Token;
use rome_js_syntax::JsBigIntLiteralExpression;
use rome_js_syntax::JsBigIntLiteralExpressionFields;

impl ToFormatElement for JsBigIntLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsBigIntLiteralExpressionFields { value_token } = self.as_fields();
        let value_token = value_token?;

        let original = value_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => value_token.format(formatter),
            Cow::Owned(lowercase) => Ok(formatter.format_replaced(
                &value_token,
                Token::new_dynamic(lowercase, value_token.text_trimmed_range().start()).into(),
            )),
        }
    }
}
