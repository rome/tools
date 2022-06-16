use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::{JsAnyExpression, JsReturnStatement, JsReturnStatementFields};

impl FormatNodeFields<JsReturnStatement> for FormatNodeRule<JsReturnStatement> {
    fn fmt_fields(node: &JsReturnStatement, f: &mut JsFormatter) -> FormatResult<()> {
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
                            // SAFETY: a sequence expression contains at least the `,` comma token. Therefore, it's safe
                            // to call `unwrap` here
                            format_parenthesize(
                                &argument.syntax().first_token().unwrap(),
                                &argument.format(),
                                &argument.syntax().last_token().unwrap(),
                            )
                            .grouped()
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
