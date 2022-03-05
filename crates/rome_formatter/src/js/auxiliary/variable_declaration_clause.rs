use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsVariableDeclarationClause;
use rslint_syntax::JsVariableDeclarationClauseFields;

impl ToFormatElement for JsVariableDeclarationClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableDeclarationClauseFields {
            declaration,
            semicolon_token,
        } = self.as_fields();

        let declarations = declaration.format(formatter)?;
        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![declarations, semicolon])
    }
}
