use rome_js_syntax::JsForInStatement;

use crate::prelude::*;
use crate::utils::format_head_body_statement;
use crate::FormatNodeFields;
use rome_js_syntax::JsForInStatementFields;

impl FormatNodeFields<JsForInStatement> for FormatNodeRule<JsForInStatement> {
    fn format_fields(
        node: &JsForInStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsForInStatementFields {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let for_token = for_token.format();
        let initializer = initializer.format();
        let in_token = in_token.format();
        let expression = expression.format();

        format_head_body_statement(
            formatter,
            formatted![
                formatter,
                [
                    for_token,
                    space_token(),
                    formatter
                        .delimited(
                            &l_paren_token?,
                            formatted![
                                formatter,
                                [
                                    initializer,
                                    soft_line_break_or_space(),
                                    in_token,
                                    soft_line_break_or_space(),
                                    expression,
                                ]
                            ]?,
                            &r_paren_token?
                        )
                        .soft_block_indent()
                        .finish()?,
                ]
            ]?,
            body?,
        )
    }
}
