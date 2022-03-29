use std::borrow::Cow;

use crate::format_element::normalize_newlines;

use crate::{format_element::Token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsStringLiteralExpression;
use rome_js_syntax::JsStringLiteralExpressionFields;

impl ToFormatElement for JsStringLiteralExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsStringLiteralExpressionFields { value_token } = self.as_fields();

        let value_token = value_token?;
        let quoted = value_token.text_trimmed();

        // replace single quotes with double quotes if the string does not contain any
        let content = if quoted.starts_with('\'') && !quoted.contains('"') {
            let s = &quoted[1..quoted.len() - 1];
            let s = format!("\"{}\"", s);
            match normalize_newlines(&s, ['\r']) {
                Cow::Borrowed(_) => s,
                Cow::Owned(s) => s,
            }
        } else {
            normalize_newlines(quoted, ['\r']).into_owned()
        };

        formatter.format_replaced(
            &value_token,
            Token::new_dynamic(content, value_token.text_trimmed_range()).into(),
        )
    }
}
