use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsVariableStatement;
use rome_js_syntax::JsVariableStatementFields;

impl FormatNodeFields<JsVariableStatement> for FormatNodeRule<JsVariableStatement> {
    fn fmt_fields(node: &JsVariableStatement, f: &mut JsFormatter) -> FormatResult<()> {
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
