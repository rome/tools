use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_parser::ast::JsDebuggerStatement;
use rslint_parser::ast::JsDebuggerStatementFields;

impl ToFormatElement for JsDebuggerStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDebuggerStatementFields {
            debugger_token,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            debugger_token.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?
        ])
    }
}
