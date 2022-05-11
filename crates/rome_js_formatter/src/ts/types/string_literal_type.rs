use crate::prelude::*;
use crate::utils::format_string_literal_token;
use rome_js_syntax::TsStringLiteralType;

impl FormatNode for TsStringLiteralType {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_string_literal_token(
            self.literal_token()?,
            formatter,
        ))
    }
}
