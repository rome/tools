use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};
use rome_js_syntax::{JsDoWhileStatementFields, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsDoWhileStatement;

impl FormatNodeRule<JsDoWhileStatement> for FormatJsDoWhileStatement {
    fn fmt_fields(&self, node: &JsDoWhileStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDoWhileStatementFields {
            do_token,
            body,
            while_token,
            l_paren_token,
            test,
            r_paren_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [do_token.format()])?;

        match body? {
            JsAnyStatement::JsEmptyStatement(body) => {
                write!(f, [body.format(), hard_line_break()])?;
            }
            body => {
                write!(f, [space(), body.format()])?;
            }
        };

        write![
            f,
            [
                space(),
                while_token.format(),
                space(),
                format_delimited(&l_paren_token?, &test.format(), &r_paren_token?,)
                    .soft_block_indent(),
            ]
        ]?;

        match semicolon_token {
            Some(semicolon_token) => write!(f, [semicolon_token.format()]),
            None => format_inserted(JsSyntaxKind::SEMICOLON).fmt(f),
        }
    }
}
