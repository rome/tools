use std::borrow::Cow;

use crate::utils::string_utils::ToAsciiLowercaseCow;
use crate::{Format, FormatElement, FormatNode, Formatter, JsFormatter};

use rome_formatter::{FormatResult, Token};
use rome_js_syntax::JsBigIntLiteralExpression;
use rome_js_syntax::JsBigIntLiteralExpressionFields;

impl FormatNode for JsBigIntLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
