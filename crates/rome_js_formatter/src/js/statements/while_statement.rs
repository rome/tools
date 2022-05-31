use crate::prelude::*;

use crate::utils::format_head_body_statement;
use crate::FormatNodeFields;
use rome_js_syntax::JsWhileStatement;
use rome_js_syntax::JsWhileStatementFields;

impl FormatNodeFields<JsWhileStatement> for FormatNodeRule<JsWhileStatement> {
    fn format_fields(
        node: &JsWhileStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsWhileStatementFields {
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            body,
        } = node.as_fields();

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    while_token.format(),
                    space_token(),
                    formatter
                        .delimited(
                            &l_paren_token?,
                            formatted![formatter, [test.format()]]?,
                            &r_paren_token?,
                        )
                        .soft_block_indent()
                        .finish()?,
                ]
            ]?,
            body?,
        )
    }
}
