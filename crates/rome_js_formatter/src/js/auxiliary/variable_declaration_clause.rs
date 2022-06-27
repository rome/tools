use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsVariableDeclarationClause;
use rome_js_syntax::JsVariableDeclarationClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsVariableDeclarationClause;

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

        FormatWithSemicolon::new(&declaration.format(), semicolon_token.as_ref()).fmt(f)
    }
}
