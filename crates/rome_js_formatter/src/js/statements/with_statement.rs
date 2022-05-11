use crate::prelude::*;
use crate::utils::format_head_body_statement;

use crate::FormatNodeFields;
use rome_js_syntax::JsWithStatement;
use rome_js_syntax::JsWithStatementFields;

impl FormatNodeFields<JsWithStatement> for FormatNodeRule<JsWithStatement> {
    fn format_fields(node: &JsWithStatement, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsWithStatementFields {
            with_token,
            l_paren_token,
            object,
            r_paren_token,
            body,
        } = node.as_fields();

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    with_token.format(),
                    space_token(),
                    formatter.format_delimited_soft_block_indent(
                        &l_paren_token?,
                        formatted![formatter, [object.format()]]?,
                        &r_paren_token?,
                    )?,
                ]
            ]?,
            body?,
        )
    }
}
