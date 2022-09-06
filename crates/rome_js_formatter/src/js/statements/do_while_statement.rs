use crate::prelude::*;

use crate::builders::format_delimited;
use crate::utils::{FormatStatementBody, FormatWithSemicolon};
use rome_formatter::{format_args, write};
use rome_js_syntax::JsDoWhileStatementFields;
use rome_js_syntax::{JsAnyStatement, JsDoWhileStatement};

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

        let body = body?;
        let l_paren_token = l_paren_token?;
        let r_paren_token = r_paren_token?;

        let format_statement = format_with(|f| {
            write!(
                f,
                [group(&format_args![
                    do_token.format(),
                    FormatStatementBody::new(&body)
                ])]
            )?;

            if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
                write!(f, [space()])?;
            } else {
                write!(f, [hard_line_break()])?;
            }

            write!(
                f,
                [
                    while_token.format(),
                    space(),
                    format_delimited(&l_paren_token, &test.format(), &r_paren_token)
                        .soft_block_indent(),
                ]
            )
        });

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_statement,
                semicolon_token.as_ref()
            )]
        )
    }
}
