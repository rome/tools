use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsVariableDeclarationClause;
use rome_js_syntax::JsVariableDeclarationClauseFields;

impl FormatNodeFields<JsVariableDeclarationClause> for FormatNodeRule<JsVariableDeclarationClause> {
    fn fmt_fields(node: &JsVariableDeclarationClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        FormatWithSemicolon::new(&declaration.format(), semicolon_token.as_ref()).fmt(f)
    }
}
