use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsVariableDeclarationClause;
use rome_js_syntax::JsVariableDeclarationClauseFields;

impl FormatNodeFields<JsVariableDeclarationClause> for FormatNodeRule<JsVariableDeclarationClause> {
    fn format_fields(
        node: &JsVariableDeclarationClause,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![formatter, [declaration.format()]]?,
            semicolon_token,
        )
    }
}
