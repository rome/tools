use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsDoWhileStatementFields;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};

impl FormatNodeFields<JsDoWhileStatement> for FormatNodeRule<JsDoWhileStatement> {
    fn format_fields(
        node: &JsDoWhileStatement,
        formatter: &JsFormatter,
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

        let head = formatted![formatter, [do_token.format()]]?;

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

        let body = body?;
        if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            formatted![formatter, [head, body.format(), hard_line_break(), tail,]]
        } else {
            formatted![formatter, [head, space_token(), body.format(), tail,]]
        }
    }
}
