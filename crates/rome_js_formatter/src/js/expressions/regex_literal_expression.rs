use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatError;
use rome_formatter::FormatResult;

use rome_formatter::concat_elements;
use rome_formatter::format_elements;
use rome_formatter::token;
use rome_formatter::Token;
use rome_js_syntax::JsRegexLiteralExpression;
use rome_js_syntax::JsRegexLiteralExpressionFields;
use rome_js_syntax::TextRange;
use rome_js_syntax::TextSize;

impl FormatNode for JsRegexLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRegexLiteralExpressionFields { value_token } = self.as_fields();
        let value_token = value_token?;
        let raw_string = value_token.text();
        let range = value_token.text_range();
        let end_slash_pos = raw_string
            .rfind('/')
            .ok_or_else(|| FormatError::MissingRequiredChild)?;
        if end_slash_pos == raw_string.len() - 1 {
            return value_token.format(formatter);
        }
        let mut flag = raw_string[end_slash_pos + 1..].chars().collect::<Vec<_>>();
        flag.sort();
        let flag = flag.iter().collect::<String>();
        // formatter.format_replaced(current_token, content_to_replace_with);
        let flag_token = Token::from_syntax_token_cow_slice(
            std::borrow::Cow::Owned(format!("{}{}", &raw_string[0..end_slash_pos + 1], flag)),
            &value_token,
            range.start(),
        );
        Ok(formatter.format_replaced(&value_token, FormatElement::Token(flag_token)))
        // value_token.format(formatter)
    }
}
