use rome_formatter::write;
use std::borrow::Cow;

use crate::prelude::*;
use crate::utils::string_utils::ToAsciiLowercaseCow;

use rome_js_syntax::JsBigIntLiteralExpression;
use rome_js_syntax::JsBigIntLiteralExpressionFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsBigIntLiteralExpression;

impl FormatNodeRule<JsBigIntLiteralExpression> for FormatJsBigIntLiteralExpression {
    fn fmt_fields(
        &self,
        node: &JsBigIntLiteralExpression,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsBigIntLiteralExpressionFields { value_token } = node.as_fields();
        let value_token = value_token?;

        let original = value_token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write![f, [value_token.format()]],
            Cow::Owned(lowercase) => {
                write!(
                    f,
                    [format_replaced(
                        &value_token,
                        &dynamic_text(&lowercase, value_token.text_trimmed_range().start())
                    )]
                )
            }
        }
    }
}
