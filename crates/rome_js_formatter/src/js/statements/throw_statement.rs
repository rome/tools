use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsThrowStatement;
use rome_js_syntax::JsThrowStatementFields;

impl FormatNodeFields<JsThrowStatement> for FormatNodeRule<JsThrowStatement> {
    fn format_fields(
        node: &JsThrowStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        let throw_token = throw_token.format();
        let exception = argument.format();

        format_with_semicolon(
            formatter,
            formatted![formatter, [throw_token, space_token(), exception]]?,
            semicolon_token,
        )
    }
}
