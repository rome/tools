use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

impl FormatNodeFields<JsContinueStatement> for FormatNodeRule<JsContinueStatement> {
    fn fmt_fields(node: &JsContinueStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_with(|f| {
                    write!(f, [continue_token.format()])?;

                    if let Some(label_token) = &label_token {
                        write!(f, [space_token(), label_token.format()])?;
                    }

                    Ok(())
                }),
                semicolon_token.as_ref()
            )]
        )
    }
}
