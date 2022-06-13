use crate::builders::format_inserted;
use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{JsAnyExpression, JsReturnStatement, JsReturnStatementFields, JsSyntaxKind};

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
                            write![
                                f,
                                [group_elements(&format_args![
                                    format_inserted(JsSyntaxKind::L_PAREN),
                                    soft_block_indent(&argument.format()),
                                    format_inserted(JsSyntaxKind::R_PAREN)
                                ])]
                            ]?;
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
