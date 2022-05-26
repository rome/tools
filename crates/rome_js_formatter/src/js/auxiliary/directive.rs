use crate::prelude::*;
use crate::utils::{format_with_semicolon, FormatLiteralStringToken};
use crate::FormatNodeFields;
use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl FormatNodeFields<JsDirective> for FormatNodeRule<JsDirective> {
    fn format_fields(
        node: &JsDirective,
        formatter: &Formatter<JsFormatContext>,
    ) -> FormatResult<FormatElement> {
        let JsDirectiveFields {
            value_token,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            FormatLiteralStringToken::from_directive(&value_token?).format(formatter)?,
            semicolon_token,
        )
    }
}
