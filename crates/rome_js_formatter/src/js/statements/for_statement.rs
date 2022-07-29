use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsForStatement;
use rome_js_syntax::JsForStatementFields;
use rome_js_syntax::{JsAnyStatement, JsSyntaxKind};

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

        let condition = format_with(|f| {
            if initializer.is_some() || test.is_some() || update.is_some() {
                write![
                    f,
                    [
                        initializer.format(),
                        first_semi_token.format(),
                        soft_line_break_or_space(),
                        test.format(),
                        second_semi_token.format(),
                        soft_line_break_or_space(),
                        update.format(),
                    ]
                ]
            } else {
                write![f, [first_semi_token.format(), second_semi_token.format()]]
            }
        });

        let content = format_with(|f| {
            write!(
                f,
                [
                    for_token.format(),
                    space(),
                    format_delimited(l_paren_token.as_ref()?, &condition, r_paren_token.as_ref()?,)
                        .soft_block_indent(),
                ]
            )?;

            // Force semicolon insertion for empty bodies
            match body.as_ref()? {
                JsAnyStatement::JsEmptyStatement(body) => {
                    write![f, [body.format(), format_inserted(JsSyntaxKind::SEMICOLON)]]
                }
                body => {
                    write!(f, [space(), body.format()])
                }
            }
        });

        write!(f, [group(&content)])
    }
}
