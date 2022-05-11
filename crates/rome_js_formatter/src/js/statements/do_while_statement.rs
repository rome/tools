use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsDoWhileStatementFields;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};

impl FormatNodeFields<JsDoWhileStatement> for FormatNodeRule<JsDoWhileStatement> {
    fn format_fields(
        node: &JsDoWhileStatement,
        formatter: &Formatter,
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
                formatter.format_delimited_soft_block_indent(
                    &l_paren_token?,
                    formatted![formatter, [test.format()]]?,
                    &r_paren_token?,
                )?,
                semicolon_token.format().or_format(|| token(";"))
            ]
        ]?;

        let body = body?;
        if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            Ok(hard_group_elements(formatted![
                formatter,
                [head, body.format(), tail,]
            ]?))
        } else {
            formatted![
                formatter,
                [
                    hard_group_elements(head),
                    body.format(),
                    hard_group_elements(tail),
                ]
            ]
        }
    }
}
