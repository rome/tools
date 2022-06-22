use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsVariableStatement;
use rome_js_syntax::JsVariableStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsVariableStatement;

impl FormatNodeRule<JsVariableStatement> for FormatJsVariableStatement {
    fn fmt_fields(&self, node: &JsVariableStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &declaration.format(),
                semicolon_token.as_ref()
            )]
        )
    }
}
