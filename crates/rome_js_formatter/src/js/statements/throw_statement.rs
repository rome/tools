use crate::prelude::*;
use crate::utils::format_with_semicolon;

use rome_js_syntax::JsThrowStatement;
use rome_js_syntax::JsThrowStatementFields;

impl FormatNode for JsThrowStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = self.as_fields();

        let throw_token = throw_token.format(formatter)?;
        let exception = argument.format(formatter)?;

        format_with_semicolon(
            formatter,
            formatted![formatter, throw_token, space_token(), exception]?,
            semicolon_token,
        )
    }
}
