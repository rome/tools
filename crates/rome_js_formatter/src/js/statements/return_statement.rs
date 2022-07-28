use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::write;
use rome_js_syntax::{JsAnyExpression, JsReturnStatement, JsReturnStatementFields};

#[derive(Debug, Clone, Default)]
pub struct FormatJsReturnStatement;

impl FormatNodeRule<JsReturnStatement> for FormatJsReturnStatement {
    fn fmt_fields(&self, node: &JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsReturnStatementFields {
            return_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_with(|f| {
                    write!(f, [return_token.format()])?;

                    if let Some(argument) = &argument {
                        write!(f, [space_token()])?;

                        if let JsAnyExpression::JsSequenceExpression(_expression) = argument {
                            format_parenthesize(
                                argument.syntax().first_token().as_ref(),
                                &argument.format(),
                                argument.syntax().last_token().as_ref(),
                            )
                            .grouped_with_soft_block_indent()
                            .fmt(f)?;
                        } else {
                            write![f, [argument.format()]]?;
                        }
                    }

                    Ok(())
                }),
                semicolon_token.as_ref()
            )]
        )
    }
}
