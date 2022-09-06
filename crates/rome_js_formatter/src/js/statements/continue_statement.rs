use crate::prelude::*;
use rome_formatter::{write, CstFormatContext};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsContinueStatement;

impl FormatNodeRule<JsContinueStatement> for FormatJsContinueStatement {
    fn fmt_fields(&self, node: &JsContinueStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_with(|f: &mut JsFormatter| {
                    write!(f, [continue_token.format()])?;

                    if let Some(label) = &label_token {
                        if f.context().comments().has_dangling_trivia(&label) {
                            write!(f, [space(), format_dangling_trivia(label)])?;
                        }

                        write!(f, [space(), label.format()])?;
                    }

                    Ok(())
                }),
                semicolon_token.as_ref()
            )]
        )
    }
}
