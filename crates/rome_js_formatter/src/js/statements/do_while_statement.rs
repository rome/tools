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

        formatted![formatter, [head, body.format(), tail,]]
        let body = body?;
        if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
            Ok(hard_group_elements(format_elements![
                head,
                space_token(),
                body.format(formatter)?,
                tail,
            ]))
        } else if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            Ok(format_elements![
                hard_group_elements(format_elements![head, body.format(formatter)?,]),
                hard_line_break(),
                tail,
            ])
        } else {
            Ok(format_elements![
                hard_group_elements(format_elements![head, space_token()]),
                body.format(formatter)?,
                hard_group_elements(tail),
            ])
        }
    }
}
