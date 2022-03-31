use crate::utils::{format_string_literal_token, format_with_semicolon};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDirectiveFields {
            value_token,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            format_string_literal_token(value_token?, formatter),
            semicolon_token,
        )
    }
}
