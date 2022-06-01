use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsBreakStatement;
use rome_js_syntax::JsBreakStatementFields;

impl FormatNodeFields<JsBreakStatement> for FormatNodeRule<JsBreakStatement> {
    fn format_fields(
        node: &JsBreakStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsBreakStatementFields {
            break_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    break_token.format(),
                    label_token
                        .format()
                        .with_or_empty(|label| formatted![formatter, [space_token(), label]])
                ]
            ]?,
            semicolon_token,
        )
    }
}
