use rome_js_syntax::JsForOfStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;

use crate::FormatNodeFields;
use rome_js_syntax::JsForOfStatementFields;

impl FormatNodeFields<JsForOfStatement> for FormatNodeRule<JsForOfStatement> {
    fn format_fields(
        node: &JsForOfStatement,
        formatter: &Formatter,
    ) -> FormatResult<FormatElement> {
        let JsForOfStatementFields {
            for_token,
            await_token,
            l_paren_token,
            initializer,
            of_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    for_token.format(),
                    space_token(),
                    await_token
                        .format()
                        .with_or_empty(|token| formatted![formatter, [token, space_token()]]),
                    formatter.format_delimited_soft_block_indent(
                        &l_paren_token?,
                        formatted![
                            formatter,
                            [
                                initializer.format(),
                                soft_line_break_or_space(),
                                of_token.format(),
                                soft_line_break_or_space(),
                                expression.format(),
                            ]
                        ]?,
                        &r_paren_token?
                    )?,
                ]
            ]?,
            body?,
        )
    }
}
