use crate::{Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_formatter::Token;
use rome_js_syntax::JsRegexLiteralExpression;
use rome_js_syntax::JsRegexLiteralExpressionFields;

impl FormatNode for JsRegexLiteralExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsRegexLiteralExpressionFields { value_token } = self.as_fields();
        let value_token = value_token?;
        let trimmed_raw_string = value_token.text_trimmed();
        // find end slash, so we could split our regex literal to two part `body`: raw_string[0..end_slash_pos + 1] and `flags`: raw_string[end_slash_pos + 1..]
        // reference https://tc39.es/ecma262/#prod-RegularExpressionLiteral
        let ends_with_slash = trimmed_raw_string.ends_with('/');
        // this means that we have a regex literal with no flags
        if ends_with_slash {
            return value_token.format(formatter);
        }
        // SAFETY: a valid regex literal must have a end slash
        let end_slash_pos = trimmed_raw_string.rfind('/').unwrap();
        let mut flag_char_vec = trimmed_raw_string[end_slash_pos + 1..]
            .chars()
            .collect::<Vec<_>>();
        flag_char_vec.sort_unstable();
        let sorted_flag_string = flag_char_vec.iter().collect::<String>();

        let sorted_regex_literal = Token::from_syntax_token_cow_slice(
            std::borrow::Cow::Owned(format!(
                "{}{}",
                &trimmed_raw_string[0..end_slash_pos + 1],
                sorted_flag_string
            )),
            &value_token,
            value_token.text_trimmed_range().start(),
        );
        Ok(formatter.format_replaced(&value_token, sorted_regex_literal.into()))
    }
}
