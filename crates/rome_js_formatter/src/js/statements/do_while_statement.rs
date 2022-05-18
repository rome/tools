use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsDoWhileStatement;
use rome_js_syntax::JsDoWhileStatementFields;

impl FormatNodeFields<JsDoWhileStatement> for FormatNodeRule<JsDoWhileStatement> {
    fn format_fields(
        node: &JsDoWhileStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        let head = formatted![formatter, [do_token.format(), space_token(),]]?;

        let tail = formatted![
            formatter,
            [
                space_token(),
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
                semicolon_token.format().or_format(|| token(";"))
            ]
        ]?;

        formatted![formatter, [head, body.format(), tail,]]
    }
}
