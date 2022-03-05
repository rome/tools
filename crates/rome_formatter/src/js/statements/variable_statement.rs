use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsVariableStatement;
use rslint_syntax::JsVariableStatementFields;

impl ToFormatElement for JsVariableStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            declaration.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
