use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatStatementBody;
use rome_js_syntax::JsForStatement;
use rome_js_syntax::JsForStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsForStatement;

impl FormatNodeRule<JsForStatement> for FormatJsForStatement {
    fn fmt_fields(&self, node: &JsForStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForStatementFields {
            for_token,
            l_paren_token,
            initializer,
            first_semi_token,
            test,
            second_semi_token,
            update,
            r_paren_token,
            body,
        } = node.as_fields();

        let body = body?;

        let format_body = FormatStatementBody::new(&body);

        if initializer.is_none() && test.is_none() && update.is_none() {
            return write!(
                f,
                [group(&format_args![
                    for_token.format(),
                    space(),
                    l_paren_token.format(),
                    first_semi_token.format(),
                    second_semi_token.format(),
                    r_paren_token.format(),
                    format_body
                ])]
            );
        }

        let format_inner = format_with(|f| {
            write!(
                f,
                [
                    for_token.format(),
                    space(),
                    l_paren_token.format(),
                    group(&soft_block_indent(&format_args![
                        initializer.format(),
                        first_semi_token.format(),
                        soft_line_break_or_space(),
                        test.format(),
                        second_semi_token.format(),
                        soft_line_break_or_space(),
                        update.format()
                    ])),
                    r_paren_token.format(),
                    format_body
                ]
            )
        });

        write!(f, [group(&format_inner)])
    }
}
