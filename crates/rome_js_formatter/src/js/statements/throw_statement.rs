use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use crate::js::statements::return_statement::FormatReturnOrThrowArgument;
use rome_js_syntax::JsThrowStatement;
use rome_js_syntax::JsThrowStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsThrowStatement;

impl FormatNodeRule<JsThrowStatement> for FormatJsThrowStatement {
    fn fmt_fields(&self, node: &JsThrowStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsThrowStatementFields {
            throw_token,
            argument,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args![
                    throw_token.format(),
                    space(),
                    FormatReturnOrThrowArgument::new(&argument?)
                ],
                semicolon_token.as_ref()
            )]
        )
    }
}
