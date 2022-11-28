use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatStatementSemicolon;

use rome_js_syntax::JsContinueStatement;
use rome_js_syntax::JsContinueStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsContinueStatement;

impl FormatNodeRule<JsContinueStatement> for FormatJsContinueStatement {
    fn fmt_fields(&self, node: &JsContinueStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsContinueStatementFields {
            continue_token,
            label_token,
            semicolon_token,
        } = node.as_fields();

        write!(f, [continue_token.format()])?;

        if let Some(label) = &label_token {
            write!(f, [space(), label.format()])?;
        }

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }
}
