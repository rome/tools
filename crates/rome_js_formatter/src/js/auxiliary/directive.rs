use crate::prelude::*;
use crate::utils::{format_string_literal_token, format_with_semicolon};

use crate::FormatNodeFields;
use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl FormatNodeFields<JsDirective> for FormatNodeRule<JsDirective> {
    fn format_fields(node: &JsDirective, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDirectiveFields {
            value_token,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            format_string_literal_token(value_token?, formatter),
            semicolon_token,
        )
    }
}
