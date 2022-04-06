use crate::formatter_traits::FormatTokenAndNode;
use crate::utils::format_with_semicolon;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsVariableStatement;
use rome_js_syntax::JsVariableStatementFields;

impl ToFormatElement for JsVariableStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(formatter, declaration.format(formatter)?, semicolon_token)
    }
}
