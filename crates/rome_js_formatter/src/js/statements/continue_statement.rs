use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

impl FormatNodeFields<JsContinueStatement> for FormatNodeRule<JsContinueStatement> {
    fn format_fields(
        node: &JsContinueStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    continue_token.format(),
                    label_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [space_token(), token]])
                ]
            ]?,
            semicolon_token,
        )
    }
}
