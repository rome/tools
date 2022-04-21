use crate::utils::format_with_semicolon;
use crate::{Format, FormatElement, FormatNode, FormatResult, Formatter};

use rome_js_syntax::JsVariableDeclarationClause;
use rome_js_syntax::JsVariableDeclarationClauseFields;

impl FormatNode for JsVariableDeclarationClause {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(formatter, declaration.format(formatter)?, semicolon_token)
    }
}
