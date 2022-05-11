use crate::prelude::*;
use crate::utils::{format_string_literal_token, format_with_semicolon};

use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl FormatNode for JsDirective {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
