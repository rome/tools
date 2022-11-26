use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;
use rome_formatter::write;

use rome_js_syntax::JsVariableDeclarationClause;
use rome_js_syntax::JsVariableDeclarationClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclarationClause;

impl FormatNodeRule<JsVariableDeclarationClause> for FormatJsVariableDeclarationClause {
    fn fmt_fields(
        &self,
        node: &JsVariableDeclarationClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                declaration.format(),
                FormatStatementSemicolon::new(semicolon_token.as_ref())
            ]
        )
    }
}
