use crate::utils::format_string_literal_token;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::TsStringLiteralType;

impl ToFormatElement for TsStringLiteralType {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_string_literal_token(
            self.literal_token()?,
            formatter,
        ))
    }
}
